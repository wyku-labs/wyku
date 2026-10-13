use crate::syntax::Tool;
use crate::syntax::Unit;

use toml::Value as Toml;

pub(crate) enum Syntaxes {
    Unit(Unit),
    Tool(Tool),
}

pub(crate) trait Parsable {
    fn parser(toml: Toml) -> Result<Syntaxes, String>;
    fn linter(toml: Toml) -> Result<Syntaxes, String>;
}
