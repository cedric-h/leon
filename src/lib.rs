#![feature(trait_alias)]

mod lex;
mod parse;
mod util;
mod error;

use self::lex::Lexeme;

pub use error::Error;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Thing {
    Atom,
    Lexeme(Lexeme),
    Ident,
    Expr,
}

#[derive(Default)]
pub struct Engine;

impl Engine {
    pub fn execute(&mut self, code: &str) -> Result<Value, Vec<Error>> {
        let (tokens, ctx) = lex::lex(code)?;

        println!("--- Tokens ---");
        ctx.print_debug(&tokens);

        let ast = parse::parse(&tokens)?;

        println!("--- Syntax Tree ---");
        ast.print_debug(&ctx);

        unimplemented!()
    }
}

pub enum Value {}
