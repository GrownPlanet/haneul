use crate::token::{Token, TokenType};

pub struct Lexer {
    source: Vec<char>,
    current_char: char,
    current_pos: usize,
}

impl Lexer {
    pub fn new(mut source: String) -> Self {
        source.push('\n');
        let source: Vec<char> = source.chars().collect();
        Self {
            // should be fine since we just appended a newline to source
            current_char: source[0],
            source,
            current_pos: 0,
        }
    }

    pub fn next_char(&mut self) {
        self.current_pos += 1;

        if self.current_pos >= self.source.len() {
            self.current_char = '\0';
        } else {
            self.current_char = self.source[self.current_pos];
        }
    }

    fn peek(&self) -> char {
        if self.current_pos + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current_pos + 1]
        }
    }

    pub fn get_token(&mut self) -> Token {
        self.skip_whitespace();
        self.skip_comment();

        let mut current_str: String = self.current_char.into();

        match self.current_char {
            '+' => Token::new(current_str, TokenType::Plus),
            '-' => Token::new(current_str, TokenType::Minus),
            '*' => Token::new(current_str, TokenType::Asterisk),
            '/' => Token::new(current_str, TokenType::Slash),
            '\n' => Token::new(current_str, TokenType::Newline),
            '\0' => Token::new(current_str, TokenType::Eof),
            '=' => {
                if self.peek() == '=' {
                    self.next_char();
                    current_str.push(self.current_char);
                    Token::new(current_str, TokenType::EqEq)
                } else {
                    Token::new(current_str, TokenType::Eq)
                }
            }
            '>' => {
                if self.peek() == '=' {
                    self.next_char();
                    current_str.push(self.current_char);
                    Token::new(current_str, TokenType::GtEq)
                } else {
                    Token::new(current_str, TokenType::Gt)
                }
            }
            '<' => {
                if self.peek() == '=' {
                    self.next_char();
                    current_str.push(self.current_char);
                    Token::new(current_str, TokenType::LtEq)
                } else {
                    Token::new(current_str, TokenType::Lt)
                }
            }
            '!' => {
                if self.peek() == '=' {
                    self.next_char();
                    current_str.push(self.current_char);
                    Token::new(current_str, TokenType::NotEq)
                } else {
                    Self::die(format!["Expected !=, got !{}", self.peek()]);
                }
            }
            '"' => {
                self.next_char();
                let mut string = String::new();

                while self.current_char != '"' {
                    match self.current_char {
                        '%' => string.push_str("\\%"),
                        '\\' => string.push_str("\\\\"),
                        _ => string.push(self.current_char),
                    }
                    self.next_char();
                }

                Token::new(string, TokenType::String)
            }
            '0'..='9' | '.' => {
                let mut raw_num = String::new();
                let mut is_float = self.current_char == '.';
                raw_num.push(self.current_char);

                while self.peek().is_ascii_digit() || (self.peek() == '.' && !is_float) {
                    self.next_char();
                    raw_num.push(self.current_char);

                    if self.current_char == '.' {
                        is_float = true;
                    }
                }

                Token::new(raw_num, TokenType::Number)
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut ident = String::new();
                ident.push(self.current_char);

                while self.peek().is_alphanumeric() {
                    self.next_char();
                    ident.push(self.current_char);
                }

                if let Some(tokentype) = Self::is_keyword(&ident) {
                    Token::new(ident, tokentype)
                } else {
                    Token::new(ident, TokenType::Ident)
                }
            }
            _ => Self::die(format!("unknown token: {}", self.current_char)),
        }
    }

    fn is_keyword(token_text: &str) -> Option<TokenType> {
        // Could be replaced with a hashmap, but it doesn't have enough keywords to be efficient
        let keywords = [
            ("LABEL", TokenType::Label),
            ("GOTO", TokenType::Goto),
            ("PRINT", TokenType::Print),
            ("INPUT", TokenType::Input),
            ("LET", TokenType::Let),
            ("IF", TokenType::If),
            ("THEN", TokenType::Then),
            ("ENDIF", TokenType::Endif),
            ("WHILE", TokenType::While),
            ("REPEAT", TokenType::Repeat),
            ("ENDWHILE", TokenType::EndWhile),
        ];

        for (keyword, tokentype) in keywords {
            if token_text == keyword {
                return Some(tokentype);
            }
        }
        None
    }

    fn skip_comment(&mut self) {
        if self.current_char == '#' {
            while self.current_char != '\n' {
                self.next_char();
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while self.current_char == ' ' || self.current_char == '\t' || self.current_char == '\r' {
            self.next_char();
        }
    }

    fn die(message: String) -> ! {
        println!("Error while lexing: {}", message);
        std::process::exit(1);
    }
}
