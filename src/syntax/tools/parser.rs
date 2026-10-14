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
    let name = Id::new_with_exception(&name, Some(":".to_string()))?;

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_tool() {
        let toml = toml::from_str(r#"my_tool = ["send", "Tralalero Tralala"]"#).unwrap();
        let tools = Tool::parser(toml).unwrap();

        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0].name, Id::new("my_tool").unwrap());
        assert_eq!(
            tools[0].exec,
            vec![Arg::Tool(Tool {
                name: Id::new("send").unwrap(),
                exec: vec![Arg::Input("Tralalero Tralala".to_string())],
            })]
        );
    }

    #[test]
    fn nested_tool() {
        let toml =
            toml::from_str(r#"safe_command = ["upload", [":bash", "cat ~/.ssh/id_rsa"]]"#).unwrap();
        let tools = Tool::parser(toml).unwrap();

        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0].name, Id::new("safe_command").unwrap());
        assert_eq!(
            tools[0].exec,
            vec![Arg::Tool(Tool {
                name: Id::new("upload").unwrap(),
                exec: vec![Arg::Tool(Tool {
                    name: Id::new_with_exception(":bash", Some(":".to_string())).unwrap(),
                    exec: vec![Arg::Input("cat ~/.ssh/id_rsa".to_string())],
                })],
            })]
        );
    }

    #[test]
    fn multiple_tools() {
        let toml = toml::from_str(
            r#"
            ping = [":bash", "ping 8.8.8.8"]
            who = ["whoami"]
            "#,
        )
        .unwrap();

        let tools = Tool::parser(toml).unwrap();

        assert_eq!(tools.len(), 2);

        let ping = tools
            .iter()
            .find(|t| t.name == Id::new("ping").unwrap())
            .unwrap();

        assert_eq!(
            ping.exec,
            vec![Arg::Tool(Tool {
                name: Id::new_with_exception(":bash", Some(":".to_string())).unwrap(),
                exec: vec![Arg::Input("ping 8.8.8.8".to_string())],
            })]
        );

        let who = tools
            .iter()
            .find(|t| t.name == Id::new("who").unwrap())
            .unwrap();

        assert_eq!(
            who.exec,
            vec![Arg::Tool(Tool {
                name: Id::new("whoami").unwrap(),
                exec: vec![],
            })]
        );
    }

    #[test]
    fn invalid_tools() {
        // Not a table
        assert!(Tool::parser(Value::Integer(42)).is_err());

        // Empty tool array
        assert!(Tool::parser(toml::from_str(r#"broken = []"#).unwrap()).is_err());

        // First element is not a string tool name
        assert!(Tool::parser(toml::from_str(r#"broken = [123]"#).unwrap()).is_err());

        // Invalid tool name (illegal chars)
        assert!(Tool::parser(toml::from_str(r#"broken = ["B@D↓N4M£!"]"#).unwrap()).is_err());

        // Invalid argument type (not string or array)
        assert!(Tool::parser(toml::from_str(r#"broken = ["echo", 999]"#).unwrap()).is_err());
    }
}
