use super::super::id::Id;
use regex::Regex;

enum Runner {
    Model,
    Tool,
}

pub(crate) struct Unit {
    id: Id,
    runner: Runner,
    task: String,
    when: String,
    tools: Vec<(Id, Regex)>,
    ok: bool,
}
