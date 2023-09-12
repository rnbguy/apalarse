mod next;
mod unchanged;
mod variable;

#[cfg(test)]
mod test;

pub(crate) use next::Next;
use serde::{Deserialize, Deserializer};
pub(crate) use unchanged::Unchanged;
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
        Ok(Symbol::Concrete(v))
    }
}

impl<T> Symbol<T>
where
    T: TlaType,
{
    pub fn new(var: Variable<T>) -> Self {
        Symbol::Variable(var)
    }

    pub fn concrete(&self) -> &T {
        match self {
            Symbol::Variable(_) => unimplemented!(),
            Symbol::Concrete(v) => v,
        }
    }

    pub fn variable(&self) -> Variable<T> {
        match self {
            Symbol::Variable(v) => v.clone(),
            Symbol::Concrete(_) => unimplemented!(),
        }
    }
}
