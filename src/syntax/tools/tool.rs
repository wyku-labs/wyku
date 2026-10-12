use crate::syntax::Id;

#[derive(Debug)]
enum Arg {
    Tool(Tool),
    Arg(String),
}

#[derive(Debug)]
pub(crate) struct Tool {
    name: Id,
    args: Vec<Arg>,
}
