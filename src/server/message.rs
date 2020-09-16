use crate::storage::executor::command::Command;

pub enum AnnotatedCommandSender {
    TCP,
    HTTP,
}

pub struct AnnotatedCommand {
    pub command: Command,
    pub sender: AnnotatedCommandSender,
}

impl AnnotatedCommand {
    pub fn new(command: Command, sender: AnnotatedCommandSender) -> Self {
        AnnotatedCommand { command, sender }
    }
}
