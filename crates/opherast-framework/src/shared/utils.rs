use chrono::{DateTime, Utc};
use deunicode::deunicode;
use ulid::Ulid;

pub fn now_utc() -> DateTime<Utc> {
    Utc::now()
}

pub fn generate_id() -> Ulid {
    Ulid::new()
}

pub fn slugify(input: &str) -> String {
    deunicode(input)
        .to_lowercase()
        .replace(|c: char| !c.is_alphanumeric(), "-")
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}
