#[derive(Debug)]
enum Arg {
    Tool(Tool),
    Arg(String),
}

#[derive(Debug)]
pub(crate) struct Tool {
    name: String,
    args: Vec<Arg>,
}
