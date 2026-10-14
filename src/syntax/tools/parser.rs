use toml::value::Array;
use toml::{Table, Value};

use crate::syntax::tools::tool::{Arg, Tool};
use crate::syntax::{Id, Parsable};

impl Parsable<Vec<Tool>> for Tool {
    fn parser(toml: toml::Value) -> Result<Vec<Tool>, String> {
        let Value::Table(table) = toml else {
            return Err(format!("expected an table, got '{toml}'"));
        };

        pre_parser(table)
            .into_iter()
            .map(|i| match i {
                Value::Array(tool) => parse_tool(tool),
                _ => Err("a tool must be an array: [base, [first arg], ...]".to_string()),
            })
            .collect()
    }
}

fn pre_parser(table: Table) -> Vec<Value> {
    table
        .into_iter()
        .map(|(name, value)| Value::Array(vec![Value::String(name), value]))
        .collect()
}

fn parse_tool(mut tool: Array) -> Result<Tool, String> {
    if tool.is_empty() {
        return Err("a tool cannot be empty".to_string());
    }

    let Value::String(name) = tool.remove(0) else {
        return Err(format!("expected tool name string"));
    };
    let name = Id::new(&name)?;

    let mut exec = Vec::<Arg>::new();
    for i in tool {
        match i {
            Value::String(string) => exec.push(Arg::Input(string)),
            Value::Array(array) => exec.push(Arg::Tool(parse_tool(array)?)),
            _ => {
                return Err(format!(
                    "expected a string or an array of strings, got '{i}'"
                ));
            }
        }
    }

    Ok(Tool { name, exec })
}
