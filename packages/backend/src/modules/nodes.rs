pub mod delete;
pub mod query;
pub mod save;

fn check_name(name: String) -> bool {
    name.chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}
