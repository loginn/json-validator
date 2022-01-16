use std::collections::HashSet;

// Macro to create a set
// Shamelessly stolen from stack overflow
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

// All the valid tokens types in JSON
#[derive(PartialEq, Debug)]
pub enum TokenType {
    OPBR,   // {
    CLBR,   // }
    OPSQ,   // [
    CLSQ,   // ]
    COMMA,  // ,
    STRING, // "*"
    NUMBER, // 1, 1.0, 1e+1, -1, ...
    TRUE,   // true
    FALSE,  // false
    NULL,   // null
    COLON,  // :
    EOF,    // \0
    NONE    // Special type used for a lack of token when initializing
}

// Token structure
#[derive(Debug)]
pub struct Token {
    pub ttype: TokenType,
}

// Lexer structure
pub struct Lexer {
    input: String,
    pos: usize,
    current_char: Option<char>,
    whitespace: HashSet<char>,
    esc: HashSet<char>,
    unsafe_code_point: HashSet<char>,
}

// Impl block for our lexer
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

        // Hashsets for specific values in strings
        // Hashset for whitespace
        lexer.whitespace = set![' ', '\n', '\r', '\t'];
        // Hashset for escapable characters
        lexer.esc = set!['"', '\\', '/', 'b', 'f', 'n', 'r', 't', 'u'];
        // Hashset for unsafe code points
        lexer.unsafe_code_point = set!['"', '\\'];

        return lexer
    }

    // Getter for pos
    pub fn pos(&self) -> usize {
        return self.pos
    }

    // A function to skip whitespace in the input
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.input.chars().nth(self.pos) {
            if self.whitespace.contains(&c) {
                self.advance()
            } else {
                break
            }
        }
    }

    // A function to loop over digits
    fn loop_digits(&mut self, exponent: bool) {
        let mut invalid_point = exponent;
        while let Some(c) = self.current_char {
            if c == '.' && invalid_point {
                panic!("Unexpected floating point in number")
            } else if c == '.' && !invalid_point {
                invalid_point = true;
                self.advance();
            } else if c.is_digit(10)  {
                self.advance();
            } else {
                break
            }
        }
    }

    // A function to handle a number token (1, -1, 1.0, -1.0, 1e1, 1e+1, 1e-1, ...)
    fn number(&mut self) {
        //minus
        if self.current_char.unwrap() == '-' {
            self.advance();
        }

        //digits
        self.loop_digits(false);
        //exponents
        match self.current_char {
            Some(c) if c == 'e' || c == 'E' => {
                self.advance();
                match self.current_char {
                    None => {panic!()}
                    Some(c) => {
                        if c == '+' || c == '-' {
                            self.advance();
                        }
                        self.loop_digits(true);
                    }
                }
            }
            Some(_) => {}
            None => {panic!()}
        }
    }

    // Look at the next character without consuming it
    fn peek(&self) -> Option<char> {
        return self.input.chars().nth(self.pos + 1);
    }

    // Handle hex number for \u strings
    fn hex_num(&mut self) {
        // \u strings must have the form \u HEX HEX HEX HEX meaning we can loop over 0..3
        for _ in 0..3 {
            match self.current_char {
                // Check if the character is a digit in base 16
                Some(c) if !c.is_digit(16) => {
                    panic!("Invalid hex value")
                },
                None => panic!("Unexpected end of string"),
                _ => { self.advance() }
            }
        }
    }

    // A function to handle string tokens (i.e. "*+")
    fn string(&mut self) {
        while let Some(c) = self.current_char {
            // check if the next character is escaped and only allow those not in self.esc
            if c == '\\' {
                match self.peek() {
                    None => panic!("Invalid end of string at pos {}", self.pos),
                    Some(p) => {
                        if p == 'u' {
                            // if the next char is unicode advance twice to skip the '\' and 'u'
                            self.advance();
                            self.advance();
                            // parse the hex num
                            self.hex_num();
                        } else if !self.esc.contains(&p) {
                            // If the value isnt in our accepted escapable characters, crash
                            panic!("Invalid escaped character : {} at pos {}", p, self.pos)
                        } else {
                            // If we find a valid escaped character, advance to the next one
                            self.advance();
                        }
                    }
                }
            } else if self.unsafe_code_point.contains(&c) || c.is_control() {
                // if c is not a safe character, return (used if we find the final " otherwise the json isnt valid)
                return
            }
            // loop through characters
            self.advance();
        }
        panic!("Invalid end of string at pos {}", self.pos)
    }

    // consume and return the next char
    fn advance(&mut self) {
        self.pos += 1;
        self.current_char = self.input.chars().nth(self.pos);
    }

    // Check if the lexer's input matches a given string (true, false, null)
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

    // check that the next characters form the token 'true'
    fn ttrue(&mut self) {
        self.string_check("true")
    }

    // check that the next characters form the token 'false'
    fn tfalse(&mut self) {
        self.string_check("false")
    }

    // check that the next characters form the token 'null'
    fn tnull(&mut self) {
        self.string_check("null")
    }

    // get the next token based on the current character
    // If no character is found, we reached the end of the input
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
            } else {
                panic!("Invalid character {:?} at position {}", self.current_char, self.pos)
            }
        }
        // return end of file token
        return Token {
            ttype: TokenType::EOF
        }
    }
}
