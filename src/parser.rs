use crate::lexer;

#[derive(Debug, Eq, PartialEq)]
pub struct ParserError(&'static str);

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to parse source code: {}", self.0)
    }
}

impl std::error::Error for ParserError {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Instruction {
    MovePointerRight(usize),
    MovePointerLeft(usize),
    IncrementCell(u8),
    DecrementCell(u8),
    DisplayCell,
    ReplaceCell,
    While(Vec<Instruction>),
}

pub fn parse(program: &str) -> Result<Vec<Instruction>, ParserError> {
    let (instructions, _) = parse_instructions(program, false)?;

    Ok(instructions)
}

fn parse_instructions(
    program: &str,
    search_end_block: bool,
) -> Result<(Vec<Instruction>, usize), ParserError> {
    let mut ast = Vec::new();
    let mut index = 0;
    let chars: Vec<char> = program.chars().collect();
    while index < program.len() {
        let c = chars[index];
        index += 1;
        let instruction = lexer::lex(c);

        match instruction {
            lexer::Token::GreaterThan => ast.push(Instruction::MovePointerRight(1)),
            lexer::Token::LessThan => ast.push(Instruction::MovePointerLeft(1)),
            lexer::Token::Plus => ast.push(Instruction::IncrementCell(1)),
            lexer::Token::Minus => ast.push(Instruction::DecrementCell(1)),
            lexer::Token::Dot => ast.push(Instruction::DisplayCell),
            lexer::Token::Comma => ast.push(Instruction::ReplaceCell),
            lexer::Token::LBracket => {
                let (sub_ast, i) = parse_instructions(&program[index..], true)?;
                index += i;
                ast.push(Instruction::While(sub_ast))
            }
            lexer::Token::RBracket => {
                if !search_end_block {
                    return Err(ParserError("unexpected closing bracket"));
                }
                return Ok((ast, index));
            }
            lexer::Token::Unknown => continue,
        }
    }

    if search_end_block {
        return Err(ParserError("missing closing bracket"));
    }

    Ok((ast, index))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsable_program() {
        let dummy = "++[>++>++<<-]>+.";

        let ast = vec![
            Instruction::IncrementCell(1),
            Instruction::IncrementCell(1),
            Instruction::While(vec![
                Instruction::MovePointerRight(1),
                Instruction::IncrementCell(1),
                Instruction::IncrementCell(1),
                Instruction::MovePointerRight(1),
                Instruction::IncrementCell(1),
                Instruction::IncrementCell(1),
                Instruction::MovePointerLeft(1),
                Instruction::MovePointerLeft(1),
                Instruction::DecrementCell(1),
            ]),
            Instruction::MovePointerRight(1),
            Instruction::IncrementCell(1),
            Instruction::DisplayCell,
        ];

        assert_eq!(ast, parse(dummy).unwrap())
    }

    #[test]
    fn missing_closing_bracket() {
        let dummy = "++[>++>++<<->+.";
        let err = ParserError("missing closing bracket");

        assert_eq!(err, parse(dummy).unwrap_err())
    }

    #[test]
    fn unexpected_closing_bracket() {
        let dummy = "++[>++>++<<-]]>+.";
        let err = ParserError("unexpected closing bracket");

        assert_eq!(err, parse(dummy).unwrap_err())
    }
}
