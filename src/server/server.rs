use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;

use crate::constants as Constants;
use crate::server::errors::{ServerError, ServerResult};
use crate::server::message::{AnnotatedCommand, AnnotatedCommandSender};
use crate::storage::chain_height::ChainHeight;
use crate::storage::executor::command::{Command, SelectCondition};
use crate::storage::executor::executor::Executor;
use crate::storage::executor::filter::parse_filter_string;
use crate::storage::executor::grouping_label::GroupingLabel;
use crate::storage::executor::outcome::Outcome;
use crate::storage::executor::unit_content::UnitContent;
use crate::storage::executor::unit_key::UnitKey;
use crate::storage::transaction_manager::TransactionId;

use crate::storage::preferences::DBPreferences;
use tiny_http::{Method, Request, Response, Server};
use url::Url;

pub struct UrlInformation {
    pub queries: HashMap<String, String>,
    pub main_path: String,
}

impl UrlInformation {
    fn extract_numeric_query(&self, key: &str) -> ServerResult<u64> {
        match self.queries.get(key) {
            None => Err(ServerError::UrlParsingError),
            Some(string) => match string.parse::<u64>() {
                Err(_error) => Err(ServerError::UrlParsingError),
                Ok(value) => Ok(value),
            },
        }
    }
    fn extract_string_query(&self, key: &str) -> Option<String> {
        match self.queries.get(key) {
            None => None,
            Some(string) => Some(string.clone()),
        }
    }
}

fn spawn_db_thread(
    preferences: &DBPreferences,
    server_db_receiver: Receiver<AnnotatedCommand>,
    db_http_sender: Sender<Outcome>,
    db_tcp_sender: Sender<Outcome>,
) -> JoinHandle<ServerResult<()>> {
    let preferences = preferences.to_owned();
    thread::spawn(move || -> ServerResult<()> {
        let mut executor = Executor::open(&preferences)?;

        loop {
            let message = server_db_receiver.recv()?;
            let received_command = message.command;

            let outcome = handle_command(received_command, &mut executor)?;

            match message.sender {
                AnnotatedCommandSender::HTTP => {
                    db_http_sender.send(outcome)?;
                }
                AnnotatedCommandSender::TCP => {
                    db_tcp_sender.send(outcome)?;
                }
            }
        }
    })
}

pub fn run_db_servers(prefs: &DBPreferences) -> Vec<ServerResult<()>> {
    let (server_db_sender, server_db_receiver) = mpsc::channel::<AnnotatedCommand>();
    let (db_http_sender, db_http_receiver) = mpsc::channel::<Outcome>();
    let (db_tcp_sender, db_tcp_receiver) = mpsc::channel::<Outcome>();

    let mut handles = vec![];

    let db_handle = spawn_db_thread(prefs, server_db_receiver, db_http_sender, db_tcp_sender);
    handles.push(db_handle);

    if let Some(port) = prefs.tcp_port {
        let tcp_handle = run_tcp_server(port, server_db_sender.clone(), db_tcp_receiver);
        handles.push(tcp_handle);
    }

    if let Some(port) = prefs.http_port {
        let http_handle = run_http_server(port, server_db_sender, db_http_receiver);
        handles.push(http_handle);
    }

    let mut server_results = vec![];

    for handle in handles {
        match handle.join() {
            Ok(thread_result) => server_results.push(thread_result),
            Err(_error) => server_results.push(Err(ServerError::ThreadError)),
        }
    }
    return server_results;
}

