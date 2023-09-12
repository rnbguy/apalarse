use crate::tla::{Context, Expr, Int, TlaType};

impl TlaType for Int {
    fn get_type() -> String {
        "Int".into()
    }
}

impl Expr for Int {
    type Output = Self;

    fn tla_expr(&self, _cx: &mut Context) -> String {
        self.to_string()
    }

    fn evaluate(&self) -> Self::Output {
        *self
    }
}
