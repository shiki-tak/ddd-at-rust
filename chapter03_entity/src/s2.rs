#![allow(dead_code)]

use derive_getters::Getters;
use anyhow::Result;

use common::MyError;

#[derive(Clone, Debug, Getters)]
pub struct User {
    name: Name,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Name(String);

impl User {
    pub fn new(name: Name) -> Self {
        Self { name }
    }

    pub fn change_name(&mut self, name: Name) -> Result<()> {
        self.name = name;
        Ok(())
    }
}

impl Name {
    fn new(name: Option<&str>) -> Result<Self>  {
        match name {
            Some(name) => {
                match name {
                    name if name.chars().count() > 3 => Ok(Name(name.to_string())),
                    _ => bail!(MyError::type_error("User's name should be grater than 3")),
                }
            }
            None => bail!(MyError::none_error("User's name is nil")),
        }
    }
}

#[test]
fn test_change_name() {
    let name1 = Name::new(Some("taro")).unwrap();
    let name1_cloned = name1.clone();

    let mut u1 = User::new(name1);

    assert_eq!(name1_cloned, u1.name);

    let name2 = Name::new(Some("jiro")).unwrap();
    let name2_cloned = name2.clone();
    u1.change_name(name2);

    assert_eq!(name2_cloned, u1.name);

    assert!(Name::new(None).is_err());
    assert!(Name::new(Some("a")).is_err());
}
