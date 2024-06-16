use core::fmt::{Debug, Write};
use std::collections::{HashMap, HashSet};

use crate::tla::{Expr, TlaType, Variable};
use crate::utils::AResult;

#[derive(Default, Debug)]
pub struct Context {
    pub global_variables: HashMap<u64, (String, String)>,
    pub modules: HashSet<String>,
    pub last_variable_id: u64,
}

impl Context {
    pub fn add_global_variable<T>(&mut self, var: &Variable<T>)
    where
        T: TlaType,
    {
        assert!(var.is_global());
        self.global_variables
            .insert(var.id(), (var.name(), T::get_type()));
    }

    pub fn add_module(&mut self, module: &str) {
        self.modules.insert(module.into());
    }

    pub fn tla_static_operator<T>(&mut self, name: &str, expr: &T) -> String
    where
        T: Expr,
    {
        format!(
            "(* @type: () => {}; *) {} == {}",
            T::Output::get_type(),
            name,
            expr.tla_expr(self)
        )
    }

    pub fn tla_variables(&self) -> AResult<String> {
        if self.global_variables.is_empty() {
            return Ok(String::new());
        }
        let mut rt = String::from("VARIABLES");
        for (v, t) in self.global_variables.values() {
            write!(rt, " (* @type: {t}; *) {v},")?;
        }
        rt.pop();
        Ok(rt)
    }

    pub fn tla_modules(&self) -> AResult<String> {
        if self.modules.is_empty() {
            return Ok(String::new());
        }
        let mut rt = String::from("EXTENDS");
        for m in &self.modules {
            write!(rt, " {m},")?;
        }
        rt.pop();
        Ok(rt)
    }

    pub fn enter_scope(&mut self) {
        self.last_variable_id += 1;
    }

    pub fn exit_scope(&mut self) {
        self.last_variable_id -= 1;
    }

    pub fn var<T>(&mut self) -> Variable<T>
    where
        T: TlaType,
    {
        self.last_variable_id += 1;
        Variable::global(self.last_variable_id - 1)
    }

    pub fn var_with_name<T>(&mut self, name: &str) -> Variable<T>
    where
        T: TlaType,
    {
        self.last_variable_id += 1;
        Variable::global_with_name(self.last_variable_id - 1, name)
    }

    pub fn bound_var<T>(&mut self) -> Variable<T>
    where
        T: TlaType,
    {
        self.last_variable_id += 1;
        Variable::bound(self.last_variable_id - 1)
    }

    pub fn map_var<T>(&mut self) -> Variable<T>
    where
        T: TlaType,
    {
        self.last_variable_id += 1;
        Variable::map(self.last_variable_id - 1)
    }
}
