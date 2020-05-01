use std::str::FromStr;

use uuid::Uuid;
use anyhow::Result;
use derive_getters::Getters;
use derive_more::Display;
use derive_new::new;

use common::MyError;

// -------------------------
// Repository
// -------------------------

pub trait IUserRepository: Clone {
    fn save(&self, user: User) -> Result<()>;
    fn find(&self, username: Name) -> Result<Option<User>>;
}

// -------------------------
// DomainService
// -------------------------

#[derive(Clone, Debug, new, Getters)]
pub struct Program<Repo: IUserRepository> {
    repo: Repo,
}

impl<Repo> Program<Repo> where Repo: IUserRepository {
    pub fn create_user(&mut self, username: Name) -> Result<()> {
        let user = User::new(username);

        let user_service = UserService::new(self.repo());
        if user_service.exists(user.clone())? {
            bail!(MyError::internal_server_error("target user already exist"))
        }
        self.repo.save(user)
    }
}

#[derive(Clone, Debug, Getters)]
pub struct UserService<Repo: IUserRepository> {
    repo: Repo,
}

impl<Repo> UserService<Repo> where Repo: IUserRepository {
    pub fn new(repo: &Repo) -> Self {
        Self { repo: repo.clone() }
    }

    pub fn exists(&self, user: User) -> Result<bool> {
        let result = self.repo.find(user.name().clone())?;
        match result {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }
}

// -------------------------
// Entity & Value Object
// -------------------------

#[derive(Clone, Debug, Getters)]
pub struct User {
    id: UserId,
    name: Name,
}

impl User {
    pub fn new(name: Name) -> Self {
        Self {
            id: UserId::default(),
            name,
        }
    }

    pub fn rebuild(id: UserId, name: Name) -> Self {
        Self { id, name }
    }
}

#[derive(Clone, Debug, Display, Hash, PartialEq, Eq)]
pub struct UserId(String);

impl Default for UserId {
    fn default() -> Self {
        let uuid = Uuid::new_v4();
        UserId(uuid.to_string())
    }
}

impl FromStr for UserId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(UserId(s.to_string()))
    }
}

#[derive(Clone, Debug, Display, PartialEq, Eq)]
pub struct Name(String);

impl FromStr for Name {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Name(s.to_string()))
    }
}

#[test]
fn test_new_user() {
    let name1 = Name::from_str("alice").unwrap();
    let user = User::new(name1);
    println!("{:?}", user);

    assert_eq!("alice", user.name.to_string());
}