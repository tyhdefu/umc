pub mod ast;

use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar); // synthesized by LALRPOP

#[cfg(test)]
mod grammar_tests;

fn main() {
    println!("Hello, world!");
}
