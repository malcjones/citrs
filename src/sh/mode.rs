use super::cmd;

#[derive(Debug)]
pub struct Mode {
    pub name: String,
    pub description: String,
    pub commands: Vec<cmd::Command>,
}
