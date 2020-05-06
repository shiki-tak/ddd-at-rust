use std::str::FromStr;

use anyhow::Result;
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Display, Serialize, Deserialize)]
pub struct MailAddress(String);

impl MailAddress {
    pub fn new(s: &str) -> Self {
        MailAddress(s.to_string())
    }
}

impl FromStr for MailAddress {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(MailAddress(s.to_string()))
    }
}
