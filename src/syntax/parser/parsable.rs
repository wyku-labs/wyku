use crate::syntax::Tool;
use crate::syntax::Unit;

use toml::Value as Toml;

pub(crate) trait Parsable<T> {
    fn parser(toml: Toml) -> Result<T, String>;
    fn linter(toml: Toml) -> Result<T, String>;
}
