macro_rules! unary_expr_struct {
    ($name: ident, $input_trait: ident, $output_type: ident, $char: expr, $module: expr) => {
        #[derive(Debug, Clone)]
        pub struct $name<T>
        where
            T: $input_trait,
        {
            pub inner: T,
        }

        impl<T> $name<T>
        where
            T: $input_trait,
        {
            pub const fn new(inner: T) -> $name<T> {
                $name { inner }
            }
        }

        impl<T> Expr for $name<T>
        where
            T: $input_trait,
        {
            type Output = $output_type;

            fn tla_expr(&self, cx: &mut Context) -> String {
                if let Some(m) = $module {
                    cx.add_module(m);
                }
                format!("({}{})", $char, self.inner.tla_expr(cx))
            }

            fn evaluate(&self) -> Self::Output {
                todo!()
            }
        }
    };
}

macro_rules! unary_expr_func {
    ($func_name: ident, $struct_name: ident) => {
        fn $func_name(&self) -> $struct_name<Self>
        where
            Self: Clone,
        {
            $struct_name::new(self.clone())
        }
    };
}

macro_rules! generic_binary_expr_struct {
    ($name: ident, $output_type: ident, $char: expr, $module: expr) => {
        #[derive(Debug, Clone)]
        pub struct $name<Lhs, Rhs>
        where
            Lhs: Expr,
            Rhs: Expr<Output = Lhs::Output>,
        {
            pub lhs: Lhs,
            pub rhs: Rhs,
        }

        impl<Lhs, Rhs> $name<Lhs, Rhs>
        where
            Lhs: Expr,
            Rhs: Expr<Output = Lhs::Output>,
        {
            pub const fn new(lhs: Lhs, rhs: Rhs) -> $name<Lhs, Rhs> {
                $name { lhs, rhs }
            }
        }

        impl<Lhs, Rhs> Expr for $name<Lhs, Rhs>
        where
            Lhs: Expr,
            Rhs: Expr<Output = Lhs::Output>,
        {
            type Output = $output_type;

            fn tla_expr(&self, cx: &mut Context) -> String {
                if let Some(m) = $module {
                    cx.add_module(m);
                }
                format!(
                    "({} {} {})",
                    self.lhs.tla_expr(cx),
                    $char,
                    self.rhs.tla_expr(cx)
                )
            }

            fn evaluate(&self) -> Self::Output {
                todo!()
            }
        }
    };
}

macro_rules! binary_expr_struct {
    ($name: ident, $input_trait: ident, $output_type: ident, $char: expr, $module: expr) => {
        #[derive(Debug, Clone)]
        pub struct $name<Lhs, Rhs>
        where
            Lhs: $input_trait,
            Rhs: $input_trait,
        {
            pub lhs: Lhs,
            pub rhs: Rhs,
        }

        impl<Lhs, Rhs> $name<Lhs, Rhs>
        where
            Lhs: $input_trait,
            Rhs: $input_trait,
        {
            pub const fn new(lhs: Lhs, rhs: Rhs) -> $name<Lhs, Rhs> {
                $name { lhs, rhs }
            }
        }

        impl<Lhs, Rhs> Expr for $name<Lhs, Rhs>
        where
            Lhs: $input_trait,
            Rhs: $input_trait,
        {
            type Output = $output_type;

            fn tla_expr(&self, cx: &mut Context) -> String {
                if let Some(m) = $module {
                    cx.add_module(m);
                }
                format!(
                    "({} {} {})",
                    self.lhs.tla_expr(cx),
                    $char,
                    self.rhs.tla_expr(cx)
                )
            }

            fn evaluate(&self) -> Self::Output {
                todo!()
            }
        }
    };
}

macro_rules! binary_expr_func {
    ($func_name: ident, $struct_name: ident) => {
        fn $func_name<Rhs>(&self, other: Rhs) -> $struct_name<Self, Rhs>
        where
            Self: Clone,
            Rhs: Expr<Output = Self::Output>,
        {
            $struct_name::new(self.clone(), other)
        }
    };
}

pub(crate) use {
    binary_expr_func, binary_expr_struct, generic_binary_expr_struct, unary_expr_func,
    unary_expr_struct,
};
