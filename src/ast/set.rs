pub struct Set {
    pub identifier: String,
    pub values: Vec<SetValue>,
}

pub enum SetValue {
    Text(String),
    Code(String),
    SubsetOf(String),
}
