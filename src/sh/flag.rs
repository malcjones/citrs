#[derive(Debug, Clone, PartialEq)]
pub enum Flag {
    String(String),
    Bool(bool),
    Int(i32),
}
