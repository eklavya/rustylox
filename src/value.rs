#[derive(Debug, Copy, Clone)]
pub enum Value {
    Bool(bool),
    Number(f64),
    Empty,
}

// #[derive(Debug)]
// pub struct Values {
//     capacity: i32,
//     count: i32,
//     values: Vec<Value>,
// }

// impl Values {
//     pub fn new() -> Self {
//         Values {
//             capacity: 0,
//             count: 0,
//             values: vec![],
//         }
//     }

//     pub fn write(&mut self, value: Value) -> () {
//         self.values.push(value)
//     }

//     pub fn free(&mut self) -> () {
//         self.values = vec![];
//         self.count = 0;
//         self.capacity = 0;
//     }

//     pub fn len(&self) -> usize {
//         self.values.len()
//     }
// }

pub fn print_value(value: Value) -> () {
    match value {
        Value::Number(v) => print!("{}", v),
        Value::Bool(v) => print!("{}", v),
        Value::Empty => print!(""),
    }
}
