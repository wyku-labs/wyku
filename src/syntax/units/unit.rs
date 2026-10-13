use super::when::Expression;
use crate::syntax::Id;
use crate::syntax::Tool;

use regex::Regex;

#[derive(Debug)]
pub(crate) struct Unit {
    id: Id,
    task: String,
    exec: Tool,
    when: Expression,
    tools: Vec<(Tool, Regex)>,

    state: State,
    ok: bool,
}

#[derive(Debug)]
enum State {
    Start,
    Finish,
    Fail,
    Watch,
}
