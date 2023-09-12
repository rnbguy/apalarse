mod append;
mod concat;
mod head;
mod list_domain;
mod list_get;
mod replace;
mod size;
mod tail;

pub use append::Append;
pub use concat::Concat;
pub use head::Head;
pub use list_domain::ListDomain;
pub use list_get::ListGet;
pub use replace::Replace;
pub use size::Size;
pub use tail::Tail;

use crate::tla::{Expr, Int, List, TlaType, Variable};

pub trait ListExpr: Expr {
    type ElemType: TlaType;

    fn nth(&self, key: Int) -> ListGet<Self>
    where
        Self: Clone,
    {
        ListGet::new(self.clone(), key)
    }

    fn domain(&self) -> ListDomain<Self>
    where
        Self: Clone,
    {
        ListDomain::new(self.clone())
    }

    fn concat<O>(&self, other: O) -> Concat<Self, O>
    where
        Self: Clone,
        O: ListExpr<ElemType = Self::ElemType>,
    {
        Concat::new(self.clone(), other)
    }

    fn size(&self) -> Size<Self>
    where
        Self: Clone,
    {
        Size::new(self.clone())
    }

    fn append<E>(&self, value: E) -> Append<Self, E>
    where
        Self: Clone,
        E: Expr<Output = Self::ElemType>,
    {
        Append::new(self.clone(), value)
    }

    fn head(&self) -> Head<Self>
    where
        Self: Clone,
    {
        Head::new(self.clone())
    }

    fn tail(&self) -> Tail<Self>
    where
        Self: Clone,
    {
        Tail::new(self.clone())
    }

    fn replace<P, U>(&self, key: Int, value: P) -> Replace<Self, P, U>
    where
        Self: Clone,
        P: Fn(Variable<Self::ElemType>) -> U,
        U: Expr<Output = Self::ElemType>,
    {
        Replace::new(self.clone(), key, value)
    }
}

impl<U, V> ListExpr for U
where
    V: TlaType,
    U: Expr<Output = List<V>>,
{
    type ElemType = V;
}
