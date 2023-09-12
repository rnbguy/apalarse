use crate::tla::{Context, Expr, Int, Set};

#[derive(Debug, Clone, Copy)]
pub struct Nat;

impl Expr for Nat {
    type Output = Set<Int>;

    fn tla_expr(&self, _cx: &mut Context) -> String {
        "Nat".into()
    }

    fn evaluate(&self) -> Self::Output {
        unimplemented!("can't compute the set of all natural numbers")
    }
}
