use anyhow::Result;

use common::MyError;

#[derive(Clone, Debug)]
pub struct User {
    name: String,
}

impl User {
    pub fn new(name: String) -> Result<Self> {
        if name.chars().count() < 3 {
            bail!(MyError::type_error("User's name should be grater than 3"));
        }
        Ok(Self { name })
    }
}