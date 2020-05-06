use derive_getters::Getters;
use serde::{Deserialize, Serialize};

use crate::domain::{UserId, Name, MailAddress};

#[derive(Clone, Debug, Getters, Serialize, Deserialize)]
pub struct User {
    id: UserId,
    name: Name,
    mail_address: MailAddress,
}

impl User {
    pub fn new(name: Name, mail_address: MailAddress) -> Self {
        Self {
            id: UserId::default(),
            name,
            mail_address,
        }
    }

    pub fn rebuild(id: UserId, name: Name, mail_address: MailAddress) -> Self {
        Self { id, name, mail_address, }
    }

    pub fn change_name(&mut self, name: Name) {
        self.name = name;
    }

    pub fn change_mail_address(&mut self, mail_address: MailAddress) {
        self.mail_address = mail_address;
    }
}