pub fn run_http_server(
    http_port: u16,
    server_db_sender: Sender<AnnotatedCommand>,
    db_http_receiver: Receiver<Outcome>,
) -> JoinHandle<ServerResult<()>> {
    let http_address = format!("{}:{}", Constants::SERVER_END_POINT, http_port);

    thread::spawn(move || -> ServerResult<()> {
        match Server::http(http_address) {
            Ok(server) => {
                for mut request in server.incoming_requests() {
                    let command = parse_http_request(&mut request)?;
                    let message = AnnotatedCommand::new(command, AnnotatedCommandSender::HTTP);
                    server_db_sender.send(message)?;

                    let outcome = db_http_receiver.recv()?;

                    let (status, body): (u16, String) = match outcome {
                        Outcome::Select(outcome) => {
                            let outcome_string_vec: Vec<String> = outcome
                                .iter()
                                .map(|unit_content| unit_content.to_string())
                                .collect();
                            let body = outcome_string_vec.join("\r\n");
                            (200, body)
                        }
                        Outcome::InspectOne(outcome) => {
                            let mut body = String::new();
                            for (command, height) in outcome {
                                body += &command.to_string();
                                body += "\t";
                                body += &format!("height: {:?}", height);
                                body += "\r\n";
                            }
                            (200, body)
                        }
                        Outcome::InspectAll(outcome) => {
                            let mut body = String::new();
                            for (command, height) in outcome {
                                body += &command.to_string();
                                body += "\t";
                                body += &format!("height: {:?}", height);
                                body += "\r\n";
                            }
                            (200, body)
                        }
                        Outcome::CreateTransaction(transaction_id) => {
                            let body = transaction_id.as_u64().to_string();
                            (200, body)
                        }
                        Outcome::GetAllGroupingsSuccess(groupings) => {
                            let outcome_string_vec: Vec<String> = groupings
                                .iter()
                                .map(|grouping| grouping.to_string())
                                .collect();
                            let body = outcome_string_vec.join("\r\n");
                            (200, body)
                        }
                        _ => (200, String::from("Unspecified outcome")),
                    };

                    let response = if body.is_empty() {
                        Response::from_string(UnitContent::Nil.to_string()).with_status_code(status)
                    } else {
                        Response::from_string(body).with_status_code(status)
                    };

                    match request.respond(response) {
                        Ok(_) => {}
                        Err(error) => return Err(ServerError::HttpResponseError(error)),
                    }
                }
            }
            Err(_error) => return Err(ServerError::TinyHTTPError),
        }
        return Ok(());
    })
}

fn run_tcp_server(
    tcp_port: u16,
    server_db_sender: Sender<AnnotatedCommand>,
    db_tcp_receiver: Receiver<Outcome>,
) -> JoinHandle<ServerResult<()>> {
    let tcp_address = format!("{}:{}", Constants::SERVER_END_POINT, tcp_port);
    let listener = TcpListener::bind(tcp_address).unwrap();

    thread::spawn(move || -> ServerResult<()> {
        for stream in listener.incoming() {
            let mut stream = stream?;

            loop {
                let mut buffer = [0; 1024 * 1024];
                stream.read(&mut buffer)?;
                let (command, _) = Command::parse(&buffer)?;
                let message = AnnotatedCommand::new(command, AnnotatedCommandSender::TCP);
                server_db_sender.send(message)?;

                let outcome = db_tcp_receiver.recv()?;
                let outcome_bytes = outcome.marshal();

                stream.write(&outcome_bytes)?;
                stream.flush().unwrap();
            }
        }
        return Ok(());
    })
}

fn handle_command(command: Command, executor: &mut Executor) -> ServerResult<Outcome> {
    match command {
        Command::Select { condition } => {
            let outcome = executor.get(&condition)?;
            return Ok(outcome);
        }
        Command::Insert {
            grouping,
            key,
            content,
        } => {
            let outcome = executor.set(&grouping, &key, &content, None)?;
            return Ok(outcome);
        }
        Command::RemoveOne { grouping, key } => {
            let outcome = executor.remove_one(&grouping, &key, None)?;
            return Ok(outcome);
        }
        Command::RemoveAll => {
            let outcome = executor.remove_all()?;
            return Ok(outcome);
        }
        Command::RevertOne {
            grouping,
            key,
            height,
        } => {
            let outcome = executor.revert_one(&grouping, &key, &height, None)?;
            return Ok(outcome);
        }
        Command::RevertAll { height } => {
            let outcome = executor.revert_all(&height)?;
            return Ok(outcome);
        }
        Command::InspectOne { grouping, key } => {
            let outcome = executor.inspect_one(&grouping, &key)?;
            return Ok(outcome);
        }
        Command::InspectAll => {
            let outcome = executor.inspect_all()?;
            return Ok(outcome);
        }
        Command::CreateTransaction => {
            let outcome = executor.start_transaction()?;
            return Ok(outcome);
        }
        Command::TransactionCommit { transaction_id } => {
            let outcome = executor.commit_transaction(transaction_id)?;
            return Ok(outcome);
        }
        Command::TransactionAbort { transaction_id } => {
            let outcome = executor.abort_transaction(transaction_id)?;
            return Ok(outcome);
        }
        Command::TransactionalInsert {
            grouping,
            key,
            content,
            transaction_id,
        } => {
            let outcome = executor.set(&grouping, &key, &content, Some(transaction_id))?;
            return Ok(outcome);
        }
        Command::TransactionalRemoveOne {
            grouping,
            key,
            transaction_id,
        } => {
            let outcome = executor.remove_one(&grouping, &key, Some(transaction_id))?;
            return Ok(outcome);
        }
        Command::TransactionalRevertOne {
            grouping,
            key,
            height,
            transaction_id,
        } => {
            let outcome = executor.revert_one(&grouping, &key, &height, Some(transaction_id))?;
            return Ok(outcome);
        }
    }
}

