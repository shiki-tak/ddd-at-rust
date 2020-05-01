use anyhow::Result;

use super::{Id, Name, User};

pub trait IUserRepository: Clone {
    fn save(&self, user: User) -> Result<()>;
    fn find_by_id(&self, id: Id) -> Result<Option<User>>;
    fn find_by_name(&self, name: Name) -> Result<Option<User>>;
    fn delete(&self, user: User) -> Result<()>;
}
