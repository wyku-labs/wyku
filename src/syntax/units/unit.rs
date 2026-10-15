use super::when::Expression;

use crate::syntax::Id;
use crate::syntax::Tool;

use regex::Regex;

#[derive(Debug)]
pub(crate) struct Unit {
    id: Id,
    task: Option<String>,
    exec: Option<Tool>,
    when: Option<Expression>,
    tools: Option<Vec<Id>>,

    state: State,
    ok: Option<bool>,
}

#[derive(Debug)]
enum State {
    Start,
    Finish,
    Fail,
    Watch,
}