fn parse_http_request(request: &mut Request) -> ServerResult<Command> {
    let mut incoming_body = String::new();
    match request.as_reader().read_to_string(&mut incoming_body) {
        Ok(_) => (),
        Err(error) => return Err(ServerError::BodyExtractionError(error)),
    }

    let url_info = parse_path(&request.url())?;
    let segments: Vec<&str> = url_info.main_path.split("/").collect();

    match request.method() {
        Method::Get => {
            if segments.len() >= 5 {
                let url_transactions_key_word = segments[1];
                let transaction_id_str = segments[2];
                let grouping_str = segments[3];
                let unit_key_str = segments[4];

                if url_transactions_key_word != Constants::URL_TRANSACTIONS_KEY_WORD
                    || unit_key_str.is_empty()
                {
                    return Err(ServerError::UrlParsingError);
                }

                let transaction_id = transaction_id_str.parse::<u64>()?;
                let transaction_id = TransactionId::new(transaction_id);
                let grouping = GroupingLabel::new(grouping_str.as_bytes());
                let unit_key = UnitKey::from(unit_key_str);

                let condition = SelectCondition::Key(grouping, unit_key, Some(transaction_id));
                let command = Command::Select { condition };

                return Ok(command);
            } else if segments.len() >= 4 {
                let grouping_str = segments[1];
                let unit_key_str = segments[2];
                let url_journal_key_word = segments[3];

                if url_journal_key_word != Constants::URL_JOURNAL_KEY_WORD
                    || unit_key_str.is_empty()
                {
                    return Err(ServerError::UrlParsingError);
                }

                let grouping = GroupingLabel::new(grouping_str.as_bytes());
                let unit_key = UnitKey::from(unit_key_str);
                let command = Command::InspectOne {
                    grouping,
                    key: unit_key,
                };
                return Ok(command);
            } else if segments.len() >= 3 {
                let grouping_str = segments[1];
                let unit_key_str = segments[2];
                let grouping = GroupingLabel::new(grouping_str.as_bytes());
                let unit_key = UnitKey::from(unit_key_str);

                let condition = SelectCondition::Key(grouping, unit_key, None);
                let command = Command::Select { condition };

                return Ok(command);
            } else if segments.len() >= 2 {
                if segments[1] == Constants::URL_JOURNAL_KEY_WORD {
                    let command = Command::InspectAll;
                    return Ok(command);
                } else if segments[1] == Constants::URL_GROUPING_KEY_WORD {
                    let condition = SelectCondition::AllGrouping;
                    let command = Command::Select { condition };
                    return Ok(command);
                } else {
                    if let Some(filter_string) =
                        url_info.extract_string_query(Constants::FILTER_KEY_WORD)
                    {
                        let grouping_str = segments[1];
                        let grouping = GroupingLabel::new(grouping_str.as_bytes());
                        let filter = parse_filter_string(filter_string)?;
                        let condition = SelectCondition::Filter(grouping, filter);

                        let command = Command::Select { condition };

                        return Ok(command);
                    } else {
                        let grouping_str = segments[1];
                        let grouping = GroupingLabel::new(grouping_str.as_bytes());
                        let condition = SelectCondition::UnconditionalMatch(grouping);

                        let command = Command::Select { condition };

                        return Ok(command);
                    }
                }
            } else {
                return Err(ServerError::UrlParsingError);
            }
        }
        Method::Put => {
            if segments.len() >= 5 {
                let url_transactions_key_word = segments[1];
                let transaction_id_str = segments[2];
                let grouping_str = segments[3];
                let unit_key_str = segments[4];

                if url_transactions_key_word != Constants::URL_TRANSACTIONS_KEY_WORD
                    || unit_key_str.is_empty()
                {
                    return Err(ServerError::UrlParsingError);
                }

                let grouping = GroupingLabel::new(grouping_str.as_bytes());
                let unit_key = UnitKey::from(unit_key_str);
                let transaction_id = transaction_id_str.parse::<u64>()?;

                if let Ok(height) = url_info.extract_numeric_query(Constants::HEIGHT) {
                    let height = ChainHeight::new(height);
                    let transaction_id = TransactionId::new(transaction_id);
                    let command = Command::TransactionalRevertOne {
                        grouping,
                        key: unit_key,
                        height,
                        transaction_id,
                    };
                    return Ok(command);
                } else {
                    let content = UnitContent::from(incoming_body.as_str());
                    let transaction_id = TransactionId::new(transaction_id);
                    let command = Command::TransactionalInsert {
                        grouping,
                        key: unit_key,
                        content,
                        transaction_id,
                    };
                    return Ok(command);
                }
            } else if segments.len() >= 3 {
                let grouping_str = segments[1];
                let unit_key_str = segments[2];

                if unit_key_str.is_empty() {
                    return Err(ServerError::UrlParsingError);
                }

                let grouping = GroupingLabel::new(grouping_str.as_bytes());
                let unit_key = UnitKey::from(unit_key_str);

                if let Ok(height) = url_info.extract_numeric_query(Constants::HEIGHT) {
                    let height = ChainHeight::new(height);
                    let command = Command::RevertOne {
                        grouping,
                        key: unit_key,
                        height,
                    };
                    return Ok(command);
                } else {
                    let content = UnitContent::from(incoming_body.as_str());
                    let command = Command::Insert {
                        grouping,
                        key: unit_key,
                        content,
                    };
                    return Ok(command);
                }
            } else if let Ok(height) = url_info.extract_numeric_query(Constants::HEIGHT) {
                let height = ChainHeight::new(height);
                let command = Command::RevertAll { height };
                return Ok(command);
            } else {
                return Err(ServerError::UrlParsingError);
            }
        }
        Method::Post => {
            let (url_transactions_key_word, transaction_id_str) = if segments.len() >= 3 {
                (segments[1], segments[2])
            } else if segments.len() == 2 {
                (segments[1], "")
            } else {
                ("", "")
            };

            if url_transactions_key_word != Constants::URL_TRANSACTIONS_KEY_WORD {
                return Err(ServerError::UrlParsingError);
            }

            if let Some(_) = url_info.extract_string_query(Constants::COMMIT_TRANSACTION_KEY_WORD) {
                if transaction_id_str.is_empty() {
                    return Err(ServerError::UrlParsingError);
                }

                let transaction_id = transaction_id_str.parse::<u64>()?;
                let transaction_id = TransactionId::new(transaction_id);

                let command = Command::TransactionCommit { transaction_id };
                return Ok(command);
            } else if let Some(_) =
                url_info.extract_string_query(Constants::ABORT_TRANSACTION_KEY_WORD)
            {
                if transaction_id_str.is_empty() {
                    return Err(ServerError::UrlParsingError);
                }

                let transaction_id = transaction_id_str.parse::<u64>()?;
                let transaction_id = TransactionId::new(transaction_id);

                let command = Command::TransactionAbort { transaction_id };
                return Ok(command);
            } else {
                let command = Command::CreateTransaction;
                return Ok(command);
            }
        }
        Method::Delete => {
            if segments.len() >= 5 {
                let url_transactions_key_word = segments[1];
                let transaction_id_str = segments[2];
                let grouping_str = segments[3];
                let unit_key_str = segments[4];

                if unit_key_str.is_empty()
                    || transaction_id_str.is_empty()
                    || url_transactions_key_word != Constants::URL_TRANSACTIONS_KEY_WORD
                {
                    return Err(ServerError::UrlParsingError);
                }

                let transaction_id = transaction_id_str.parse::<u64>()?;
                let transaction_id = TransactionId::new(transaction_id);
                let grouping = GroupingLabel::new(grouping_str.as_bytes());
                let unit_key = UnitKey::from(unit_key_str);

                let command = Command::TransactionalRemoveOne {
                    grouping,
                    key: unit_key,
                    transaction_id,
                };
                return Ok(command);
            } else if segments.len() >= 3 {
                let grouping_str = segments[1];
                let unit_key_str = segments[2];
                let grouping = GroupingLabel::new(grouping_str.as_bytes());
                let unit_key = UnitKey::from(unit_key_str);

                let command = Command::RemoveOne {
                    grouping,
                    key: unit_key,
                };
                return Ok(command);
            } else {
                let command = Command::RemoveAll;
                return Ok(command);
            }
        }
        _ => return Err(ServerError::BodyParsingError),
    }
}

pub fn parse_path(path: &str) -> ServerResult<UrlInformation> {
    let path_to_parse = format!("{}{}", "http://127.0.0.1", path);
    match Url::parse(&path_to_parse) {
        Err(_error) => Err(ServerError::UrlParsingError),
        Ok(parse) => {
            let url_queries: HashMap<_, _> = parse.query_pairs().into_owned().collect();
            Ok(UrlInformation {
                queries: url_queries,
                main_path: String::from(parse.path()),
            })
        }
    }
}
