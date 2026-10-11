use super::super::id::Id;
use super::super::tools::tool::Tool;
use super::when::When;

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
