use core::fmt::Display;
use core::marker::PhantomData;

use serde_json::Value;

use crate::tla::{Context, Expr, Next, TlaType, Unchanged};
use crate::utils::{AContext, AResult};

#[derive(Debug, Clone)]
enum Scope {
    Global,
    Bound,
    Map,
}

#[derive(Debug, Clone)]
pub struct Variable<T>
where
    T: TlaType,
{
    id: u64,
    name: String,
    scope: Scope,
    _phantom: PhantomData<T>,
}

impl<T> Variable<T>
where
    T: TlaType,
{
    #[must_use]
    pub const fn is_global(&self) -> bool {
        matches!(self.scope, Scope::Global)
    }
}

impl<T> Display for Variable<T>
where
    T: TlaType,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl<T> Expr for Variable<T>
where
    T: TlaType,
{
    type Output = T;

    fn tla_expr(&self, cx: &mut Context) -> String {
        if self.is_global() {
            cx.add_global_variable(self);
        }
        self.name()
    }

    fn evaluate(&self) -> Self::Output {
        todo!()
    }
}

impl<T> Variable<T>
where
    T: TlaType,
{
    #[must_use]
    pub fn global(id: u64) -> Self {
        Self {
            id,
            name: format!("global_{id}"),
            scope: Scope::Global,
            _phantom: PhantomData,
        }
    }
    #[must_use]
    pub fn global_with_name(id: u64, name: &str) -> Self {
        Self {
            id,
            name: name.into(),
            scope: Scope::Global,
            _phantom: PhantomData,
        }
    }

    #[must_use]
    pub fn bound(id: u64) -> Self {
        Self {
            id,
            name: format!("bound_{id}"),
            scope: Scope::Bound,
            _phantom: PhantomData,
        }
    }

    #[must_use]
    pub fn map(id: u64) -> Self {
        Self {
            id,
            name: "@".into(),
            scope: Scope::Map,
            _phantom: PhantomData,
        }
    }

    #[must_use]
    pub fn next(&self) -> Next<T> {
        Next::new(self.clone())
    }

    #[must_use]
    pub fn unchanged(&self) -> Unchanged<T> {
        Unchanged::new(self.clone())
    }

    #[must_use]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[must_use]
    pub const fn id(&self) -> u64 {
        self.id
    }

    /// Get the value of the variable in the given state.
    ///
    /// # Errors
    /// If the variable is not found in the state.
    pub fn value(&self, state: &Value) -> AResult<Value> {
        state
            .as_object()
            .and_then(|obj| obj.get(&self.name()))
            .cloned()
            .context("variable not found")
    }
}
