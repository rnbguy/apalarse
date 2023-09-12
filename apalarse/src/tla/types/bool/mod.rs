mod constant;
mod expr;

pub(crate) use expr::{Eq, InSet, Neq, NotInSet, Quant};

#[cfg(test)]
mod test;

pub use expr::BoolExpr;
