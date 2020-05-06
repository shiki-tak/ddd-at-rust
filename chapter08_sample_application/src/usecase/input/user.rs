use derive_getters::Getters;
use derive_new::new;
use serde::Deserialize;

use crate::domain::{MailAddress, Name, UserId};

#[derive(Clone, Debug, Getters, new, Deserialize)]
pub struct CreateUserCommand {
    name: Name,
    mail_address: MailAddress,
}

#[derive(Clone, Debug, Getters, new)]
pub struct UpdateUserCommand {
    id: UserId,
    name: Option<Name>,
    mail_address: Option<MailAddress>,
}

#[derive(Clone, Debug, Getters, new)]
pub struct DeleteUserCommand {
    id: UserId,
}

