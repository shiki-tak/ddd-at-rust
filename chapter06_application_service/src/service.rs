use super::{IUserRepository, User};

use anyhow::Result;

use derive_getters::Getters;

#[derive(Clone, Debug, Getters)]
pub struct UserService<Repo: IUserRepository> {
    repo: Repo,
}

impl<Repo> UserService<Repo> where Repo: IUserRepository {
    pub fn new(repo: Repo) -> Self {
        Self {repo}
    }

    pub fn exists(&self, user: &User) -> Result<bool> {
        let result = self.repo.find_by_name(user.name().clone())?;
        match result {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }
}

