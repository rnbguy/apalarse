mod domain;
mod map_get;
mod replace;

use domain::Domain;
use map_get::MapGet;
use replace::Replace;

use crate::tla::{Expr, Map, TlaType, Variable};

pub trait MapExpr: Expr {
    type KeyType: TlaType;
    type ValueType: TlaType;

    fn tla_get<K>(&self, key: K) -> MapGet<Self, K>
    where
        Self: Clone,
        K: Expr<Output = Self::KeyType>,
    {
        MapGet::new(self.clone(), key)
    }

    fn domain(&self) -> Domain<Self>
    where
        Self: Clone,
    {
        Domain::new(self.clone())
    }

    fn replace<K, V, U>(&self, key: K, value: V) -> Replace<Self, K, V, U>
    where
        Self: Clone,
        K: Expr<Output = Self::KeyType>,
        V: Fn(Variable<Self::ValueType>) -> U,
        U: Expr<Output = Self::ValueType>,
    {
        // TODO: value should be function on old value
        Replace::new(self.clone(), key, value)
    }
}

impl<U, K, V> MapExpr for U
where
    K: TlaType,
    V: TlaType,
    U: Expr<Output = Map<K, V>>,
{
    type KeyType = K;
    type ValueType = V;
}
