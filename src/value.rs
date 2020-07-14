use crate::obj::{print_obj, Object};
#[derive(Debug, Copy, Clone)]
pub enum Value {
    Bool(bool),
    Number(f64),
    Obj(*mut dyn Object),
    Empty,
}

pub fn print_value(value: Value) -> () {
    match value {
        Value::Number(v) => print!("{}", v),
        Value::Bool(v) => print!("{}", v),
        Value::Empty => print!(""),
        Value::Obj(o) => print_obj(o),
    }
}
