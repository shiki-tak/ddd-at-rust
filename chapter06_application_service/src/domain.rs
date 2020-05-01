use uuid::Uuid;
use anyhow::Result;

use std::str::FromStr;

use derive_more::Display;
use derive_getters::Getters;

use common::MyError;

#[derive(Clone, Debug, Getters)]
pub struct User {
    id: Id,
    name: Name,
    mail_address: MailAddress,
}

impl User {
    pub fn new(name: Name, mail_address: MailAddress) -> Self {
        Self {id: Id::default(), name, mail_address}
    }

    pub fn rebuild(id: Id, name: Name, mail_address: MailAddress) -> Self {
        Self {id, name, mail_address}
    }

    pub fn change_name(&mut self, name: Name) {
        self.name = name;
    }

    pub fn change_mail_address(&mut self, mail_address: MailAddress) {
        self.mail_address = mail_address;
    }
}

#[derive(Clone, Debug, Display, Hash, PartialEq, Eq)]
pub struct Id(String);

impl Id {
    pub fn new(id: &str) -> Result<Self> {
        type Err = anyhow::Error;
        if id.chars().count() < 1 {
            bail!(MyError::type_error("User'id should be grater than 1"))
        }
        Ok(Self(id.to_string()))
    }
}

impl Default for Id {
    fn default() -> Self {
        let uuid = Uuid::new_v4();
        Id(uuid.to_string())
    }
}

#[derive(Clone, Debug, Display, PartialEq, Eq)]
pub struct Name(String);

impl Name {
    pub fn new(name: &str) -> Result<Self> {
        if name.chars().count() < 3 {
            bail!(MyError::type_error("User'id should be grater than 3"))
        }
        if name.chars().count() > 20 {
            bail!(MyError::type_error("User'id should be smaller than 20"))
        }

        Ok(Self(name.to_string()))
    }
}

impl FromStr for Name {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Name(s.to_string()))
    }
}

#[derive(Clone, Debug, Display, Hash, PartialEq, Eq)]
pub struct MailAddress(String);

impl MailAddress {
    pub fn new(s: &str) -> Result<Self> {
        Ok(MailAddress(s.to_string()))
    }
}
