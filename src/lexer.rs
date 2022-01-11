use std::collections::HashSet;

macro_rules! set {
    ( $( $x:expr ),* ) => {  // Match zero or more comma delimited items
        {
            let mut temp_set = HashSet::new();  // Create a mutable HashSet
            $(
                temp_set.insert($x); // Insert each item matched into the HashSet
            )*
            temp_set // Return the populated HashSet
        }
    };
}

#[derive(PartialEq, Debug)]
pub enum TokenType {
    OPBR,
    CLBR,
    OPSQ,
    CLSQ,
    COMMA,
    STRING,
    NUMBER,
    TRUE,
    FALSE,
    NULL,
    COLON,
    EOF,
    NONE
}

#[derive(Debug)]
pub struct Token {
    pub ttype: TokenType,
}

pub struct Lexer {
    input: String,
    pos: usize,
    current_char: Option<char>,
    whitespace: HashSet<char>,
    esc: HashSet<char>,
    unsafe_code_point: HashSet<char>,
}

impl Lexer {
    pub fn new(input: &String) -> Lexer {
        let mut lexer = Lexer {
            input: input.clone(),
            pos: 0,
            current_char: None,
            whitespace: HashSet::new(),
            esc: HashSet::new(),
            unsafe_code_point: HashSet::new(),
        };

        lexer.current_char = input.chars().nth(lexer.pos);

        lexer.whitespace = set!['\u{0020}', '\u{000A}', '\u{000D}', '\u{0009}'];
        lexer.esc = set!['"', '\\', '/', 'b', 'f', 'n', 'r', 't', 'u'];
        lexer.unsafe_code_point = set!['"', '\\'];

        return lexer
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.input.chars().nth(self.pos) {
            if ch.is_whitespace() {
                self.advance()
            } else {
                break
            }
        }
    }

    fn number(&mut self) {
        let mut result = String::new();
        if self.current_char.unwrap() == '-' {
            result.push('-');
            self.advance();
        }

        while let Some(c) = self.current_char {
            if c.is_digit(10) || c == '.' {
                result.push(c);
                self.advance();
            } else {
                break
            }
        }
        if result.contains('.') {
            match result.parse::<f32>() {
                Ok(_) => return,
                Err(e) => panic!("{}", e)
            }
        } else {
            match result.parse::<i32>() {
                Ok(_) => return,
                Err(e) => panic!("{}", e)
            }
        }
    }

    fn peek(&self) -> Option<char> {
        return self.input.chars().nth(self.pos + 1);
    }

    fn hex_num(&mut self) {
        for _ in 0..3 {
            match self.current_char {
                Some(c) if !c.is_digit(16) => {
                    panic!("Invalid hex value")
                },
                None => panic!("Unexpected end of string"),
                _ => { self.advance() }
            }
        }
    }

    fn string(&mut self) -> String {
        let mut result = String::new();
        while let Some(c) = self.current_char {
            if c == '\\' {
                match self.peek() {
                    None => panic!("Invalid end of string at pos {}", self.pos),
                    Some(p) => {
                        if !self.esc.contains(&p) {
                            panic!("Invalid escaped character : {} at pos {}", p, self.pos)
                        } else if p == 'u' {
                            self.advance();
                            self.advance();
                            self.hex_num();
                        }
                    }
                }
            } else if self.unsafe_code_point.contains(&c) || c.is_control() {
                return result;
            }
            result.push(c);
            self.advance();
        }
        panic!("Invalid end of string at pos {}", self.pos)
    }

    fn advance(&mut self) {
        self.pos += 1;
        self.current_char = self.input.chars().nth(self.pos);
        // println!("{:?}", self.current_char)
    }

    fn string_check(&mut self, str_check: &str) {
        for i in 0..str_check.len() {
            match self.current_char {
                None => panic!("Invalid string at pos {}", self.pos),
                Some(c) if c != str_check.chars().nth(i).unwrap() => {panic!("Invalid string at pos {}", self.pos)},
                _ => {}
            }
            self.advance()
        }
    }

    fn ttrue(&mut self) {
        self.string_check("true")
    }

    fn tfalse(&mut self) {
        self.string_check("false")
    }

    fn tnull(&mut self) {
        self.string_check("null")
    }

    pub fn get_next_token(&mut self) -> Token {
        while let Some(c) = self.current_char {
            if self.whitespace.contains(&c) {
                self.skip_whitespace();
            } else if c.is_digit(10) || c == '-' {
                self.number();
                return Token {ttype: TokenType::NUMBER}
            } else if c == '{' {
                self.advance();
                return Token {ttype: TokenType::OPBR }
            } else if c == '}' {
                self.advance();
                return Token {ttype: TokenType::CLBR }
            } else if c == '[' {
                self.advance();
                return Token {ttype: TokenType::OPSQ }
            } else if c == ']' {
                self.advance();
                return Token {ttype: TokenType::CLSQ }
            } else if c == '"' {
                self.advance();
                self.string();
                match self.current_char {
                    Some(c) if c == '"' => {
                        self.advance();
                        return Token{ ttype: TokenType::STRING }
                    }
                    _ => panic!("Invalid string terminator")
                }
            } else if c == 't' {
                self.ttrue();
                return Token { ttype: TokenType::TRUE }
            } else if c == 'f' {
                self.tfalse();
                return Token { ttype: TokenType::FALSE }
            } else if c == 'n' {
                self.tnull();
                return Token { ttype: TokenType::NULL }
            } else if c == ':' {
                self.advance();
                return Token {ttype: TokenType::COLON}
            } else if c == ',' {
                self.advance();
                return Token { ttype: TokenType::COMMA }
            }
        }
        return Token {
            ttype: TokenType::EOF
        }
    }
}
