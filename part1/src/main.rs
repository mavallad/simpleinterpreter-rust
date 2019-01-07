use std::str::Chars;
use std::iter::Peekable;
use std::io::{self, Write};

#[derive(Debug)]
enum Operation {
    Sum,
    Difference,
    Product,
    Division
}

impl Operation {
    fn calculate(&self, val1: u32, val2: u32) -> u32 {
        match *self {
            Operation::Sum => val1 + val2,
            Operation::Difference => val1 - val2,
            Operation::Product => val1 * val2,
            Operation::Division => val1 / val2
        }
    }
}

#[derive(Debug)]
enum Token {
    Integer(u32),
    Op(Operation),
    Eof
}

struct Interpreter<'a> {
    chars: Peekable<Chars<'a>>
}

impl <'a> Interpreter<'a> {
    fn new(text: &'a str) -> Interpreter<'a> {
        Interpreter { chars: text.chars().peekable() }
    }

    fn get_next_token(&mut self) -> Result<Token, String> {
        self.skip_spaces();
        match self.chars.next() {
            None => Ok(Token::Eof),
            Some('+') => Ok(Token::Op(Operation::Sum)),
            Some('-') => Ok(Token::Op(Operation::Difference)),
            Some('*') => Ok(Token::Op(Operation::Product)),
            Some('/') => Ok(Token::Op(Operation::Division)),
            Some(c) =>
                match c.to_digit(10) {
                    Some(d) => {
                        let val = self.extract_complete_integer(d);
                        Ok(Token::Integer(val))
                    },
                    None => Err("Invalid token".to_string())
                }
        }
    }

    fn skip_spaces(&mut self) {
        while let Some(' ') = self.chars.peek() {
            self.chars.next();
        }
    }

    fn extract_complete_integer(&mut self, first_digit: u32) -> u32 {
        let mut val = first_digit;
        let mut still_integer = true;
        while still_integer {
            match self.chars.peek() {
                Some(n) if n.is_digit(10) => {
                    if let Some(v) = n.to_digit(10) {
                        self.chars.next();
                        val = val * 10 + v;
                    }
                },
                _ => still_integer = false
            }
        }
        val
    }

    fn eat_integer(&mut self) -> Result<u32, String> {
        let curr = self.get_next_token()?;
        if let Token::Integer(v) = curr {
            Ok(v)
        } else {
            println!("token: {:?}", curr);
            Err("Unexpected token".to_string())
        }
    }

    fn eat_operation(&mut self) -> Result<Operation, String> {
        let curr = self.get_next_token()?;
        if let Token::Op(operation) = curr {
            Ok(operation)
        } else {
            println!("token: {:?}", curr);
            Err("Unexpected token".to_string())
        }
    }

    fn eat_eof(&mut self) -> Result<(), String> {
        let curr = self.get_next_token()?;
        if let Token::Eof = curr {
            Ok(())
        } else {
            println!("token: {:?}", curr);
            Err("Unexpected token".to_string())
        }
    }

    fn expr(&mut self) -> Result<u32, String> {
        let left = self.eat_integer()?;
        let operation = self.eat_operation()?;
        let right = self.eat_integer()?;
        let _ = self.eat_eof()?;
        Ok(operation.calculate(left, right))
    }
}

fn read_input() -> Option<String> {
    print!("input> ");
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    if input.is_empty() {
        None
    } else {
        Some(input.trim().to_string())
    }
}

fn main() {
    loop {
        match read_input() {
            Some(str) => {
                let mut inter = Interpreter::new(&str);
                match inter.expr() {
                    Ok(result) => println!("RESULT: {}", result),
                    Err(msg) => eprintln!("ERROR: {}", msg)
                }
            },
            None => break
        }
    }
}
