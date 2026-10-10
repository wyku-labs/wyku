use regex::Regex;

struct Id(String);

impl Id {
    pub(crate) fn new(id: &str) -> Result<Id, String> {
        let pattern = r"^[a-z_-]+$";
        let id_regex = Regex::new(pattern).expect("Invalid ID Regex");

        if !id_regex.is_match(&id) {
            return Err(format!("Invalid ID, Must Match '{pattern}'"));
        }

        Ok(Id(id.to_string()))
    }

    pub(crate) fn force(id: &str) -> Id {
        Id(id.to_string())
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
