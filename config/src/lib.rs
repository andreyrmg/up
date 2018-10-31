use std::io::prelude::*;
use std::rc::Rc;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Origin {
    Generic,
    File,
}

#[derive(Debug, PartialEq)]
pub struct ConfigOrigin {
    description: Rc<String>,
    origin_type: Origin,
    line_number: u32,
    end_line_number: u32,
}

impl ConfigOrigin {
    fn with_line_number(&self, line_number: u32) -> ConfigOrigin {
        ConfigOrigin {
            description: Rc::clone(&self.description),
            origin_type: self.origin_type,
            line_number: line_number,
            end_line_number: line_number,
        }
    }
}

impl ConfigOrigin {
    fn new(description: &str) -> ConfigOrigin {
        ConfigOrigin {
            description: Rc::new(description.to_string()),
            origin_type: Origin::Generic,
            line_number: 0u32,
            end_line_number: 0u32,
        }
    }
}

pub trait Config {
    fn getString(&self, path: &str) -> &str;
}

pub struct ConfigImpl;

impl Config for ConfigImpl {
    fn getString(&self, path: &str) -> &str {
        ""
    }
}

pub fn load() -> impl Config {
    ConfigImpl {}
}

#[derive(Debug, PartialEq)]
enum Token {
    Start,
    End,
    Newline(ConfigOrigin),
}

struct TokenIterator<'a> {
    origin: ConfigOrigin,
    input: &'a BufRead,
    line_number: u32,
    token: Option<Token>,
}

impl<'a> Iterator for TokenIterator<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.token.take();
        if result != Some(Token::End) {}
        result
    }
}

fn tokenize(origin: ConfigOrigin, input: &BufRead) -> TokenIterator {
    TokenIterator {
        origin: origin,
        input: input,
        line_number: 1u32,
        token: Some(Token::Start),
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::io;

    fn tokenize_to_vec(s: &[u8]) -> Vec<Token> {
        let reader = io::Cursor::new(s);
        tokenize(ConfigOrigin::new("anonymous Reader"), &reader).collect()
    }

    #[test]
    fn tokenize_empty_string() {
        assert_eq!(vec![Token::Start, Token::End], tokenize_to_vec(b""))
    }

    #[test]
    fn tokenize_newlines() {}

}
