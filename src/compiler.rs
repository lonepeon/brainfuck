pub mod rust {
    use crate::parser;
    use askama::Template;

    #[derive(Debug)]
    pub struct CompilationError(&'static str);

    impl std::error::Error for CompilationError {}

    impl std::fmt::Display for CompilationError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    #[derive(Template)]
    #[template(path = "program.rs.txt")]
    struct ProgramTemplate<'a> {
        program: &'a Program,
    }

    struct Program {
        default_memory: usize,
        read_buffer_definition: bool,
        body: Vec<String>,
    }

    impl Program {
        pub fn new(default_memory: usize) -> Self {
            Program {
                default_memory,
                read_buffer_definition: false,
                body: Vec::new(),
            }
        }
    }

    pub fn compile(src: &str, dest: &str) -> Result<(), CompilationError> {
        std::process::Command::new("rustc")
            .arg("-o")
            .arg(dest)
            .arg(src)
            .status()
            .map_err(|_| CompilationError("failed to compile brainfuck Rust generated program"))?;

        Ok(())
    }

    pub fn generate<W: std::io::Write>(
        default_memory: usize,
        ast: &[parser::Instruction],
        out: &mut W,
    ) -> std::io::Result<()> {
        let mut program = Program::new(default_memory);
        do_generate(&mut program, 2, ast);
        write_program(&program, out)
    }

    fn write_program<W: std::io::Write>(program: &Program, out: &mut W) -> std::io::Result<()> {
        let tmpl = ProgramTemplate { program };
        println!("{}", tmpl.render().unwrap());

        writeln!(out, "{}", tmpl.render().unwrap())
    }

    fn do_generate(program: &mut Program, spacing: usize, ast: &[parser::Instruction]) {
        for instruction in ast {
            match instruction {
                parser::Instruction::MovePointerRight(n) => program
                    .body
                    .push(format!("mem.move_to_cell(mem.index + {});", n)),
                parser::Instruction::MovePointerLeft(n) => program
                    .body
                    .push(format!("mem.move_to_cell(mem.index - {});", n)),
                parser::Instruction::IncrementCell(n) => {
                    program.body.push(format!("mem.increment_cell({});", n))
                }
                parser::Instruction::DecrementCell(n) => {
                    program.body.push(format!("mem.decrement_cell({});", n))
                }
                parser::Instruction::DisplayCell => {
                    program
                        .body
                        .push("print!(\"{}\", mem.current_cell_value() as char);".to_string());
                }
                parser::Instruction::ReplaceCell => {
                    program.read_buffer_definition = true;
                    program.body.push(
                        "reader.read_exact(&mut buffer).expect(\"failed to read from STDIN\");"
                            .to_string(),
                    );
                    program
                        .body
                        .push("mem.set_current_cell_value(buffer[0]);".to_string());
                }
                parser::Instruction::While(sub_ast) => {
                    program
                        .body
                        .push("while mem.current_cell_value() != 0 {".to_string());
                    do_generate(program, spacing + 2, sub_ast);
                    program.body.push("}".to_string());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::BufWriter;

    use crate::parser;

    use super::*;

    #[test]
    fn generate_proper_source() {
        let ast = vec![
            parser::Instruction::IncrementCell(8),
            parser::Instruction::While(vec![
                parser::Instruction::MovePointerRight(1),
                parser::Instruction::IncrementCell(4),
            ]),
            parser::Instruction::DisplayCell,
        ];
        let mut out = BufWriter::new(Vec::new());
        rust::generate(4096, &ast, &mut out).unwrap();
        let actual = String::from_utf8(out.into_inner().unwrap()).unwrap();
        let expected = std::fs::read_to_string("golden-files/compiler/proper_source.txt").unwrap();

        assert_eq!(expected, actual)
    }
}
