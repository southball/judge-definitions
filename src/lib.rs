use std::clone::Clone;
use serde::{Serialize, Deserialize};

pub mod verdicts {
    pub const VERDICT_AC: &'static str = "AC";
    pub const VERDICT_WA: &'static str = "WA";
    pub const VERDICT_MLE: &'static str = "MLE";
    pub const VERDICT_TLE: &'static str = "TLE";
    pub const VERDICT_RE: &'static str = "RE";
    pub const VERDICT_WJ: &'static str = "WJ";
    pub const VERDICT_SE: &'static str = "SE";
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TestcaseOutput {
    pub verdict: String,
    pub time: f64,
    pub memory: i64,
    pub checker_output: String,
    pub sandbox_output: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct JudgeOutput {
    pub verdict: String,
    pub time: f64,
    pub memory: i64,
    pub testcases: Vec<TestcaseOutput>,
}
