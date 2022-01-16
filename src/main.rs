mod lexer;

use std::fs;
use crate::lexer::{Lexer, Token, TokenType};


struct Parser {
    lexer: Lexer,
    current_token: lexer::Token
}

impl Parser {

    fn new(lexer: Lexer) -> Parser {
        let mut p = Parser {
            lexer,
            current_token: Token { ttype: TokenType::NONE }
        };
        p.current_token = p.lexer.get_next_token();
        return p
    }

    fn eat(&mut self, lookahead: TokenType) {
        if self.current_token.ttype == lookahead {
            self.current_token = self.lexer.get_next_token();
        } else {
            panic!("Unexpected token at position {}\nExpected {:?} but got {:?}", self.lexer.pos(), lookahead, self.current_token.ttype);
        }
    }

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

    fn array(&mut self) {
        self.eat(TokenType::OPSQ);
        self.value();
        while self.current_token.ttype != TokenType::CLSQ {
            self.eat(TokenType::COMMA);
            self.value();
        }
        self.eat(TokenType::CLSQ);
    }

    fn pair(&mut self) {
        self.eat(TokenType::STRING);
        self.eat(TokenType::COLON);
        self.value();
    }

    fn object(&mut self) {
        self.eat(TokenType::OPBR);
        self.pair();

        while self.current_token.ttype != TokenType::CLBR {
            self.eat(TokenType::COMMA);
            self.pair();
        }
        self.eat(TokenType::CLBR)
    }

    fn parse(&mut self) {
        if self.current_token.ttype == TokenType::OPSQ {
            self.array();
        } else if self.current_token.ttype == TokenType::OPBR {
            self.object()
        }
    }
}


fn load_input(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}

fn main() {
    let path = std::env::args().nth(1).expect("No path given");
    let input = load_input(&path);
    let lexer = Lexer::new(&input);
    let mut parser = Parser::new(lexer);
    parser.parse();
    println!("Input json is valid")
}
