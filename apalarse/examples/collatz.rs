use apalarse::runner;
use apalarse::tla::{BoolExpr, Expr, Int, IntExpr, Nat};
use apalarse::utils::AResult;
use serde_json::Value;

const CONSTANT: Int = 2_000_000;

fn main() -> AResult<()> {
    let config = runner::Config::default();
    let mut checker = config.checker();

    let x = checker.context.var::<Int>();

    let init = x.is_in(Nat).and(x.tla_leq(CONSTANT / (2 ^ 10)));

    let next =
        (x.rem(2).equal(0)).if_then_else(x.next().equal(x.quo(2)), x.next().equal(x.mul(3).add(1)));

    let inv = x.tla_leq(CONSTANT);

    for (i, cex) in checker
        .check::<_, _, _, _, Value>(&init, &next, &inv, &x, 20, 5)?
        .into_iter()
        .enumerate()
    {
        println!("counterexample {}", i);
        for (j, state) in cex.states.into_iter().enumerate() {
            println!("  state {}", j);
            println!("    {}: {}", x, x.value(&state.value)?);
        }
    }

    Ok(())
}
