#[cfg(test)]
mod tcp_e2e_tests {
    use std::io::{Read, Write};
    use std::net::TcpStream;
    use std::thread;

    use immuxsys::constants as Constants;
    use immuxsys::storage::executor::command::Command;
    use immuxsys::storage::executor::grouping_label::GroupingLabel;
    use immuxsys::storage::executor::outcome::Outcome;
    use immuxsys::storage::executor::unit_content::UnitContent;
    use immuxsys::storage::executor::unit_key::UnitKey;
    use immuxsys_client::tcp_client::ImmuxDBTCPClient;
    use immuxsys_dev_utils::dev_utils::{launch_db_server, notified_sleep};

    #[test]
    fn tcp_e2e_grouping_get_set() {
        let port = Constants::TCP_SERVER_DEFAULT_PORT;
        thread::spawn(move || launch_db_server("tcp_e2e_grouping_get_set", None, Some(port)));
        notified_sleep(5);

        let host = &format!("{}:{}", Constants::SERVER_END_POINT, port);
        let grouping = GroupingLabel::new("a".as_bytes());
        let unit_key = UnitKey::new("key".as_bytes());
        let unit_content = UnitContent::String("content".to_string());

        let mut stream = TcpStream::connect(host).unwrap();
        let command = Command::Insert {
            grouping: grouping.clone(),
            key: unit_key.clone(),
            content: unit_content.clone(),
        };

        let command_bytes = command.marshal();

        stream.write_all(&command_bytes).unwrap();
        stream.flush().unwrap();
        println!("Sent command, awaiting reply...");

        let mut data = vec![];
        match stream.read_to_end(&mut data) {
            Ok(_) => {
                let outcome = Outcome::parse(&data);
                println!("Reply is ok! {:?}", &outcome);
            }
            Err(e) => {
                println!("Failed to receive data: {}", e);
            }
        }
    }
}
