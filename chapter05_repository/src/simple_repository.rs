use std::convert::{TryFrom, TryInto};

use anyhow::Result;
use derive_new::new;
use postgres::{Client, NoTls};

use super::{IUserRepository, Name, User};

// -------------------------
// Infrastucture
// -------------------------

#[derive(Clone, Debug, new)]
pub struct UserRepository {}

const CONNECTION_STRING: &str = "postgres://postgres:password@localhost:5432/users";

impl IUserRepository for UserRepository {
    fn save(&self, user: User) -> Result<()> {
        let mut client = Client::connect(CONNECTION_STRING, NoTls)?;

        let id = user.id().to_string();
        let name = user.name().to_string();
        client.execute(
            r#"
            INSERT INTO users (id, name)
            VALUES ($1, $2)
            ON CONFLICT(id)
            DO UPDATE SET name = $2;
            "#,
            &[&id, &name],
        )?;

        Ok(())
    }

    fn find(&self, name: Name) -> Result<Option<User>> {
        let mut client = Client::connect(CONNECTION_STRING, NoTls)?;

        let name = name.to_string();
        let row = client.query("SELECT id, name FROM users WHERE name = $1", &[&name])?;

        let row = row.iter().next();
        match row {
            Some(row) => {
                let user_dto = UserDto {
                    id: row.get(0),
                    name: row.get(1),
                };
                Ok(Some(user_dto.try_into()?))
            }
            None => Ok(None),
        }
    }
}

pub struct UserDto {
    id: String,
    name: String,
}

impl TryFrom<UserDto> for User {
    type Error = anyhow::Error;

    fn try_from(v: UserDto) -> Result<Self> {
        let id = v.id.parse()?;
        let name = v.name.parse()?;
        Ok(User::rebuild(id, name))
    }
}