use crate::services;

#[get("/")]
pub fn index() -> &'static str {
    services::index::home()
}