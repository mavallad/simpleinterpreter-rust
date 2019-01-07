use std::str::Chars;
use std::iter::Peekable;

#[derive(Debug)]
enum Token {
    Integer(u32),
    Plus,
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
        match self.chars.next() {
            None => Ok(Token::Eof),
            Some('+') => Ok(Token::Plus),
            Some(c) =>
                match c.to_digit(10) {
                    Some(d) => {
                        let mut val = d;
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
                        Ok(Token::Integer(val))
                    },
                    None => Err("Invalid token".to_string())
                }
        }
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

    fn eat_operation(&mut self) -> Result<(), String> {
        let curr = self.get_next_token()?;
        if let Token::Plus = curr {
            Ok(())
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
        let _ = self.eat_operation()?;
        let right = self.eat_integer()?;
        let _ = self.eat_eof()?;
        Ok(left + right)
    }
}

fn main() -> Result<(), String> {
    let mut inter = Interpreter::new("10+56");
    // let t1 = inter.get_next_token()?;
    // println!("{:?}", t1);
    // let t2 = inter.get_next_token()?;
    // println!("{:?}", t2);
    // let t3 = inter.get_next_token()?;
    // println!("{:?}", t3);
    // let t4 = inter.get_next_token()?;
    // println!("{:?}", t4);
    let result = inter.expr()?;
    println!("RESULT: {}", result);
    Ok(())
}
