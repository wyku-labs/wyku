use crate::syntax::Id;
use regex::Regex;

#[derive(Debug)]
pub(crate) struct Permission {
    pub(super) id: Option<Id>,
    pub(super) pattern: Regex,
}

impl Permission {
    pub(crate) fn parser(raw: &str) -> Result<Permission, String> {
        let default_regex = Regex::new(".*").map_err(|e| e.to_string())?;

        if raw == "*" {
            return Ok(Permission {
                id: None,
                pattern: default_regex,
            });
        }

        let regex = Regex::new(r"^([^(]+)\((.+)\)$").map_err(|e| e.to_string())?;

        let Some(capture) = regex.captures(raw) else {
            return Ok(Permission {
                id: Some(Id::new_tool(raw)?),
                pattern: default_regex,
            });
        };

        Ok(Permission {
            // NOTE: panics if change the regex capture groups
            id: Some(Id::new_tool(&capture[1])?),
            pattern: Regex::new(&capture[2]).map_err(|e| e.to_string())?,
        })
    }
}
