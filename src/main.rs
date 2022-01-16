mod lexer;

// Imports
// Importing the filesystem from standard
use std::fs;
// Importing locally from lexer.rs
use crate::lexer::{Lexer, Token, TokenType};

// Define the Validator struct
struct Validator {
    lexer: Lexer,
    current_token: lexer::Token
}

// Impl block = Namespace tied to the Validator struct
impl Validator {

    // Build a new parser
    fn new(lexer: Lexer) -> Validator {
        let mut p = Validator {
            lexer,
            current_token: Token { ttype: TokenType::NONE }
        };
        // Get the first token from our lexer
        p.current_token = p.lexer.get_next_token();
        return p
    }

    // If the current token matches the expected type,
    // consume it and get the next token from the lexer
    fn eat(&mut self, lookahead: TokenType) {
        if self.current_token.ttype == lookahead {
            self.current_token = self.lexer.get_next_token();
        } else {
            panic!("Unexpected token at position {}\nExpected {:?} but got {:?}", self.lexer.pos(), lookahead, self.current_token.ttype);
        }
    }

    // Value function consumes the possible values for JSON, based on the current token
    // Value is anything that can be found in a json pair (i.e. {"key": value})
    // or as the member of an array (i.e. [value, value, ...])
    // Possible values are : string, number, object, array, true, false, null
    fn value(&mut self) {
        match self.current_token.ttype {
            TokenType::OPBR => { self.object() },
            TokenType::OPSQ => { self.array() },
            TokenType::TRUE => { self.eat(TokenType::TRUE) },
            TokenType::FALSE => { self.eat(TokenType::FALSE) },
            TokenType::NUMBER => { self.eat(TokenType::NUMBER) },
            TokenType::NULL => { self.eat(TokenType::NULL) },
            TokenType::STRING => { self.eat(TokenType::STRING) },
            _ => { panic!("Invalid Value") }
        }
    }

    // A function used to consume an array in a json file (i.e. [value, value])
    fn array(&mut self) {
        // Consume the opening square bracket
        self.eat(TokenType::OPSQ);

        // Consume values as long as they exist
        while self.current_token.ttype != TokenType::CLSQ {
            self.value();

            // If there is another value, the current token should be a comma
            // Consume it and keep going, otherwise break
            if self.current_token.ttype == TokenType::COMMA {
                self.eat(TokenType::COMMA);
            } else {
                break;
            }
        }
        // Consume the final square bracket of the array
        self.eat(TokenType::CLSQ);
    }

    // A function used to consume a pair in a json file (i.e. "key": value)
    fn pair(&mut self) {
        // Consume the key
        self.eat(TokenType::STRING);
        // Consume the colon
        self.eat(TokenType::COLON);
        // Consume the value
        self.value();
    }

    // A function used to consume an array in a json file (i.e. {"key1": value1, "key2": value2, ...})
    fn object(&mut self) {
        // Consume the opening bracket
        self.eat(TokenType::OPBR);

        // Consume the key value pairs
        while self.current_token.ttype != TokenType::CLBR {
            self.pair();

            // If there is another key value pair, the current token should be a comma
            // Consume it and keep going, otherwise break
            if self.current_token.ttype == TokenType::COMMA {
                self.eat(TokenType::COMMA);
            } else {
                break;
            }
        }

        // Consume the closing bracket
        self.eat(TokenType::CLBR)
    }

    // The main function of validator
    fn validate(&mut self) {
        // If the json object starts with a '[' validate it as an array
        if self.current_token.ttype == TokenType::OPSQ {
            self.array();
        // If the json object starts with a '{' validate it as an object
        } else if self.current_token.ttype == TokenType::OPBR {
            self.object()
        }
    }
}

// A function to load a file into a String
fn load_input(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

fn main() {
    // Parse the first argument
    // Crash if it doesnt exist
    let path = std::env::args().nth(1).expect("No path given");
    println!("{}", path);
    // Load the file
    let input = load_input(&path);

    // Create a lexer
    let lexer = Lexer::new(&input);

    // Create a validator
    let mut validator = Validator::new(lexer);

    // Validate the file
    validator.validate();
    println!("Input json is valid")
}
