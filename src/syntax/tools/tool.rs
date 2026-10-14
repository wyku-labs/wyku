use crate::syntax::Id;

#[derive(Debug, PartialEq)]
pub(super) enum Arg {
    Tool(Tool),
    Input(String),
}

#[derive(Debug, PartialEq)]
pub(crate) struct Tool {
    pub(super) name: Id,
    pub(super) exec: Vec<Arg>,
}
