/// THIS MODULE IS A WORK IN PROGRESS
/// DO NOT USE IT
mod ast;
mod clause;
mod expression;
mod operators;
mod statement;
mod utils;

pub use ast::*;
pub use statement::parse_sql_statement;

use self::tokenizer::SqlToken;

pub(crate) mod tokenizer;

type SqlTokenStream<'a> = &'a [SqlToken<'a>];
