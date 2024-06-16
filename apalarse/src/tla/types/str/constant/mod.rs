mod id;

pub use id::Id;

use crate::tla::{Context, Expr, Str, TlaType};

impl TlaType for Str {
    fn get_type() -> String {
        "Str".into()
    }
}

impl Expr for Str {
    type Output = Self;

    fn tla_expr(&self, _cx: &mut Context) -> String {
        format!("\"{self}\"")
    }

    fn evaluate(&self) -> Self::Output {
        self.clone()
    }
}
