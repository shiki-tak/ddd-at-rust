use derive_getters::Getters;
use regex::Regex;
use std::str::FromStr;

use common::MyError;

#[derive(Clone, Debug, Getters, PartialEq, Eq)]
pub struct FullName {
    first_name: Name,
    last_name: Name,
}

impl FullName {
    pub fn new(first_name: Name, last_name: Name) -> Self {
        Self {
            first_name: first_name,
            last_name: last_name,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Name(String);

impl FromStr for Name {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r#"^[a-zA-Z]+$"#).unwrap();
        if re.is_match(s) {
            Ok(Name(s.to_string()))
        } else {
            bail!(MyError::type_error("invalid character used"))
        }
    }
}

#[test]
fn show_full_name() {
    let first_name = "taro".parse().unwrap();
    let last_name = "yamada".parse().unwrap();

    let full_name = FullName::new(first_name, last_name);

    println!("{:?}", full_name);
}

#[test]
fn test_parse_name() {
    let valid_name = "taro".parse::<Name>();
    let invalid_name_with_num = "taro123".parse::<Name>();
    let invalid_name_with_jpn = "太郎".parse::<Name>();

    assert!(valid_name.is_ok());
    assert!(invalid_name_with_num.is_err());
    assert!(invalid_name_with_jpn.is_err());
}
