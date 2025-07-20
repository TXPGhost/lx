use std::collections::HashMap;

pub enum Value {
    I32(i32),
}

pub trait Eval {
    fn eval(&self, ctxt: HashMap<String, Value>) -> Result<Option<Value>, &'static str>;
}
