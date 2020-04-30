use derive_getters::Getters;
use anyhow::Result;

use common::MyError;

#[derive(Clone, Debug, Getters)]
pub struct User {
    name: String,
}

impl User {
    pub fn new(name: &str) -> Result<Self> {
        if name.chars().count() < 3 {
            bail!(MyError::type_error("User's name should be grater than 3"));
        }
        Ok(Self { name: name.to_string() })
    }

    pub fn change_name(&mut self, name: Option<&str>) -> Result<()> {
        match name {
            Some(name) => {
                match name {
                    name if name.chars().count() > 3 => self.name = name.to_string(),
                    _ => bail!(MyError::type_error("User's name should be grater than 3")),
                }
            }
            None => bail!(MyError::none_error("User's name is nil")),
        };

        Ok(())
    }
}

#[test]
fn test_change_name() {
    let mut u1 = User::new("taro").unwrap();
    assert_eq!("taro".to_string(), u1.name);

    u1.change_name(Some("jiro"));
    assert_eq!("jiro".to_string(), u1.name);

    assert!(u1.change_name(None).is_err());
}