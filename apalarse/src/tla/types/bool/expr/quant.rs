use crate::tla::{Bool, BoolExpr, Context, Expr, SetExpr, Variable};

#[derive(Debug, Clone)]
pub enum QuantEnum {
    ForAll,
    Exists,
}

impl QuantEnum {
    fn char(&self) -> char {
        match self {
            Self::ForAll => 'A',
            Self::Exists => 'E',
        }
    }
}

#[derive(Debug, Clone)]
pub struct Quant<S, P, U>
where
    S: SetExpr,
    P: Fn(Variable<S::ElemType>) -> U,
    U: BoolExpr,
{
    pub _type: QuantEnum,
    pub set: S,
    pub predicate: P,
}

impl<S, P, U> Quant<S, P, U>
where
    S: SetExpr,
    P: Fn(Variable<S::ElemType>) -> U,
    U: BoolExpr,
{
    pub fn for_all(set: S, predicate: P) -> Self {
        Self {
            _type: QuantEnum::ForAll,
            set,
            predicate,
        }
    }

    pub fn exists(set: S, predicate: P) -> Self {
        Self {
            _type: QuantEnum::Exists,
            set,
            predicate,
        }
    }
}

impl<S, P, U> Expr for Quant<S, P, U>
where
    S: SetExpr,
    P: Fn(Variable<S::ElemType>) -> U + Clone,
    U: BoolExpr,
{
    type Output = Bool;

    fn tla_expr(&self, cx: &mut Context) -> String {
        cx.enter_scope();
        let new_var = cx.bound_var();
        let rt = format!(
            "(\\{} {} \\in {}: {})",
            self._type.char(),
            new_var.tla_expr(cx),
            self.set.tla_expr(cx),
            (self.predicate)(new_var.clone()).tla_expr(cx)
        );
        cx.exit_scope();
        rt
    }

    fn evaluate(&self) -> Self::Output {
        todo!()
    }
}
