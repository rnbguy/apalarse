mod context;
mod expr;
mod symbol;
mod types;

#[macro_use]
pub(crate) mod macros;

pub use context::*;
pub use expr::*;
use serde::Deserialize;
pub use symbol::*;
pub use types::bool::*;
pub use types::int::*;
pub use types::list::*;
pub use types::map::*;
pub use types::set::*;
pub use types::str::*;

// pub use itf::ItfBool as Bool;
// pub use itf::ItfInt as Int;
// pub use itf::ItfString as Str;
// pub use itf::ItfSet as Set;
// pub use itf::ItfMap as Map;

#[derive(Debug, Clone, Deserialize)]
pub struct Set<T> {
    #[serde(rename = "#set")]
    inner: Vec<T>,
}

impl<T> From<Vec<T>> for Set<T> {
    fn from(inner: Vec<T>) -> Self {
        Self { inner }
    }
}

impl<T> Set<T> {
    #[must_use]
    pub const fn empty() -> Self {
        Self { inner: vec![] }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Map<K, V> {
    #[serde(rename = "#map")]
    inner: Vec<(K, V)>,
}

impl<K, V> From<Vec<(K, V)>> for Map<K, V> {
    fn from(inner: Vec<(K, V)>) -> Self {
        Self { inner }
    }
}

impl<K, V> Map<K, V> {
    #[must_use]
    pub const fn empty() -> Self {
        Self { inner: vec![] }
    }
}

pub type Bool = bool;
pub type Int = i64;
pub type Str = String;
pub type List<T> = Vec<T>;
