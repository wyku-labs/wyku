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
