use clap::{App, Arg, value_t_or_exit};

use crate::interpreter::Interpreter;

mod interpreter;

fn main() {
    #[cfg(windows)] {
        let _ = ansi_term::enable_ansi_support(); //some bf programs use ansi escape codes and these need to be enabled for windows
    }

    let matches = App::new("Brainfuck Interpreter")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Thomas Lienbacher <lienbacher.tom@gmail.com>")
        .about("Runs Brainfuck programs")
        .arg(Arg::with_name("cells")
            .short("c")
            .long("cells")
            .value_name("CELLS")
            .help("Sets the amount of memory cells available")
            .takes_value(true)
            .default_value("30000"))
        .arg(Arg::with_name("file")
            .help("Brainfuck program to run")
            .value_name("FILE")
            .required(true)
            .index(1))
        .arg(Arg::with_name("type")
            .help("Type to store in a cell")
            .value_name("TYPE")
            .short("t")
            .long("type")
            .default_value("i32")
            .possible_values(&["i8", "u8", "i16", "u16", "i32", "u32", "i64", "u64"])
            .case_insensitive(true))
        .get_matches();

    let cells = value_t_or_exit!(matches, "cells", usize);
    let file = matches.value_of("file").unwrap();

    match matches.value_of("type").unwrap() {
        "i8" => { Interpreter::<i8>::new(cells).execute(file) }
        "u8" => { Interpreter::<u8>::new(cells).execute(file) }
        "i16" => { Interpreter::<i16>::new(cells).execute(file) }
        "u16" => { Interpreter::<u16>::new(cells).execute(file) }
        "i32" => { Interpreter::<i32>::new(cells).execute(file) }
        "u32" => { Interpreter::<u32>::new(cells).execute(file) }
        "i64" => { Interpreter::<i64>::new(cells).execute(file) }
        "u64" => { Interpreter::<u64>::new(cells).execute(file) }
        _ => { Interpreter::<i32>::new(cells).execute(file) }
    };
}
