use serde::{Deserialize, Serialize};

use crate::tla::{Context, Expr, Str};

#[derive(Debug, Clone)]
pub struct Id<T>
where
    T: Serialize + for<'a> Deserialize<'a> + Clone,
{
    inner: T,
}

impl<T> Id<T>
where
    T: Serialize + for<'a> Deserialize<'a> + Clone,
{
    fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> From<T> for Id<T>
where
    T: Serialize + for<'a> Deserialize<'a> + Clone,
{
    fn from(inner: T) -> Self {
        Self::new(inner)
    }
}

impl<T> Expr for Id<T>
where
    T: Serialize + for<'a> Deserialize<'a> + Clone,
{
    type Output = Str;

    fn tla_expr(&self, _cx: &mut Context) -> String {
        serde_json::to_string(&self.inner).unwrap()
    }

    fn evaluate(&self) -> Self::Output {
        serde_json::to_string(&self.inner)
            .unwrap()
            .trim_matches('"')
            .to_string()
    }
}
