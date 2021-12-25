#[derive(Debug, PartialEq)]
pub enum Value {
    Text(String),
    Code(String),
    Reference(String),
}