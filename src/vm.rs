use crate::chunk::Chunk;
use crate::common::OpCode;
use crate::compiler::Compiler;
use crate::gc::GC;
use crate::obj::{self, ObjString, ObjType, Object};
use crate::value::{print_value, Value};
use crate::vm::InterpretResult::{InterpretCompileError, InterpretOk, InterpretRuntimeError};
use std::alloc::Layout;

pub struct VM {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Value>,
    top_stack: usize,
}

pub enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError,
}

macro_rules! binary_op {
    ($self:ident, $variant:ident, $op: tt) => {
    {
        let b = $self.pop();
        let a = $self.pop();
        match (a, b) {
            (Value::Number(av), Value::Number(bv)) => {
                $self.push(Value::$variant(av $op bv));
            }
            _ => {
                eprintln!("Operands must be numbers.");
                $self.runtime_error();
                return InterpretRuntimeError
            }
        }
    }
    }
}

impl VM {
    pub fn new() -> Self {
        VM {
            chunk: Chunk::new(),
            ip: 0,
            stack: vec![Value::Empty; 256],
            top_stack: 0,
        }
    }

    pub fn init(&mut self) -> () {
        self.top_stack = 0;
    }

    pub fn reset_stack(&mut self) -> () {
        self.top_stack = 0;
    }

    pub fn reset(&mut self) -> () {
        self.chunk = Chunk::new();
        self.ip = 0;
    }

    pub fn push(&mut self, value: Value) -> () {
        self.stack[self.top_stack] = value;
        self.top_stack += 1;
    }

    pub fn pop(&mut self) -> Value {
        self.top_stack -= 1;
        self.stack[self.top_stack]
    }

    pub fn peek(&self) -> Value {
        self.stack[self.top_stack - 1]
    }

    pub fn peek_next(&self) -> Value {
        self.stack[self.top_stack - 2]
    }

    pub fn free(&mut self) -> () {
        self.chunk = Chunk::new();
        self.ip = 0;
        self.stack = vec![];
    }

    pub fn interpret(&mut self, source: &str) -> InterpretResult {
        let mut compiler = Compiler::new(source, &mut self.chunk);
        if !compiler.compile(source) {
            eprintln!("error");
            self.runtime_error();
            InterpretCompileError
        } else {
            let res = self.run();
            self.reset();
            res
        }
    }

    // pub fn interpret(&mut self, chunk: Chunk) -> InterpretResult {
    //     self.chunk = chunk;
    //     if self.chunk.code.len() > 0 {
    //         self.ip = 0;
    //     }
    //     self.run()
    // }

    #[inline(always)]
    fn get_code(&mut self) -> u8 {
        let op = self.chunk.code[self.ip];
        self.ip += 1;
        op
    }

    #[inline(always)]
    fn read_constant(&mut self) -> Value {
        let ind = self.get_code();
        self.chunk.get_constant(ind)
    }

    #[cfg(debug_assertions)]
    fn show_stack(&self) {
        print!("                  ");
        self.stack.iter().for_each(|x| {
            print!("[ ");
            print_value(*x);
            print!(" ]");
        });
        println!();
    }

    pub fn runtime_error(&mut self) {
        let instruction = if self.ip > 0 { self.ip - 1 } else { self.ip };
        let line = self.chunk.get_line(instruction);
        eprintln!("[line {}] in script", line);
        self.reset_stack();
    }

    #[cfg(not(debug_assertions))]
    fn showStack(&self) {}

    fn run(&mut self) -> InterpretResult {
        loop {
            // self.show_stack();
            // self.chunk.disassemble_instruction(self.ip);
            let i = unsafe { OpCode::from_unchecked(self.get_code()) };
            match i {
                OpCode::OpReturn => {
                    print_value(self.pop());
                    println!();
                    return InterpretOk;
                }
                OpCode::OpConstant => {
                    let constant = self.read_constant();
                    self.push(constant);
                }
                OpCode::OpNegate => {
                    if let Value::Number(v) = self.peek() {
                        self.pop();
                        self.push(Value::Number(-v));
                    } else {
                        return InterpretRuntimeError;
                    }
                }
                OpCode::OpAdd => {
                    let a = self.peek();
                    let b = self.peek_next();
                    match (a, b) {
                        (Value::Number(_), Value::Number(_)) => binary_op!(self, Number, +),
                        (Value::Obj(o1), Value::Obj(o2)) => unsafe {
                            match ((*o1).get_type(), (*o2).get_type()) {
                                (ObjType::OString, ObjType::OString) => {
                                    let s = VM::concatenate(o2, o1);
                                    self.pop();
                                    self.pop();
                                    self.push(Value::Obj(s));
                                }
                            }
                        },
                        _ => {
                            eprintln!("Operands must be numbers or strings.");
                            self.runtime_error();
                            return InterpretRuntimeError;
                        }
                    }
                }
                OpCode::OpSubtract => binary_op!(self, Number, -),
                OpCode::OpMultiply => binary_op!(self, Number, *),
                OpCode::OpDivide => binary_op!(self, Number, /),
                OpCode::OpNil => self.push(Value::Empty),
                OpCode::OpTrue => self.push(Value::Bool(true)),
                OpCode::OpFalse => self.push(Value::Bool(false)),
                OpCode::OpNot => {
                    let v = self.pop();
                    self.push(Value::Bool(VM::is_falsey(v)))
                }
                OpCode::OpEqual => {
                    let a = self.pop();
                    let b = self.pop();
                    self.push(Value::Bool(VM::values_equal(a, b)))
                }
                OpCode::OpGreater => binary_op!(self, Bool, >),
                OpCode::OpLess => binary_op!(self, Bool, <),
                OpCode::OpGreaterEqual => binary_op!(self, Bool, >=),
                OpCode::OpLessEqual => binary_op!(self, Bool, <=),
                OpCode::OpNotEqual => {
                    let a = self.pop();
                    let b = self.pop();
                    self.push(Value::Bool(!VM::values_equal(a, b)))
                }
            };
        }
    }

    fn values_equal(a: Value, b: Value) -> bool {
        match (a, b) {
            (Value::Bool(b1), Value::Bool(b2)) => b1 == b2,
            (Value::Number(n1), Value::Number(n2)) => n1 == n2,
            (Value::Empty, Value::Empty) => true,
            (Value::Obj(o1), Value::Obj(o2)) => unsafe {
                match ((*o1).get_type(), (*o2).get_type()) {
                    (ObjType::OString, ObjType::OString) => {
                        let s1 = (o1 as *const ObjString).read();
                        let s2 = (o2 as *const ObjString).read();
                        s1.len == s2.len && s1.chars == s2.chars
                    }
                    _ => false,
                }
            },
            _ => false,
        }
    }

    fn is_falsey(v: Value) -> bool {
        match v {
            Value::Bool(b) => !b,
            Value::Number(_) => false,
            Value::Empty => true,
            Value::Obj(_) => false,
        }
    }

    fn concatenate(o1: *mut dyn Object, o2: *mut dyn Object) -> *mut ObjString {
        unsafe {
            let s1 = (o1 as *const ObjString).read();
            let s2 = (o2 as *const ObjString).read();
            let o = GC::alloc(Layout::new::<ObjString>()) as *mut ObjString;
            (*o).len = s1.len + s2.len;
            let chars = GC::alloc(Layout::array::<u8>((*o).len).unwrap());
            std::ptr::copy(s1.chars, chars, s1.len);
            std::ptr::copy(s2.chars, chars.offset(s1.len as isize), s2.len);
            (*o).chars = chars;
            o
        }
    }
}
