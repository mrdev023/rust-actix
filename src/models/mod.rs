pub(self) type DbError = Box<dyn std::error::Error + Send + Sync>;

pub mod user;
