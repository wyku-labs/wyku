use super::when::Expression;
use crate::syntax::Id;
use crate::syntax::Tool;

use regex::Regex;

#[derive(Debug)]
enum Runner {
    Model,
    Tool,
}

#[derive(Debug)]
pub(crate) struct Unit {
    id: Id,
    runner: Runner,
    task: String,
    when: Expression,
    tools: Vec<(Tool, Regex)>,
    ok: bool,
}
