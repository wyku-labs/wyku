use crate::syntax::Id;

#[derive(Debug)]
pub(super) enum Arg {
    Tool(Tool),
    Input(String),
}

#[derive(Debug)]
pub(crate) struct Tool {
    pub(super) name: Id,
    pub(super) exec: Vec<Arg>,
}
