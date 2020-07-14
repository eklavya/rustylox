mod chunk;
mod common;
mod compiler;
mod gc;
mod obj;
mod parser;
mod scanner;
mod value;
mod vm;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{stdin, stdout, Read, Write};
use std::process::exit;
use vm::*;

fn main() {
    let mut vm = VM::new();
    vm.init();

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() == 1 {
        repl(&mut vm);
    } else if args.len() == 2 {
        if run_file(&args[0], &mut vm).is_err() {
            eprintln!("Could not open file {}", &args[0]);
            exit(74)
        }
    } else {
        eprintln!("Usage: rustylox [path]");
        exit(64);
    }

    vm.free();
}

fn repl(vm: &mut VM) {
    let reader = stdin();
    loop {
        print!("> ");
        stdout().flush();
        let mut input = String::new();
        reader.read_line(&mut input);
        if input.trim().eq(":exit".into()) {
            println!();
            break;
        }

        vm.interpret(&input);
    }
}

fn run_file(path: &String, _vm: &mut VM) -> Result<(), Box<dyn Error>> {
    let mut source = File::open(path)?;
    let len = source.metadata()?.len();
    let mut input_raw = Vec::with_capacity(len as usize);
    source.read_to_end(&mut input_raw)?;
    let _input = String::from_utf8(input_raw)?;
    Ok(())
    // match vm.interpret(input) {
    //     InterpretResult::InterpretCompileError => exit(65),
    //     InterpretResult::InterpretRuntimeError => exit(70),
    //     _ => (),
    // }
}
