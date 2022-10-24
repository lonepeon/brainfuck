use crate::parser;

pub fn shrink_calls(ast: &[parser::Instruction]) -> Vec<parser::Instruction> {
    let mut optimized = Vec::new();
    let mut instructions = ast.iter().peekable();
    while let Some(instruction) = instructions.next() {
        match instruction {
            parser::Instruction::While(sub_ast) => {
                optimized.push(parser::Instruction::While(shrink_calls(sub_ast)))
            }
            parser::Instruction::MovePointerRight(x) => {
                let mut total = *x;
                while let Some(parser::Instruction::MovePointerRight(x)) = instructions.peek() {
                    instructions.next();
                    total += *x;
                }
                optimized.push(parser::Instruction::MovePointerRight(total))
            }
            parser::Instruction::MovePointerLeft(x) => {
                let mut total = *x;
                while let Some(parser::Instruction::MovePointerLeft(x)) = instructions.peek() {
                    instructions.next();
                    total += *x;
                }
                optimized.push(parser::Instruction::MovePointerLeft(total))
            }
            parser::Instruction::IncrementCell(x) => {
                let mut total = *x;
                while let Some(parser::Instruction::IncrementCell(x)) = instructions.peek() {
                    instructions.next();
                    total += *x;
                }
                optimized.push(parser::Instruction::IncrementCell(total))
            }
            parser::Instruction::DecrementCell(x) => {
                let mut total = *x;
                while let Some(parser::Instruction::DecrementCell(x)) = instructions.peek() {
                    instructions.next();
                    total += *x;
                }
                optimized.push(parser::Instruction::DecrementCell(total))
            }
            instruction => optimized.push(instruction.clone()),
        }
    }

    optimized
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;

    #[test]
    fn shrinked() {
        let input = vec![
            parser::Instruction::IncrementCell(1),
            parser::Instruction::IncrementCell(1),
            parser::Instruction::While(vec![
                parser::Instruction::MovePointerRight(1),
                parser::Instruction::IncrementCell(1),
                parser::Instruction::IncrementCell(1),
                parser::Instruction::IncrementCell(1),
                parser::Instruction::MovePointerRight(1),
                parser::Instruction::IncrementCell(1),
                parser::Instruction::IncrementCell(1),
                parser::Instruction::MovePointerLeft(1),
                parser::Instruction::MovePointerLeft(1),
                parser::Instruction::DecrementCell(1),
            ]),
            parser::Instruction::MovePointerRight(1),
            parser::Instruction::IncrementCell(1),
            parser::Instruction::DisplayCell,
        ];

        let output = vec![
            parser::Instruction::IncrementCell(2),
            parser::Instruction::While(vec![
                parser::Instruction::MovePointerRight(1),
                parser::Instruction::IncrementCell(3),
                parser::Instruction::MovePointerRight(1),
                parser::Instruction::IncrementCell(2),
                parser::Instruction::MovePointerLeft(2),
                parser::Instruction::DecrementCell(1),
            ]),
            parser::Instruction::MovePointerRight(1),
            parser::Instruction::IncrementCell(1),
            parser::Instruction::DisplayCell,
        ];

        assert_eq!(output, shrink_calls(&input))
    }
}
