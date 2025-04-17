#[derive(Debug, Clone, PartialEq)]
pub enum MgValue {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    List(Vec<MgValue>),
    Map(std::collections::HashMap<String, MgValue>),
}