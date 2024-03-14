pub struct Lexer {
    source: String,
    current_char: char,
    current_pos: usize,
}

impl Lexer {
    pub fn new(mut source: String) -> Self {
        source.push('\n');
        Self {
            // should be fine since we just appended a newline to source
            current_char: source.chars().next().unwrap(),
            source,
            current_pos: 0,
        }
    }

    pub fn next_char(&mut self) {
        self.current_pos += 1;

        if self.current_pos > self.source.len() {
            self.current_char = '\0';
        } else {
            self.current_char = self.source.chars().nth(self.current_pos).unwrap();
        }
    }

    pub fn peek(&self) -> char {
        if self.current_pos + 1 >= self.source.len() {
            '\0'
        } else {
            self.source.chars().nth(self.current_pos + 1).unwrap()
        }
    }

    pub fn current_char(&self) -> char {
        self.current_char
    }

    pub fn get_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        self.skip_comment();

        let mut current_str: String = self.current_char.into();

        match self.current_char {
            '+' => Some(Token::new(current_str, TokenType::Plus)),
            '-' => Some(Token::new(current_str, TokenType::Minus)),
            '*' => Some(Token::new(current_str, TokenType::Asterisk)),
            '/' => Some(Token::new(current_str, TokenType::Slash)),
            '\n' => Some(Token::new(current_str, TokenType::Newline)),
            '\0' => Some(Token::new(current_str, TokenType::Eof)),
            '=' => {
                if self.peek() == '=' {
                    self.next_char();
                    current_str.push(self.current_char);
                    Some(Token::new(current_str, TokenType::EqEq))
                } else {
                    Some(Token::new(current_str, TokenType::Eq))
                }
            }
            '>' => {
                if self.peek() == '=' {
                    self.next_char();
                    current_str.push(self.current_char);
                    Some(Token::new(current_str, TokenType::GtEq))
                } else {
                    Some(Token::new(current_str, TokenType::Gt))
                }
            }
            '<' => {
                if self.peek() == '=' {
                    self.next_char();
                    current_str.push(self.current_char);
                    Some(Token::new(current_str, TokenType::LtEq))
                } else {
                    Some(Token::new(current_str, TokenType::Lt))
                }
            }
            '!' => {
                if self.peek() == '=' {
                    self.next_char();
                    current_str.push(self.current_char);
                    Some(Token::new(current_str, TokenType::NotEq))
                } else {
                    self.die(format!["Expected !=, got !{}", self.peek()]);
                }
            }
            '"' => {
                self.next_char();
                let mut string = String::new();

                while self.current_char != '"' {
                    string.push(self.current_char);
                    self.next_char();
                }

                Some(Token::new(string, TokenType::Sting))
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

                return if is_float {
                    Some(Token::new(raw_num, TokenType::Float))
                } else {
                    Some(Token::new(raw_num, TokenType::Int))
                };
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut ident = String::new();
                ident.push(self.current_char);

                while self.peek().is_alphanumeric() {
                    self.next_char();
                    ident.push(self.current_char);
                }

                if let Some(tokentype) = Self::is_keyword(&ident) {
                    Some(Token::new(ident, tokentype))
                } else {
                    Some(Token::new(ident, TokenType::Ident))
                }
            }
            _ => self.die(format!("unknown token: {}", self.current_char)),
        }
    }

    fn is_keyword(token_text: &str) -> Option<TokenType> {
        let keywords = [
            // comment
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

    fn die(&self, message: String) -> ! {
        panic!("Error while lexing: {}", message);
    }
}

#[derive(Debug)]
pub struct Token {
    text: String,
    kind: TokenType,
}

impl Token {
    pub fn new(text: String, kind: TokenType) -> Self {
        Self { text, kind }
    }
}

#[derive(Debug)]
#[rustfmt::skip]
pub enum TokenType {
    Eof, Newline, Int, Float, Ident, Sting, 
    // keywords
    Label, Goto, Print, Input, Let, If, Then, Endif, While, Repeat, EndWhile,
    // operators
    Eq, Plus, Minus, Asterisk, Slash, EqEq, NotEq, Lt, LtEq, Gt, GtEq
}
