use crate::tla::{Context, Expr, Int, Set};

#[derive(Debug, Clone, Copy)]
pub struct RangeSet {
    start: Int,
    end: Int,
}

impl RangeSet {
    #[must_use]
    pub const fn new(start: i64, end: i64) -> Self {
        Self { start, end }
    }
}

impl Expr for RangeSet {
    type Output = Set<Int>;

    fn tla_expr(&self, _cx: &mut Context) -> String {
        format!("{}..{}", self.start, self.end)
    }

    fn evaluate(&self) -> Self::Output {
        todo!()
    }
}
