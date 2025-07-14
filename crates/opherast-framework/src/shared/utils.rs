use chrono::{DateTime, Utc};
use deunicode::deunicode;
use ulid::Ulid;

/// Get the current UTC timestamp
pub fn now_utc() -> DateTime<Utc> {
    Utc::now()
}

/// Generate a unique, sortable ID
pub fn generate_id() -> Ulid {
    Ulid::new()
}

/// Create a slug from a string (e.g. "Caffè Latté!" → "caffe-latte")
pub fn slugify(input: &str) -> String {
    deunicode(input)
        .to_lowercase()
        .replace(|c: char| !c.is_alphanumeric(), "-")
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}
