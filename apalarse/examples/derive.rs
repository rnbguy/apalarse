use apalarse::runner::{Config, TlaTS};
use apalarse::tla::{
    BoolExpr, Context, Empty, Expr, Int, IntExpr, List, ListExpr, Map, Nat, Set, SetExpr, Symbol,
};
use apalarse::utils::AResult;
use serde::Deserialize;

#[apalarse_derive::tla_state]
struct State {
    foo: Int,
    bar: Int,
    fooset: Set<Int>,
    foolist: List<Int>,
    foomap: Map<Int, Int>,
}

impl TlaTS for State {
    type ViewType = Box<dyn Expr<Output = List<Int>>>;

    fn init(&self) -> Box<dyn BoolExpr> {
        // (((foo \in Nat) /\ (bar \in Nat)) /\ ((foo + 5) < bar))
        Box::new(
            BoolExpr::and(
                &BoolExpr::and(
                    &BoolExpr::and(
                        &self.foo.variable().is_in(Nat),
                        self.bar.variable().is_in(Nat),
                    ),
                    self.foo.variable().add(5).tla_lt(self.bar.variable()),
                ),
                self.fooset.variable().equal(Empty::new()),
            )
            .and(self.foolist.variable().equal(List::from(vec![2])))
            .and(self.foomap.variable().equal(Map::from(vec![(1, 2)]))),
        )
    }

    fn next(&self) -> Box<dyn BoolExpr> {
        // ((foo' = (foo + 1)) /\ (bar' = (bar - 2)))
        Box::new(
            BoolExpr::and(
                &BoolExpr::and(
                    &self.foo.variable().next().equal(self.foo.variable().add(1)),
                    self.bar.variable().next().equal(self.bar.variable().sub(2)),
                ),
                self.fooset
                    .variable()
                    .next()
                    .equal(self.fooset.variable().tla_insert(self.foo.variable())),
            )
            .and(
                self.foolist
                    .variable()
                    .next()
                    .equal(self.foolist.variable().append(self.foo.variable())),
            )
            .and(
                self.foomap
                    .variable()
                    .next()
                    .equal(Map::from(vec![(self.foo.variable(), self.bar.variable())])),
            ),
        )
    }

    fn inv(&self) -> Box<dyn BoolExpr> {
        // (foo /= bar)
        Box::new(self.foo.variable().nequal(self.bar.variable()))
    }

    fn view(&self) -> Self::ViewType {
        // // <<foo,bar>>
        Box::new(vec![self.foo.variable(), self.bar.variable()])
    }
}

fn main() -> AResult<()> {
    let config = Config::default();
    let mut checker = config.checker();

    let state = State::variable(&mut checker.context);

    // note: it returns step assignments in the original state struct
    for (i, cex) in state.checker(&mut checker)?.into_iter().enumerate() {
        println!("counterexample {}", i);
        for (j, step) in cex.into_iter().enumerate() {
            println!("  step {}: {:?}", j, step);
        }
    }

    Ok(())
}
