pub mod cardinality;
pub mod cross;
pub mod difference;
pub mod filter;
pub mod intersect;
pub mod mapped_set;
pub mod set_map;
pub mod singleton;
pub mod subset_eq;
pub mod union;

use cardinality::Cardinality;
use cross::Cross;
use difference::Difference;
use filter::Filter;
use intersect::Intersect;
use mapped_set::MappedSet;
use set_map::SetMap;
use singleton::Singleton;
use subset_eq::SubsetEq;
use union::Union;

use crate::tla::{BoolExpr, Expr, InSet, NotInSet, Quant, Set, TlaType, Variable};

pub trait SetExpr: Expr {
    type ElemType: TlaType;

    fn tla_contains<E>(&self, elem: E) -> InSet<E, Self>
    where
        Self: Clone + Expr<Output = Set<Self::ElemType>>,
        E: Expr<Output = Self::ElemType>,
    {
        InSet::new(elem, self.clone())
    }

    fn tla_not_contains<E>(&self, elem: E) -> NotInSet<E, Self>
    where
        Self: Clone + Expr<Output = Set<Self::ElemType>>,
        E: Expr<Output = Self::ElemType>,
    {
        NotInSet::new(elem, self.clone())
    }

    fn tla_insert<E>(&self, elem: E) -> Union<Self, Singleton<E>>
    where
        Self: Clone,
        E: Expr<Output = Self::ElemType>,
    {
        self.tla_union(Singleton::new(elem))
    }

    fn tla_remove<E>(&self, elem: E) -> Difference<Self, Singleton<E>>
    where
        Self: Clone,
        E: Expr<Output = Self::ElemType>,
    {
        self.tla_difference(Singleton::new(elem))
    }

    fn tla_union<T>(&self, vs: T) -> Union<Self, T>
    where
        Self: Clone,
        T: SetExpr<ElemType = Self::ElemType>,
    {
        Union::new(self.clone(), vs)
    }

    fn tla_difference<T>(&self, vs: T) -> Difference<Self, T>
    where
        Self: Clone,
        T: SetExpr<ElemType = Self::ElemType>,
    {
        Difference::new(self.clone(), vs)
    }

    fn tla_intersect<T>(&self, vs: T) -> Intersect<Self, T>
    where
        Self: Clone,
        T: SetExpr<ElemType = Self::ElemType>,
    {
        Intersect::new(self.clone(), vs)
    }

    fn map_to_set<P, U>(&self, p: P) -> MappedSet<Self, P, U>
    where
        Self: Clone,
        P: Fn(Variable<Self::ElemType>) -> U,
        U: Expr,
    {
        MappedSet::new(self.clone(), p)
    }

    fn filter<P, U>(&self, p: P) -> Filter<Self, P, U>
    where
        Self: Clone,
        P: Fn(Variable<Self::ElemType>) -> U,
        U: BoolExpr,
    {
        Filter::new(self.clone(), p)
    }

    fn create_map<P, U>(&self, p: P) -> SetMap<Self, P, U>
    where
        Self: Clone,
        P: Fn(Variable<Self::ElemType>) -> U,
        U: Expr,
    {
        SetMap::new(self.clone(), p)
    }

    fn for_all<P, U>(&self, p: P) -> Quant<Self, P, U>
    where
        Self: Clone,
        P: Fn(Variable<Self::ElemType>) -> U,
        U: BoolExpr,
    {
        Quant::for_all(self.clone(), p)
    }
    fn exists<P, U>(&self, p: P) -> Quant<Self, P, U>
    where
        Self: Clone,
        P: Fn(Variable<Self::ElemType>) -> U,
        U: BoolExpr,
    {
        Quant::exists(self.clone(), p)
    }

    fn cross<T, U, V>(&self, other: U) -> Cross<Self, U>
    where
        Self: Clone + SetExpr<ElemType = T>,
        U: SetExpr<ElemType = V>,
    {
        Cross::new(self.clone(), other)
    }

    fn subseteq<U, T>(&self, other: U) -> SubsetEq<Self, U>
    where
        Self: Clone + SetExpr<ElemType = T>,
        U: SetExpr<ElemType = T>,
    {
        SubsetEq::new(self.clone(), other)
    }

    fn cardinality(&self) -> Cardinality<Self>
    where
        Self: Clone,
    {
        Cardinality::new(self.clone())
    }
}

impl<U, E> SetExpr for U
where
    U: Expr<Output = Set<E>>,
    E: TlaType,
{
    type ElemType = E;
}
