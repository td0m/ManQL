use super::Value;

#[derive(Debug, Default, PartialEq)]
pub struct Set {
    pub vartype: Vec<SetType>,
    pub values: Vec<Value>
}


#[derive(Debug, PartialEq)]
pub enum SetType {
    Strict,
    NoCache,
}