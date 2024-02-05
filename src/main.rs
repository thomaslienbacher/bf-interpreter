use clap::{Parser, Subcommand};

use crate::interpreter::Interpreter;

mod interpreter;


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Brainfuck program to run
    file: String,

    /// The amount of memory cells available
    #[arg(short, long, default_value_t = 30000)]
    cells: usize,

    /// Datatype used for a cell
    #[arg(short, long, default_value_t = ("i32".into()), value_parser = clap::builder::PossibleValuesParser::new(["i8", "u8", "i16", "u16", "i32", "u32", "i64", "u64"]))]
    datatype: String,
}

fn main() {
    #[cfg(windows)] {
        let _ = ansi_term::enable_ansi_support(); //some bf programs use ansi escape codes and these need to be enabled for windows
    }

    let matches = Cli::parse();
    let cells = matches.cells;
    let file = matches.file;

    match matches.datatype.as_str() {
        "i8" => { Interpreter::<i8>::new(cells).execute(&file) }
        "u8" => { Interpreter::<u8>::new(cells).execute(&file) }
        "i16" => { Interpreter::<i16>::new(cells).execute(&file) }
        "u16" => { Interpreter::<u16>::new(cells).execute(&file) }
        "i32" => { Interpreter::<i32>::new(cells).execute(&file) }
        "u32" => { Interpreter::<u32>::new(cells).execute(&file) }
        "i64" => { Interpreter::<i64>::new(cells).execute(&file) }
        "u64" => { Interpreter::<u64>::new(cells).execute(&file) }
        _ => { Interpreter::<i32>::new(cells).execute(&file) }
    };
}
