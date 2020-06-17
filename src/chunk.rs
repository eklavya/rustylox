use super::common::OpCode;
use super::value::*;
use std::ops::Shr;

#[derive(Copy, Clone)]
struct Line {
    line_num: i32,
    count: i32,
}

pub struct Chunk {
    pub code: Vec<u8>,
    constants: Vec<Value>,
    lines: Vec<Line>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: vec![],
            constants: vec![],
            lines: vec![],
        }
    }

    #[inline(always)]
    pub fn get_constant(&self, ind: u8) -> Value {
        self.constants[ind as usize]
    }

    fn set_line(&mut self, line: i32) -> () {
        match self.lines.last_mut() {
            Some(last) => {
                if last.line_num == line {
                    last.count += 1;
                } else {
                    self.lines.push(Line {
                        line_num: line,
                        count: 1,
                    });
                }
            }
            None => self.lines.push(Line {
                line_num: line,
                count: 1,
            }),
        }
    }

    pub fn get_line(&self, offset: usize) -> i32 {
        let mut curr = 0;
        for x in self.lines.iter() {
            if curr < offset {
                curr += x.count as usize;
            } else {
                return x.line_num;
            }
        }
        return curr as i32;
    }

    pub fn write(&mut self, byte: u8, line: i32) -> () {
        self.code.push(byte);
        self.set_line(line);
    }

    pub fn write_long(&mut self, num: u16, line: i32) -> () {
        let lower = num & 0x00ff;
        let higher = num & 0xff00;
        self.code.push(lower as u8);
        self.code.push(higher.shr(8) as u8);
        self.set_line(line);
    }

    pub fn free(&mut self) -> () {
        self.code = vec![];
        self.constants = vec![];
        self.lines = vec![];
    }

    #[cfg(debug_assertions)]
    pub fn disassemble(&self, name: &str) -> () {
        println!("== {} ==", name);
        let mut offset = 0;
        while offset < self.code.len() {
            offset = self.disassemble_instruction(offset);
        }
    }

    #[cfg(not(debug_assertions))]
    pub fn disassemble(&self, name: &str) -> () {}

    #[cfg(debug_assertions)]
    pub fn disassemble_instruction(&self, offset: usize) -> usize {
        print!("{:04} ", offset);
        let line = self.get_line(offset);
        if offset > 0 && line == self.get_line(offset - 1) {
            print!("   | ")
        } else {
            print!("{:4} ", line)
        }
        let i = unsafe { OpCode::from_unchecked(self.code[offset]) };
        match i {
            OpCode::OpReturn => simple_instruction("OP_RETURN".into(), offset),
            OpCode::OpConstant => self.constant_instruction("OP_CONSTANT".into(), offset),
            OpCode::OpNegate => simple_instruction("OP_NEGATE".into(), offset),
            OpCode::OpAdd => simple_instruction("OP_ADD".into(), offset),
            OpCode::OpSubtract => simple_instruction("OP_SUBTRACT".into(), offset),
            OpCode::OpMultiply => simple_instruction("OP_MULTIPLY".into(), offset),
            OpCode::OpDivide => simple_instruction("OP_DIVIDE".into(), offset),
            OpCode::OpFalse => simple_instruction("OP_FALSE".into(), offset),
            OpCode::OpTrue => simple_instruction("OP_TRUE".into(), offset),
            OpCode::OpNil => simple_instruction("OP_NIL".into(), offset),
            // _ => {
            //     println!("Unknown opcode {:?}", i);
            //     offset + 1
            // }
        }
    }

    #[cfg(not(debug_assertions))]
    pub fn disassemble_instruction(&self, offset: usize) -> () {}

    fn constant_instruction(&self, name: String, offset: usize) -> usize {
        let constant = self.code[offset + 1] as usize;
        print!("{:16} {:4} ", name, constant);
        print_value(self.constants[constant]);
        println!();
        offset + 2
    }

    pub fn add_constant(&mut self, value: Value) -> u8 {
        self.constants.push(value);
        (self.constants.len() - 1) as u8
    }
}

fn simple_instruction(name: String, offset: usize) -> usize {
    println!("{}", name);
    offset + 1
}
