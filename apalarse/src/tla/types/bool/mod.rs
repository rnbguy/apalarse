mod constant;
mod expr;

pub use expr::{Eq, InSet, Neq, NotInSet, Quant};

#[cfg(test)]
mod test;

pub use expr::BoolExpr;
