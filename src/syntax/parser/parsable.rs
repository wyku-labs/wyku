use crate::syntax::Tool;
use crate::syntax::Unit;

use toml::Value;

pub(crate) trait Parsable<T> {
    fn parser(toml: toml::Value) -> Result<T, String>;
    // fn linter(toml: toml::Value) -> Result<T, String>;
}
