mod interpreter;

use clap::{App, Arg, value_t_or_exit};
use crate::interpreter::Interpreter;

fn main() {
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
            .default_value("16000"))
        .arg(Arg::with_name("file")
            .help("Brainfuck program to run")
            .value_name("FILE")
            .required(true)
            .index(1))
        .get_matches();

    let cells = value_t_or_exit!(matches, "cells", usize);
    let file = matches.value_of("file").unwrap().to_string();

    let mut ip = Interpreter::new(cells);

    match ip.run(file) {
        Ok(_) => {}
        Err(e) => eprintln!("Error: {}", e)
    }
}