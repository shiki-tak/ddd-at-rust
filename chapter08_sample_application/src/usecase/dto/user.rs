use crate::domain::User;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct UserData {
    id: String,
    name: String,
}

impl From<User> for UserData {
    fn from(v: User) -> Self {
        Self {
            id: v.id().to_string(),
            name: v.name().to_string(),
        }
    }
}