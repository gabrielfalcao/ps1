#![allow(unused)]
mod errors;
pub use errors::{Error, Result};
pub mod ast;
pub mod macros;
use std::collections::BTreeMap;

pub use ast::{Token, Variable};

pub mod color;
pub mod config;
pub mod resolve;

use std::collections::VecDeque;

pub use config::{Config, VcsConfig};
use iocore::Path;

#[derive(Parser, Debug, Clone)]
#[grammar = "src/grammar.pest"]
pub struct Definition;
use pest::Parser;
use pest_derive::Parser;

pub fn parse_tokens(input: &str) -> Result<Vec<Token>> {
    let mut pairs = match Definition::parse(Rule::ps1, input)
        .map_err(|e| Error::ParseError(e.to_string()))
    {
        Ok(pairs) => pairs,
        Err(e) => {
            eprintln!("warning: {}", &e);
            return Ok(Token::default_vec());
        },
    };
    let ps1 = pairs.next().unwrap();
    // eprintln!("\n\r\x1b[1;48;5;178m\x1b[1;38;5;16m{}WARN{}\x1b[0m", " ".repeat(40), " ".repeat(40));
    // dbg!(&pairs);
    // eprintln!("\r\x1b[1;48;5;178m\x1b[1;38;5;16m{}WARN{}\x1b[0m", " ".repeat(40), " ".repeat(40));

    let tokens = Token::from_pair(ps1)?;
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use k9::assert_equal;

    use crate::{parse_tokens, Result, Token, Variable};

}
