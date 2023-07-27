#[derive(Debug, PartialEq, Clone)]
pub enum BuckTypes {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Unknown(String),
}
