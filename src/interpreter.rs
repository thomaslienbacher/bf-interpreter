use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::ops::{AddAssign, SubAssign};
use std::result;

use num_traits::FromPrimitive;
use num_traits::identities::{One, Zero};
use num_traits::ToPrimitive;
use text_io::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BfError {
    #[error("IO Error: {}", internal)]
    IoError {
        internal: io::Error
    },

    #[error("None matching bracket at: {}", index)]
    NoneMatchingBracket {
        index: usize
    },

    #[error("Unexpected input: {}", internal)]
    UnexpectedInput {
        internal: text_io::Error
    },

    #[error("Memory out of bounds at: {}", index)]
    MemoryOutOfBounds {
        index: usize
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

pub struct Interpreter<T> {
    program: Vec<Commands>,
    cells: Vec<T>,
    pointer: usize,
}

impl<T: Clone + Zero + One + ToPrimitive + FromPrimitive + AddAssign + SubAssign + PartialEq> Interpreter<T> {
    pub fn new(cells: usize) -> Self {
        Interpreter {
            program: Vec::new(),
            cells: vec![T::zero(); cells],
            pointer: 0,
        }
    }

    fn compile(&mut self, file: &String) -> result::Result<(), BfError> {
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

        if !loop_begins.is_empty() {
            return Err(BfError::NoneMatchingBracket { index: loop_begins.pop().unwrap() });
        }

        Ok(())
    }

    fn get_cell(&self, index: usize) -> Result<&T, BfError> {
        match self.cells.get(self.pointer) {
            Some(o) => { Ok(o) }
            None => Err(BfError::MemoryOutOfBounds { index })
        }
    }

    fn get_cell_mut(&mut self, index: usize) -> Result<&mut T, BfError> {
        match self.cells.get_mut(self.pointer) {
            Some(o) => { Ok(o) }
            None => Err(BfError::MemoryOutOfBounds { index })
        }
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
                    *self.get_cell_mut(i)? += T::one();
                    i += 1;
                }
                Commands::Decrement => {
                    *self.get_cell_mut(i)? -= T::one();
                    i += 1;
                }
                Commands::Print => {
                    print!("{}", self.get_cell(i)?.to_u8().unwrap() as char);
                    io::stdout().flush()?;
                    i += 1;
                }
                Commands::Scan => {
                    let g: char = try_read!()?;
                    *self.get_cell_mut(i)? = T::from_u8(g as u8).unwrap();
                    i += 1;
                }
                Commands::LoopBegin { other_pair } => {
                    if *(self.get_cell(i))? == T::zero() {
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

    pub fn run(&mut self, file: &String) -> Result<(), BfError> {
        self.compile(file)?;
        self.interpret()?;
        Ok(())
    }

    pub fn execute(&mut self, file: &String) {
        match self.run(file) {
            Ok(_) => {}
            Err(e) => eprintln!("{}", e)
        }
    }
}