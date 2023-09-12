use crate::tla::{Bool, Context, Expr, TlaType};

impl TlaType for Bool {
    fn get_type() -> String {
        "Bool".into()
    }
}

impl Expr for Bool {
    type Output = Self;
    fn tla_expr(&self, _cx: &mut Context) -> String {
        match self {
            true => "TRUE".into(),
            false => "FALSE".into(),
        }
    }
    fn evaluate(&self) -> Self::Output {
        *self
    }
}
