mod next;
mod unchanged;
mod variable;

#[cfg(test)]
mod test;

pub use next::Next;
use serde::{Deserialize, Deserializer};
pub use unchanged::Unchanged;
pub use variable::Variable;

use crate::tla::TlaType;

#[derive(Clone, Debug)]
pub enum Symbol<T>
where
    T: TlaType,
{
    Variable(Variable<T>),
    Concrete(T),
}

impl<'a, T> Deserialize<'a> for Symbol<T>
where
    T: TlaType + Deserialize<'a>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'a>,
    {
        let v = T::deserialize(deserializer)?;
        Ok(Self::Concrete(v))
    }
}

impl<T> Symbol<T>
where
    T: TlaType,
{
    #[must_use]
    pub const fn new(var: Variable<T>) -> Self {
        Self::Variable(var)
    }

    pub fn concrete(&self) -> &T {
        match self {
            Self::Variable(_) => unimplemented!(),
            Self::Concrete(v) => v,
        }
    }

    pub fn variable(&self) -> Variable<T> {
        match self {
            Self::Variable(v) => v.clone(),
            Self::Concrete(_) => unimplemented!(),
        }
    }
}
