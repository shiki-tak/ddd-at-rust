use anyhow::Result;

use crate::domain::{HaveUserRepository, User, UserRepository};

pub fn exist<T>(ctx: &T, user: &User) -> Result<bool> where T: HaveUserRepository, {
    let repo = ctx.provide_user_repository();
    let duplicated_user = repo.find_by_name(user.name().clone())?;
    match duplicated_user {
        Some(_) => Ok(true),
        None => Ok(false),
    }
}