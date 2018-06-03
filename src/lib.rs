#[macro_use]
extern crate failure;
extern crate fnv;
#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate gc_arena;

pub mod lexer;
pub mod parser;
pub mod string;
pub mod table;
pub mod value;

#[cfg(test)]
mod tests;
