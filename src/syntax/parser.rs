use crate::syntax::Tool;
use crate::syntax::Unit;

pub(crate) enum Syntaxes {
    Unit(Unit),
    Tool(Tool),
}

pub(crate) struct Toml(String); // Placeholder

trait Parsable {
    fn parser(toml: Toml) -> Result<Syntaxes, String>;
    fn linter(toml: Toml) -> Result<Syntaxes, String>;
}
