use std::str::Chars;

#[derive(Debug)]
enum Token {
    Integer(u32),
    Plus,
    Eof
}

struct Interpreter<'a> {
    chars: Chars<'a>
}

impl <'a> Interpreter<'a> {
    fn new(text: &'a str) -> Interpreter<'a> {
        Interpreter { chars: text.chars() }
    }

    fn get_next_token(&mut self) -> Result<Token, String> {
        match self.chars.next() {
            None => Ok(Token::Eof),
            Some('+') => Ok(Token::Plus),
            Some(c) =>
                match c.to_digit(10) {
                    Some(d) => Ok(Token::Integer(d)),
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
    let mut inter = Interpreter::new("4+5");
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
