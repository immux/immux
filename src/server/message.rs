use crate::storage::executor::command::Command;

pub enum MessageSender {
    TCP,
    HTTP,
}

pub struct Message {
    pub command: Command,
    pub sender: MessageSender,
}

impl Message {
    pub fn new(command: Command, sender: MessageSender) -> Self {
        Message { command, sender }
    }
}
