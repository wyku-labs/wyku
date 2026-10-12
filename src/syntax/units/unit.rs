use super::When;
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
    when: When,
    tools: Vec<(Tool, Regex)>,
    ok: bool,
}
