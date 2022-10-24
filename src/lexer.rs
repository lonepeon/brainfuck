#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    GreaterThan,
    LessThan,
    Plus,
    Minus,
    Dot,
    Comma,
    LBracket,
    RBracket,
    Unknown,
}

pub fn lex(c: char) -> Token {
    match c {
        '>' => Token::GreaterThan,
        '<' => Token::LessThan,
        '+' => Token::Plus,
        '-' => Token::Minus,
        '.' => Token::Dot,
        ',' => Token::Comma,
        '[' => Token::LBracket,
        ']' => Token::RBracket,
        _ => Token::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greater_than() {
        assert_eq!(Token::GreaterThan, lex('>'))
    }

    #[test]
    fn less_than() {
        assert_eq!(Token::LessThan, lex('<'))
    }

    #[test]
    fn plus() {
        assert_eq!(Token::Plus, lex('+'))
    }

    #[test]
    fn minus() {
        assert_eq!(Token::Minus, lex('-'))
    }

    #[test]
    fn dot() {
        assert_eq!(Token::Dot, lex('.'))
    }

    #[test]
    fn comma() {
        assert_eq!(Token::Comma, lex(','))
    }

    #[test]
    fn lbracket() {
        assert_eq!(Token::LBracket, lex('['))
    }

    #[test]
    fn rbracket() {
        assert_eq!(Token::RBracket, lex(']'))
    }

    #[test]
    fn unknown_letter() {
        assert_eq!(Token::Unknown, lex('a'))
    }

    #[test]
    fn unknown_space() {
        assert_eq!(Token::Unknown, lex(' '))
    }
}
