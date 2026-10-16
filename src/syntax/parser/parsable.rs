use toml::Table;

pub(crate) trait Parsable<T> {
    fn parser(toml: Table) -> Result<T, String>;
    // fn linter(toml: toml::Value) -> Result<T, String>;
}
