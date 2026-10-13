use regex::Regex;

#[derive(Debug, PartialEq)]
pub(crate) struct Id {
    id: String,
    exception: Option<String>, // Stored Only For Debugging
}

impl Id {
    pub(crate) fn new(id: &str) -> Result<Id, String> {
        Id::new_with_exception(id, None::<String>)
    }

    pub(crate) fn new_with_exception(id: &str, exception: Option<String>) -> Result<Id, String> {
        let pattern_exception = regex::escape(exception.as_deref().unwrap_or(""));
        let pattern = format!(r"^[a-z_\-{pattern_exception}]+$");

        let Ok(id_regex) = Regex::new(&pattern) else {
            return Err(format!(
                "Invalid Regex '{pattern}', Supplied Exception '{pattern_exception}'"
            ));
        };

        if !id_regex.is_match(&id) {
            return Err(format!("Invalid ID '{id}', Must Match '{pattern}'"));
        }

        Ok(Id {
            id: id.to_string(),
            exception,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn invalid_ids() {
        let ids = vec!["Hello World", "Tooling", "Ñ", "Agent"];

        for i in ids {
            let id = Id::new(i);
            assert!(id.is_err())
        }
    }

    #[test]
    fn valid_ids() {
        let ids = vec!["main", "check", "worker", "agent"];

        for i in ids {
            let id = Id::new(i);
            assert!(id.is_ok())
        }
    }
}
