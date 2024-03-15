use crate::sh::Shell;

use super::Command;

pub enum Arg {
    Optional(String),
    Required(String),
}

pub struct CommandBuilder(pub Command);

impl CommandBuilder {
    pub fn new(name: &str, description: &str) -> CommandBuilder {
        CommandBuilder(Command {
            name: name.to_string(),
            aliases: vec![name.to_string()],
            description: description.to_string(),
            usage: name.to_string(),
            action: |_, _| Ok(()),
        })
    }

    pub fn action(
        mut self,
        action: fn(&mut Shell, Vec<String>) -> Result<(), String>,
    ) -> CommandBuilder {
        self.0.action = action;
        self
    }

    pub fn arg(mut self, arg: Arg) -> CommandBuilder {
        match arg {
            Arg::Optional(name) => {
                self.0.usage.push_str(&format!(" [{}]", name));
            }
            Arg::Required(name) => {
                self.0.usage.push_str(&format!(" <{}>", name));
            }
        }
        self
    }

    pub fn todo(name: &str) -> CommandBuilder {
        CommandBuilder::new(name, "not yet implemented")
            .action(|_, _| Err("not yet implemented".to_string()))
    }

    pub fn alias(mut self, alias: &str) -> CommandBuilder {
        self.0.aliases.push(alias.to_string());
        self
    }

    pub fn aliases(mut self, aliases: Vec<&str>) -> CommandBuilder {
        self.0.aliases.extend(aliases.iter().map(|s| s.to_string()));
        self
    }

    pub fn build(self) -> Command {
        self.0
    }
}
