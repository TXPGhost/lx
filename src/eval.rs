use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    I32(i32),
    String(String),
    Char(u8),
}

pub trait Eval {
    fn eval(&self, ctxt: HashMap<String, Value>) -> Result<Option<Value>, &'static str>;
}
