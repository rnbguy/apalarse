use crate::tla::{Context, Expr, Map, TlaType};

impl<K, V> TlaType for Map<K, V>
where
    K: TlaType,
    V: TlaType,
{
    fn get_type() -> String {
        format!("({} -> {})", K::get_type(), V::get_type())
    }
}

impl<K, V> Expr for Map<K, V>
where
    K: Expr,
    V: Expr,
{
    type Output = Map<K::Output, V::Output>;

    fn tla_expr(&self, cx: &mut Context) -> String {
        cx.add_module("Apalache");
        let s: Vec<_> = self
            .inner
            .iter()
            .map(|(k, v)| format!("<<{}, {}>>", k.tla_expr(cx), v.tla_expr(cx)))
            .collect();
        format!("SetAsFun({{{}}})", s.join(","))
    }

    fn evaluate(&self) -> Self::Output {
        todo!()
        // self.iter().map(|(k, v)| (k.evaluate(), v.evaluate())).collect()
    }
}
