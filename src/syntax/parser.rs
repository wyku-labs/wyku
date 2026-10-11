use super::tools::tool::Tool;
use super::units::unit::Unit;

pub(crate) enum Syntaxes {
    Unit(Unit),
    Tool(Tool),
}

pub(crate) struct Toml(String); // Placeholder

trait Parsable {
    fn parser(toml: Toml) -> Result<Syntaxes, String>;
    fn linter(toml: Toml) -> Result<Syntaxes, String>;
}
