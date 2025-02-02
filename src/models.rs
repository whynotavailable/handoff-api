use serde::Serialize;
use sqlx::PgPool;

#[derive(Clone, Debug)]
pub struct AppState {
    pub db: PgPool,
}

#[derive(Serialize)]
pub struct SimpleResponse {
    pub value: String,
}

// TODO: Remove this after talk and just use Display instead lol
pub trait StringLike {
    fn to_str(self) -> String;
}

impl StringLike for String {
    fn to_str(self) -> String {
        self
    }
}

impl StringLike for &str {
    fn to_str(self) -> String {
        self.to_string()
    }
}

impl SimpleResponse {
    pub fn new(value: impl StringLike) -> SimpleResponse {
        SimpleResponse {
            value: value.to_str(),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct DataSource {
    pub id: sqlx::types::Uuid,
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversions() {
        let cst: &str = "hi";
        assert_eq!(SimpleResponse::new(cst).value, "hi");

        let s: String = format!("{} {}", "hi", "dave");
        assert_eq!(SimpleResponse::new(s).value, "hi dave");
    }
}
