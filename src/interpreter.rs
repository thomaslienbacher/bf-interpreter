use std::result;
use std::io::prelude::*;
use std::io::BufReader;
use std::io;
use std::fs::File;
use text_io::*;
use failure::*;

#[derive(Debug, Fail)]
pub enum BfError {
    #[fail(display = "IO Error: {}", internal)]
    IoError {
        internal: io::Error
    },

    #[fail(display = "None matching bracket at: {}", index)]
    NoneMatchingBracket {
        index: usize
    },

    #[fail(display = "Unexpected input: {}", internal)]
    UnexpectedInput {
        internal: text_io::Error
    },
}

impl From<io::Error> for BfError {
    fn from(item: io::Error) -> Self {
        BfError::IoError { internal: item }
    }
}

impl From<text_io::Error> for BfError {
    fn from(item: text_io::Error) -> Self {
        BfError::UnexpectedInput { internal: item }
    }
}

#[derive(Debug)]
enum Commands {
    PointerUp,
    PointerDown,
    Increment,
    Decrement,
    Print,
    Scan,
    LoopBegin {
        other_pair: usize
    },
    LoopEnd {
        other_pair: usize
    },
}

pub struct Interpreter {
    program: Vec<Commands>,
    cells: Vec<i64>,
    pointer: usize,
}

impl Interpreter {
    pub fn new(cells: usize) -> Self {
        Interpreter {
            program: Vec::new(),
            cells: vec![0; cells],
            pointer: 0,
        }
    }

    fn compile(&mut self, file: String) -> result::Result<(), BfError> {
        let f = File::open(file)?;
        let mut reader = BufReader::new(f);
        let mut line = String::new();
        let mut index: usize = 0;

        let mut loop_begins = Vec::new();

        while reader.read_line(&mut line)? != 0 {
            let chars = line.chars();

            for c in chars {
                match c {
                    '>' => {
                        self.program.push(Commands::PointerUp);
                        index += 1;
                    }
                    '<' => {
                        self.program.push(Commands::PointerDown);
                        index += 1;
                    }
                    '+' => {
                        self.program.push(Commands::Increment);
                        index += 1;
                    }
                    '-' => {
                        self.program.push(Commands::Decrement);
                        index += 1;
                    }
                    '.' => {
                        self.program.push(Commands::Print);
                        index += 1;
                    }
                    ',' => {
                        self.program.push(Commands::Scan);
                        index += 1;
                    }
                    '[' => {
                        loop_begins.push(index);
                        self.program.push(Commands::LoopBegin { other_pair: std::usize::MAX });
                        index += 1;
                    }
                    ']' => {
                        let begin = match loop_begins.pop() {
                            Some(t) => { t }
                            None => { return Err(BfError::NoneMatchingBracket { index }); }
                        };
                        self.program[begin] = Commands::LoopBegin { other_pair: index };
                        self.program.push(Commands::LoopEnd { other_pair: begin });
                        index += 1;
                    }

                    _ => {}
                }
            }

            line.clear();
        }

        if loop_begins.len() > 0 {
            return Err(BfError::NoneMatchingBracket { index: loop_begins.pop().unwrap() });
        }

        Ok(())
    }

    fn interpret(&mut self) -> Result<(), BfError> {
        let mut i = 0;

        while i < self.program.len() {
            let c = &self.program[i];

            match c {
                Commands::PointerUp => {
                    self.pointer += 1;
                    i += 1;
                }
                Commands::PointerDown => {
                    self.pointer -= 1;
                    i += 1;
                }
                Commands::Increment => {
                    self.cells[self.pointer] += 1;
                    i += 1;
                }
                Commands::Decrement => {
                    self.cells[self.pointer] -= 1;
                    i += 1;
                }
                Commands::Print => {
                    print!("{}", (self.cells[self.pointer] as u8) as char);
                    io::stdout().flush()?;
                    i += 1;
                }
                Commands::Scan => {
                    let g: char = try_read!()?;
                    self.cells[self.pointer] = g as i64;
                    i += 1;
                }
                Commands::LoopBegin { other_pair } => {
                    if self.cells[self.pointer] == 0 {
                        i = *other_pair;
                    }
                    i += 1;
                }
                Commands::LoopEnd { other_pair } => {
                    i = *other_pair;
                }
            }
        }

        Ok(())
    }

    pub fn run(&mut self, file: String) -> result::Result<(), BfError> {
        self.compile(file)?;
        self.interpret()?;
        Ok(())
    }
}