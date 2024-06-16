use apalarse::runner;
use apalarse::tla::{BoolExpr, Expr, Int, IntExpr, Nat, RangeSet, SetExpr, Variable};
use apalarse::utils::AResult;
use serde_json::Value;

fn main() -> AResult<()> {
    let config = runner::Config::default();
    let mut checker = config.checker();

    struct Vars {
        x: Variable<Int>,
        y: Variable<Int>,
    }

    let vars = Vars {
        x: checker.context.var(),
        y: checker.context.var(),
    };

    let init = vars.x.equal(0).and(vars.y.equal(2));

    let next = RangeSet::new(500, 1500)
        .exists(|v| vars.x.next().equal(vars.x.add(v)))
        .and(Nat.exists(|v| vars.y.next().equal(vars.y.add(v))));

    let inv = vars.x.add(vars.y.clone()).tla_lt(2_000_000);
    let view = vars.x.add(vars.y.clone()).quo(100);

    // TODO: figure out how to make sure var' is assigned in a next operator in every path
    // I want to do this via rust type system

    for (i, cex) in checker
        .check::<_, _, _, _, Value>(&init, &next, &inv, &view, 20, 5)?
        .into_iter()
        .enumerate()
    {
        println!("counterexample {i}");
        for (j, state) in cex.states.into_iter().enumerate() {
            println!("  state {j}");
            println!("    {}: {}", vars.x, vars.x.value(&state.value)?);
            println!("    {}: {}", vars.y, vars.y.value(&state.value)?);
        }
    }

    Ok(())
}
