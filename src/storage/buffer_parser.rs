use crate::storage::command::Command;

pub struct CommandBufferParser<'a> {
    buffer: &'a [u8],
    index: usize,
}

impl<'a> CommandBufferParser<'a> {
    pub fn new(buffer: &[u8], index: usize) -> CommandBufferParser {
        return CommandBufferParser { buffer, index };
    }
}

impl<'a> Iterator for CommandBufferParser<'a> {
    type Item = (Command, usize);

    fn next(&mut self) -> Option<Self::Item> {
        match Command::try_from(&self.buffer[self.index..]) {
            Ok((command, command_length)) => {
                self.index += command_length;
                return Some((command, command_length));
            }
            Err(_) => return None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use crate::storage::buffer_parser::CommandBufferParser;
    use crate::storage::chain_height::ChainHeight;
    use crate::storage::command::Command;
    use crate::storage::kvkey::KVKey;
    use crate::storage::kvvalue::KVValue;

    fn get_commands() -> Vec<Command> {
        let commands: Vec<Command> = vec![
            Command::Set {
                key: KVKey::new(&[0x00, 0x01]),
                value: KVValue::new(&[0xff, 0xf3]),
            },
            Command::RevertOne {
                key: KVKey::new(&[0x11, 0x22]),
                height: ChainHeight::new(3),
            },
            Command::RevertAll {
                height: ChainHeight::new(6),
            },
            Command::Remove {
                key: KVKey::new(&[0x88]),
            },
            Command::RemoveAll,
        ];

        return commands;
    }

    fn serialize_commands(commands: &Vec<Command>) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();
        &commands.to_vec().iter().fold(&mut buffer, |acc, command| {
            let command_bytes: Vec<u8> = command.clone().try_into().unwrap();
            acc.extend_from_slice(&command_bytes);
            return acc;
        });

        return buffer;
    }

    #[test]
    fn iterate_buffer_parser() {
        let commands = get_commands();
        let buffer = serialize_commands(&commands);

        let command_buffer_parser = CommandBufferParser {
            buffer: &buffer,
            index: 0,
        };

        for (index, (actual_command, _)) in command_buffer_parser.enumerate() {
            let expected_command = &commands.as_slice()[index];
            assert_eq!(&actual_command, expected_command);
        }
    }

    #[test]
    fn iterate_buffer_parser_with_broken_command() {
        let commands = get_commands();
        let mut buffer = serialize_commands(&commands);

        let some_broken_bytes = [0xff, 0x00, 0xfa];
        buffer.extend_from_slice(&some_broken_bytes);

        let mut command_buffer_parser = CommandBufferParser {
            buffer: &buffer,
            index: 0,
        };

        for expected_command in commands {
            let (actual_command, _) = &command_buffer_parser.next().unwrap();
            assert_eq!(actual_command, &expected_command);
        }

        let broken_piece = command_buffer_parser.next();
        assert_eq!(broken_piece, None);
    }
}
