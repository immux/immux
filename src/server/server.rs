use std::borrow::BorrowMut;
use std::collections::HashMap;

use crate::constants as Constants;
use crate::server::errors::{ServerError, ServerResult};
use crate::storage::chain_height::ChainHeight;
use crate::storage::executor::executor::Executor;
use crate::storage::executor::instruction::Instruction;
use crate::storage::executor::outcome::Outcome;
use crate::storage::executor::unit_content::UnitContent;
use crate::storage::executor::unit_key::UnitKey;
use crate::storage::transaction_manager::TransactionId;

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

pub fn run_server(mut executor: Executor, port: u16) -> ServerResult<()> {
    let address = format!("{}:{}", Constants::SERVER_END_POINT, port);
    match Server::http(address) {
        Ok(server) => {
            for mut request in server.incoming_requests() {
                let (status, body): (u16, String) = match handle_request(request.borrow_mut(), &mut executor) {
                    Err(error) => (500, format!("Server error {:?}", error)),
                    Ok(outcome) => match outcome {
                        Outcome::Select(outcome) => {
                            let body = match outcome {
                                None => String::from(""),
                                Some(content) => content.to_string(),
                            };
                            (200, body)
                        }
                        Outcome::InspectOne(outcome) => {
                            let mut body = String::new();
                            for (instruction, height) in outcome {
                                body += &instruction.to_string();
                                body += "\t";
                                body += &format!("height: {:?}", height);
                                body += "\r\n";
                            }
                            (200, body)
                        }
                        Outcome::InspectAll(outcome) => {
                            let mut body = String::new();
                            for (instruction, height) in outcome {
                                body += &instruction.to_string();
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
                        _ => (200, String::from("Unspecified outcome")),
                    },
                };

                let response = Response::from_string(body).with_status_code(status);
                match request.respond(response) {
                    Ok(_) => {}
                    Err(error) => return Err(ServerError::HttpResponseError(error)),
                }
            }
        }
        Err(_error) => return Err(ServerError::TinyHTTPError),
    }
    return Ok(());
}

fn handle_request(request: &mut Request, executor: &mut Executor) -> ServerResult<Outcome> {
    let instruction = parse_http_request(request)?;

    match instruction {
        Instruction::Select { key, transaction_id } => {
            let result = executor.get(&key, transaction_id)?;
            return Ok(Outcome::Select(result));
        }
        Instruction::Insert { key, content } => {
            executor.set(&key, &content, None)?;
            return Ok(Outcome::InsertSuccess);
        }
        Instruction::RemoveOne { key } => {
            executor.remove_one(&key, None)?;
            return Ok(Outcome::RemoveOneSuccess);
        }
        Instruction::RemoveAll => {
            executor.remove_all()?;
            return Ok(Outcome::RemoveAllSuccess);
        }
        Instruction::RevertOne { key, height } => {
            executor.revert_one(&key, &height, None)?;
            return Ok(Outcome::RevertOneSuccess);
        }
        Instruction::RevertAll { height } => {
            executor.revert_all(&height)?;
            return Ok(Outcome::RevertAllSuccess);
        }
        Instruction::InspectOne { key } => {
            let result = executor.inspect_one(&key)?;
            return Ok(Outcome::InspectOne(result));
        }
        Instruction::InspectAll => {
            let result = executor.inspect_all()?;
            return Ok(Outcome::InspectAll(result));
        }
        Instruction::CreateTransaction => {
            let transaction_id = executor.start_transaction()?;
            return Ok(Outcome::CreateTransaction(transaction_id));
        }
        Instruction::TransactionCommit { transaction_id } => {
            executor.commit_transaction(transaction_id)?;
            return Ok(Outcome::TransactionCommitSuccess);
        }
        Instruction::TransactionAbort { transaction_id } => {
            executor.abort_transaction(transaction_id)?;
            return Ok(Outcome::TransactionAbortSuccess);
        }
        Instruction::TransactionalInsert { key, content, transaction_id } => {
            executor.set(&key, &content, Some(transaction_id))?;
            return Ok(Outcome::TransactionalInsertSuccess);
        }
        Instruction::TransactionalRemoveOne { key, transaction_id } => {
            executor.remove_one(&key, Some(transaction_id))?;
            return Ok(Outcome::TransactionalRemoveOneSuccess);
        }
        Instruction::TransactionalRevertOne { key, height, transaction_id } => {
            executor.revert_one(&key, &height, Some(transaction_id))?;
            return Ok(Outcome::TransactionalRevertOneSuccess);
        }
    }
}

fn parse_http_request(request: &mut Request) -> ServerResult<Instruction> {
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
                let _grouping_str = segments[3];
                let unit_key_str = segments[4];

                if url_transactions_key_word != Constants::URL_TRANSACTIONS_KEY_WORD || unit_key_str.is_empty() {
                    return Err(ServerError::UrlParsingError);
                }

                let transaction_id = transaction_id_str.parse::<u64>()?;
                let transaction_id = TransactionId::new(transaction_id);
                let unit_key = UnitKey::from(unit_key_str);

                let instruction = Instruction::Select {
                    key: unit_key,
                    transaction_id: Some(transaction_id),
                };
                return Ok(instruction);
            } else if segments.len() >= 4 {
                let _grouping_str = segments[1];
                let unit_key_str = segments[2];
                let url_journal_key_word = segments[3];

                if url_journal_key_word != Constants::URL_JOURNAL_KEY_WORD || unit_key_str.is_empty() {
                    return Err(ServerError::UrlParsingError);
                }

                let unit_key = UnitKey::from(unit_key_str);
                let instruction = Instruction::InspectOne { key: unit_key };
                return Ok(instruction);
            } else if segments.len() >= 3 {
                let _grouping_str = segments[1];
                let unit_key_str = segments[2];
                let unit_key = UnitKey::from(unit_key_str);

                let instruction = Instruction::Select {
                    key: unit_key,
                    transaction_id: None,
                };
                return Ok(instruction);
            } else if segments.len() >= 2 {
                if segments[1] == Constants::URL_JOURNAL_KEY_WORD {
                    let instruction = Instruction::InspectAll;
                    return Ok(instruction);
                } else {
                    return Err(ServerError::UnimplementedForGetGrouping);
                }
            } else {
                return Err(ServerError::UrlParsingError);
            }
        }
        Method::Put => {
            if segments.len() >= 5 {
                let url_transactions_key_word = segments[1];
                let transaction_id_str = segments[2];
                let _grouping_str = segments[3];
                let unit_key_str = segments[4];

                if url_transactions_key_word != Constants::URL_TRANSACTIONS_KEY_WORD || unit_key_str.is_empty() {
                    return Err(ServerError::UrlParsingError);
                }

                let unit_key = UnitKey::from(unit_key_str);
                let transaction_id = transaction_id_str.parse::<u64>()?;

                if let Ok(height) = url_info.extract_numeric_query(Constants::HEIGHT) {
                    let height = ChainHeight::new(height);
                    let transaction_id = TransactionId::new(transaction_id);
                    let instruction = Instruction::TransactionalRevertOne {
                        key: unit_key,
                        height,
                        transaction_id,
                    };
                    return Ok(instruction);
                } else {
                    let content = UnitContent::String(incoming_body);
                    let transaction_id = TransactionId::new(transaction_id);
                    let instruction = Instruction::TransactionalInsert { key: unit_key, content, transaction_id };
                    return Ok(instruction);
                }
            } else if segments.len() >= 3 {
                let _grouping_str = segments[1];
                let unit_key_str = segments[2];

                if unit_key_str.is_empty() {
                    return Err(ServerError::UrlParsingError);
                }

                let unit_key = UnitKey::from(unit_key_str);

                if let Ok(height) = url_info.extract_numeric_query(Constants::HEIGHT) {
                    let height = ChainHeight::new(height);
                    let instruction = Instruction::RevertOne { key: unit_key, height };
                    return Ok(instruction);
                } else {
                    let content = UnitContent::String(incoming_body);
                    let instruction = Instruction::Insert { key: unit_key, content };
                    return Ok(instruction);
                }
            } else if let Ok(height) = url_info.extract_numeric_query(Constants::HEIGHT) {
                let height = ChainHeight::new(height);
                let instruction = Instruction::RevertAll { height };
                return Ok(instruction);
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

                let instruction = Instruction::TransactionCommit { transaction_id };
                return Ok(instruction);
            } else if let Some(_) = url_info.extract_string_query(Constants::ABORT_TRANSACTION_KEY_WORD) {
                if transaction_id_str.is_empty() {
                    return Err(ServerError::UrlParsingError);
                }

                let transaction_id = transaction_id_str.parse::<u64>()?;
                let transaction_id = TransactionId::new(transaction_id);

                let instruction = Instruction::TransactionAbort { transaction_id };
                return Ok(instruction);
            } else {
                let instruction = Instruction::CreateTransaction;
                return Ok(instruction);
            }
        }
        Method::Delete => {
            if segments.len() >= 5 {
                let url_transactions_key_word = segments[1];
                let transaction_id_str = segments[2];
                let _grouping_str = segments[3];
                let unit_key_str = segments[4];

                if unit_key_str.is_empty() || transaction_id_str.is_empty() || url_transactions_key_word != Constants::URL_TRANSACTIONS_KEY_WORD {
                    return Err(ServerError::UrlParsingError);
                }

                let transaction_id = transaction_id_str.parse::<u64>()?;
                let transaction_id = TransactionId::new(transaction_id);
                let unit_key = UnitKey::from(unit_key_str);

                let instruction = Instruction::TransactionalRemoveOne { key: unit_key, transaction_id };
                return Ok(instruction);
            } else if segments.len() >= 3 {
                let _grouping_str = segments[1];
                let unit_key_str = segments[2];

                let unit_key = UnitKey::from(unit_key_str);

                let instruction = Instruction::RemoveOne { key: unit_key };
                return Ok(instruction);
            } else {
                let instruction = Instruction::RemoveAll;
                return Ok(instruction);
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
