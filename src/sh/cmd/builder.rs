use crate::sh::Shell;

use super::Command;

pub enum Arg {
    Optional(String),
    Required(String),
}

pub struct CommandBuilder {
    pub command: Command,
}

impl CommandBuilder {
    pub fn new(name: &str, description: &str) -> CommandBuilder {
        CommandBuilder {
            command: Command {
                name: name.to_string(),
                description: description.to_string(),
                usage: name.to_string(),
                action: |_, _| Ok(()),
            },
        }
    }

    pub fn action(mut self, action: fn(&mut Shell, Vec<String>) -> Result<(), String>) -> CommandBuilder {
        self.command.action = action;
        self
    }

    pub fn arg(mut self, arg: Arg) -> CommandBuilder {
        match arg {
            Arg::Optional(name) => {
                self.command.usage.push_str(&format!(" [{}]", name));
            }
            Arg::Required(name) => {
                self.command.usage.push_str(&format!(" <{}>", name));
            }
        }
        self
    }
    
}

