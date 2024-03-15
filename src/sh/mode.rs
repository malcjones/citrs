use super::cmd;

#[derive(Debug)]
pub struct Mode {
    pub name: String,
    pub description: String,
    pub commands: Vec<cmd::Command>
}

pub fn default() -> Vec<Mode> {
    vec![bookmark::mode()]
}

pub mod bookmark {
    use crate::sh::cmd::builder::CommandBuilder;

    use super::{cmd, Mode};

    pub fn mode() -> Mode {
        Mode {
            name: "bookmark".to_string(),
            description: "Bookmark mode".to_string(),
            commands: vec![add(), list()],
        }
    }

    fn add() -> cmd::Command {
        CommandBuilder::not_implemented("add").command
    }

    fn list() -> cmd::Command {
        CommandBuilder::not_implemented("list").command
    }
}
