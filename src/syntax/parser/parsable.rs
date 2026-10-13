use crate::syntax::Tool;
use crate::syntax::Unit;

pub(crate) struct Toml(String); // Placeholder

pub(crate) enum Syntaxes {
    Unit(Unit),
    Tool(Tool),
}

pub(crate) trait Parsable {
    fn parser(toml: Toml) -> Result<Syntaxes, String>;
    fn linter(toml: Toml) -> Result<Syntaxes, String>;
}
