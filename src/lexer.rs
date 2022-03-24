use crate::token::{Token, TokenType};

#[derive(Debug)]
struct Lexer<'a> {
    input: &'a str,
    position: usize,      // 現在の文字の位置
    read_position: usize, // 次の文字の位置
    ch: char,               // 現在の文字
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: '\u{0000}',
        };
        l.read_char();
        l
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0 as char;
        } else {
            self.ch = self.input.as_bytes()[self.read_position] as char;
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(&mut self) -> Token {
        let tok = match self.ch {
            '=' => Self::new_token(TokenType::ASSIGN, self.ch),
            '+' => Self::new_token(TokenType::PLUS, self.ch),
            ',' => Self::new_token(TokenType::COMMA, self.ch),
            ';' => Self::new_token(TokenType::SEMICOLON, self.ch),
            '(' => Self::new_token(TokenType::LPAREN, self.ch),
            ')' => Self::new_token(TokenType::RPAREN, self.ch),
            '{' => Self::new_token(TokenType::LBRAKET, self.ch),
            '}' => Self::new_token(TokenType::RBRAKET, self.ch),
            '\u{0000}' => Token {
                token_type: TokenType::EOF,
                literal: "".to_string(),
            },
            _ => Self::new_token(TokenType::ILLEGAL, self.ch),
        };
        self.read_char();
        tok
    }

    fn new_token(token_type: TokenType, ch: char) -> Token {
        Token {
            token_type,
            literal: ch.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::token::TokenType;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";

        let tests = vec![
            (TokenType::ASSIGN, "="),
            (TokenType::PLUS, "+"),
            (TokenType::LPAREN, "("),
            (TokenType::RPAREN, ")"),
            (TokenType::LBRAKET, "{"),
            (TokenType::RBRAKET, "}"),
            (TokenType::COMMA, ","),
            (TokenType::SEMICOLON, ";"),
            (TokenType::EOF, ""),
        ];

        let mut l = Lexer::new(input);

        for test in tests {
            let token = l.next_token();
            assert_eq!(token.token_type, test.0);
            assert_eq!(token.literal, test.1);
        }
    }
}
