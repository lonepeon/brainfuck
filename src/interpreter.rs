use crate::parser;

#[derive(Debug)]
pub struct RuntimeError(&'static str);

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed during execution: {}", self.0)
    }
}

impl std::error::Error for RuntimeError {}

struct Memory {
    cells: Vec<u8>,
    index: usize,
}

impl Memory {
    pub fn new(memory: usize) -> Self {
        Self {
            cells: vec![0; memory],
            index: 0,
        }
    }

    pub fn increment_cell(&mut self, n: u8) {
        let value = self.cells[self.index];
        let new_value = if n < 255 - value {
            value + n
        } else {
            n - (255 - value)
        };

        self.cells[self.index] = new_value;
    }

    pub fn decrement_cell(&mut self, n: u8) {
        let value = self.cells[self.index];
        let new_value = if n > value {
            255 - (n - value)
        } else {
            value - n
        };

        self.cells[self.index] = new_value;
    }

    pub fn next_cell(&mut self, n: usize) {
        self.index += n;
        if self.index >= self.cells.len() {
            self.cells.resize(self.index + 1, 0);
        }
    }

    pub fn previous_cell(&mut self, n: usize) -> Result<(), RuntimeError> {
        if n > self.index {
            return Err(RuntimeError("negative memory address are invalid"));
        }
        self.index -= n;
        Ok(())
    }

    pub fn current_cell_value(&self) -> u8 {
        self.cells[self.index]
    }

    pub fn set_current_cell_value(&mut self, value: u8) {
        self.cells[self.index] = value;
    }
}

pub fn run<R: std::io::BufRead, W: std::io::Write>(
    memory: usize,
    ast: &Vec<parser::Instruction>,
    stdin: &mut R,
    stdout: &mut W,
) -> Result<(), RuntimeError> {
    let mut memory = Memory::new(memory);

    execute(&mut memory, ast, stdin, stdout)
}

fn execute<R: std::io::BufRead, W: std::io::Write>(
    memory: &mut Memory,
    ast: &Vec<parser::Instruction>,
    stdin: &mut R,
    stdout: &mut W,
) -> Result<(), RuntimeError> {
    let mut buffer: [u8; 1] = [0];

    for instruction in ast {
        match instruction {
            parser::Instruction::MovePointerRight(n) => memory.next_cell(*n),
            parser::Instruction::MovePointerLeft(n) => memory.previous_cell(*n)?,
            parser::Instruction::IncrementCell(n) => memory.increment_cell(*n),
            parser::Instruction::DecrementCell(n) => memory.decrement_cell(*n),
            parser::Instruction::DisplayCell => {
                if write!(stdout, "{}", memory.current_cell_value() as char).is_err() {
                    return Err(RuntimeError("cannot write to stdout"));
                }
            }
            parser::Instruction::ReplaceCell => {
                if stdin.read_exact(&mut buffer).is_err() {
                    return Err(RuntimeError("cannot read STDIN"));
                }
                memory.set_current_cell_value(buffer[0])
            }
            parser::Instruction::While(sub_ast) => {
                while memory.current_cell_value() != 0 {
                    execute(memory, sub_ast, stdin, stdout)?
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;

    #[test]
    fn hello_world() {
        let source = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
        let ast = parser::parse(source).unwrap();
        let mut stdin = "".as_bytes();
        let mut output = std::io::BufWriter::new(Vec::new());

        run(512, &ast, &mut stdin, &mut output).unwrap();

        let output = String::from_utf8(output.into_inner().unwrap()).unwrap();
        assert_eq!("Hello World!\n".to_string(), output);
    }

    #[test]
    fn tic_tac_toe() {
        let source = "--->--->>>>->->->>>>>-->>>>>>>>>>>>>>>>>>+>>++++++++++[<<++[--<+<<+<<+>>>>[>[<->>+++>>[-]+++<<<+[<++>>+<--]]+>+++++[>>+++++++++<<-]>>++++.[-]>>+[<<<<+>>+>>-]<<<<<<[>+<-]<<]++++++++++.[-]>++]-->>[-->[-]>]<<[>>--[-[-[-----[>+>+++++++<<+]-->>-.----->,[<->-]<[[<]+[->>]<-]<[<<,[-]]>>>>]>]<[>-[+<+++]+<+++[+[---->]+<<<<<<[>>]<[-]]>[<+[---->]++[<]<[>]>[[>]+>+++++++++<<-[<]]]>[>>>>]]<[-[[>+>+<<-]>[<+>-]++>+>>]<[<<++[-->>[-]]>[[-]>[<<+>>-]>]]]<[[[<<]-[>>]<+<-]>[-<+]<<[<<]-<[>[+>>]>[>]>[-]]>[[+>>]<-->>[>]+>>>]]<[-[--[+<<<<--[+>[-]>[<<+>+>-]<<[>>+<<-]]++[>]]<<[>+>+<<-]>--[<+>-]++>>>]<[<<<[-]+++>[-]>[<+>>>+<<-]+>>>]]<[+[[<]<<[<<]-<->>+>[>>]>[>]<-]+[-<+]<++[[>+<-]++<[<<->>+]<++]<<<<<<<+>>>+>>>+[<<<->+>+>+[<<<<<<<+>->+>>>->->+[<<<<<->+>+>>+>+[<<<<->->+>->+[<<<<<<<<+>->>+>>>->+>+[<<<<<->>+>>->+[<<<<+>->+>>+]]]]]]]+++[[>+<-]<+++]--->>[-[<->-]<++>>]++[[<->-]>>]>[>]]<]]<]";
        let ast = parser::parse(source).unwrap();
        let mut stdin = "5\n7\n6\n".as_bytes();
        let mut output = std::io::BufWriter::new(Vec::new());

        run(512, &ast, &mut stdin, &mut output).unwrap();

        let output = String::from_utf8(output.into_inner().unwrap()).unwrap();
        assert_eq!(
            "X23\n456\n789\n>X23\n4O6\n78X\n>X2X\n4O6\nO8X\n>XXX\n4OO\nO8X\n".to_string(),
            output
        );
    }
}
