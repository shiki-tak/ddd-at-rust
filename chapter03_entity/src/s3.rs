use anyhow::Result;
use derive_getters::Getters;
use derive_more::Display;
use std::str::FromStr;

use common::MyError;

#[derive(Clone, Debug, Getters)]
pub struct User {
    id: UserId,
    name: Name,
}

impl User {
    pub fn new(id: UserId, name: Name) -> Self {
        Self { id, name }
    }

    pub fn change_name(&mut self, name: Name) {
        self.name = name
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for User {}

#[derive(Clone, Debug, Display, PartialEq, Eq)]
pub struct UserId(String);

impl UserId {
    pub fn new(s: &str) -> Self {
        Self(s.to_string())
    }
}

#[derive(Clone, Display, Debug)]
pub struct Name(String);

impl FromStr for Name {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.chars().count() < 3 {
            bail!(MyError::type_error("User's name should be grater than 3"))
        }
        Ok(Self(s.to_string()))
    }
}

#[test]
fn test_user_eq() {
    let user_before = User::new(UserId::new("DummyId1"), "Hoge".parse().unwrap());
    let mut user_after = user_before.clone();
    user_after.change_name("Fuga".parse().unwrap());

    assert_eq!(user_before.name().to_string(), "Hoge".to_string());
    assert_eq!(user_after.name().to_string(), "Fuga".to_string());

    assert_eq!(user_before, user_after);
}