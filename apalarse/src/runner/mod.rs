use std::fs::File;
use std::io::Write;
use std::process::{Command, Stdio};

use itf::Trace;
use serde::Deserialize;
use serde_json::Value;

use crate::tla::{Bool, BoolExpr, Context, Expr};
use crate::utils::{AContext, AResult};

#[derive(Debug, Default)]
pub struct Apalache {
    pub config: Config,
    pub context: Context,
}

const RUN_DIR: &str = "run_dir";
const MODULE_NAME: &str = "apalarse";
const MODULE_FILE: &str = "apalarse.tla";
const INIT_OP: &str = "Apalarse_Init";
const NEXT_OP: &str = "Apalarse_Next";
const INV_OP: &str = "Apalarse_Inv";
const VIEW_OP: &str = "Apalarse_View";

impl Apalache {
    /// Run the model checker on the TLA+ spec.
    ///
    /// # Errors
    /// If the model checker fails.
    pub fn check<E1, E2, E3, E4, ST>(
        &mut self,
        init: &E1,
        next: &E2,
        invariant: &E3,
        view: &E4,
        max_depth: u64,
        max_error: u64,
    ) -> AResult<Vec<Trace<ST>>>
    where
        E1: Expr<Output = Bool>,
        E2: Expr<Output = Bool>,
        E3: Expr<Output = Bool>,
        E4: Expr,
        ST: for<'de> Deserialize<'de>,
    {
        let init_tla = init.tla_expr(&mut self.context);
        let next_tla = next.tla_expr(&mut self.context);
        let inv_tla = invariant.tla_expr(&mut self.context);
        // let view_tla = view.tla_expr(&mut self.context);
        let modules_tla = self.context.tla_modules()?;
        let variable_tla = self.context.tla_variables()?;

        let view_expr = self.context.tla_static_operator(VIEW_OP, view);

        let temp_dir = tempfile::tempdir()?;

        {
            let file_path = temp_dir.path().join(MODULE_FILE);

            {
                let mut tla_code = File::create(&file_path)?;
                writeln!(tla_code, "---- MODULE {MODULE_NAME} ----")?;
                writeln!(tla_code, "{modules_tla}")?;
                writeln!(tla_code, "{variable_tla}")?;
                writeln!(tla_code, "{INIT_OP} == {init_tla}")?;
                writeln!(tla_code, "{NEXT_OP} == {next_tla}")?;
                writeln!(tla_code, "{INV_OP} == {inv_tla}")?;
                writeln!(tla_code, "{view_expr}")?;
                writeln!(tla_code, "====")?;
            }

            println!("{}", std::fs::read_to_string(&file_path)?);
        }

        let exit_status = Command::new("apalache-mc")
            .args([
                "check",
                &format!("--init={INIT_OP}"),
                &format!("--next={NEXT_OP}"),
                &format!("--inv={INV_OP}"),
                &format!("--view={VIEW_OP}"),
                &format!("--length={max_depth}"),
                &format!("--max-error={max_error}"),
                &format!("--run-dir={RUN_DIR}"),
                MODULE_FILE,
            ])
            .current_dir(&temp_dir)
            .stdout(Stdio::null())
            .status()?;

        if exit_status.success() {
            println!("No CEX");

            let file_path = temp_dir.path().join(RUN_DIR).join("example.itf.json");
            let itf_str = std::fs::read_to_string(file_path)?;
            let trace: Trace<Value> = itf::trace_from_str(&itf_str)?;
            println!("{trace:?}");
            Err(anyhow::Error::msg("No CEX"))
        } else if exit_status.code() == Some(12) {
            println!("Found CEX");

            let ignore_path = temp_dir.path().join(RUN_DIR).join("violation.itf.json");
            let glob_path = temp_dir.path().join(RUN_DIR).join("violation*.itf.json");
            let glob_pattern = glob_path
                .as_os_str()
                .to_str()
                .context("Failed to convert path to string")?;

            let violations = glob::glob(glob_pattern)
                .context("Failed to read glob pattern")?
                .filter(|entry| entry.as_ref().map_or(true, |path| !path.eq(&ignore_path)))
                .collect::<Result<Vec<_>, _>>()?;

            Ok(violations
                .into_iter()
                .map(|path| {
                    let itf_str = std::fs::read_to_string(path)?;
                    Ok(itf::trace_from_str(&itf_str)?)
                })
                .collect::<AResult<Vec<_>>>()?)
        } else {
            Err(anyhow::Error::msg(format!(
                "something else happened: {:?}",
                exit_status.code()
            )))
        }
    }
}

#[derive(Debug, Default)]
pub struct Config {}

impl Config {
    #[must_use]
    pub fn checker(self) -> Apalache {
        Apalache {
            config: self,
            context: Context::default(),
        }
    }
}

pub trait TlaTS: Sized + for<'de> Deserialize<'de> {
    type ViewType: Expr;

    fn init(&self) -> Box<dyn BoolExpr>;
    fn next(&self) -> Box<dyn BoolExpr>;
    fn inv(&self) -> Box<dyn BoolExpr>;
    fn view(&self) -> Self::ViewType;

    /// Run the model checker on the TLA+ spec.
    ///
    /// # Errors
    /// If the model checker fails.
    fn checker(&self, checker: &mut Apalache) -> AResult<Vec<Vec<Self>>> {
        Ok(checker
            .check::<_, _, _, _, Self>(
                &self.init(),
                &self.next(),
                &self.inv(),
                &self.view(),
                20,
                5,
            )?
            .into_iter()
            .map(|x| x.states.into_iter().map(|y| y.value).collect::<Vec<_>>())
            .collect::<Vec<_>>())
    }
}
