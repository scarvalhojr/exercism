use std::collections::HashMap;
use Element::*;
use Error::*;

pub type Value = i32;
pub type ForthResult = Result<(), Error>;

#[derive(Clone)]
enum Element {
    Operand(Value),
    Operator(String),
}

pub struct Forth {
    stack: Vec<Value>,
    words: HashMap<String, Vec<Element>>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            stack: Vec::new(),
            words: HashMap::new(),
        }
    }

    pub fn stack(&self) -> Vec<Value> {
        self.stack.clone()
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        let input_lower = input.to_lowercase();
        let mut tokens = input_lower.split_whitespace();

        while let Some(token) = tokens.next() {
            if token == ":" {
                // Define new word
                let word = tokens.next().ok_or(InvalidWord)?;
                if word.parse::<Value>().is_ok() {
                    // Cannot redefine numbers
                    return Err(InvalidWord);
                }
                let mut elements = Vec::new();
                loop {
                    match tokens.next() {
                        Some(";") => {
                            self.words.insert(word.to_string(), elements);
                            break;
                        }
                        Some(token) => elements.append(&mut self.parse(token)),
                        None => return Err(InvalidWord),
                    }
                }
            } else {
                for element in self.parse(token) {
                    self.execute(&element)?;
                }
            }
        }

        Ok(())
    }

    fn parse(&self, token: &str) -> Vec<Element> {
        if let Some(elements) = self.words.get(token) {
            elements.to_vec()
        } else if let Ok(number) = token.parse::<Value>() {
            vec![Operand(number)]
        } else {
            vec![Operator(token.to_string())]
        }
    }

    fn execute(&mut self, elem: &Element) -> ForthResult {
        match elem {
            Operand(value) => {
                // Push value onto the stack
                self.stack.push(*value);
                Ok(())
            }
            Operator(oper) => {
                // Execute built-in operation
                match oper.as_str() {
                    "+" | "-" | "*" | "/" => self.arithmetic(oper),
                    "dup" => self.dup(),
                    "drop" => self.drop(),
                    "swap" => self.swap(),
                    "over" => self.over(),
                    _ => Err(UnknownWord),
                }
            }
        }
    }

    fn arithmetic(&mut self, oper: &str) -> ForthResult {
        let val2 = self.pop_operand()?;
        let val1 = self.pop_operand()?;
        let res = match oper {
            "+" => Ok(val1 + val2),
            "-" => Ok(val1 - val2),
            "*" => Ok(val1 * val2),
            "/" if val2 == 0 => Err(DivisionByZero),
            "/" => Ok(val1 / val2),
            _ => panic!("Unknown arithmetic operator"),
        }?;
        self.stack.push(res);
        Ok(())
    }

    fn dup(&mut self) -> ForthResult {
        let val = self.peek_operand()?;
        self.stack.push(val);
        Ok(())
    }

    fn drop(&mut self) -> ForthResult {
        self.pop_operand()?;
        Ok(())
    }

    fn swap(&mut self) -> ForthResult {
        let val2 = self.pop_operand()?;
        let val1 = self.pop_operand()?;
        self.stack.push(val2);
        self.stack.push(val1);
        Ok(())
    }

    fn over(&mut self) -> ForthResult {
        let val2 = self.pop_operand()?;
        let val1 = self.peek_operand()?;
        self.stack.push(val2);
        self.stack.push(val1);
        Ok(())
    }

    fn pop_operand(&mut self) -> Result<Value, Error> {
        self.stack.pop().ok_or(StackUnderflow)
    }

    fn peek_operand(&self) -> Result<Value, Error> {
        self.stack.last().ok_or(StackUnderflow).map(|v| *v)
    }
}
