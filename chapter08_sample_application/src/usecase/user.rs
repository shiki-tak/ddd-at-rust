use anyhow::Result;
use common::MyError;

use crate::domain::{exists, HaveUserRepository, Name, User, UserRepository};
use crate::usecase::{CreateUserCommand, DeleteUserCommand, UpdateUserCommand, UserData};

pub trait UserApplicationService: HaveUserRepository + std::marker::Sized {
    fn register(&self, cmd: CreateUserCommand) -> Result<()> {
        unimplemented!();
    }

    fn get_by_name(&self, name: Name) -> Result<UserData> {
        unimplemented!();
    }

    fn update(&self, cmd: UpdateUserCommand) -> Result<()> {
        unimplemented!();
    }

    fn delete(&self, cmd: DeleteUserCommand) -> Result<()> {
        unimplemented!();
    }
}

impl<T: HaveUserRepository> UserApplicationService for T {}

pub trait HaveUserApplicationService {
    type UserApplicationService: UserApplicationService;

    fn provide_user_service(&self) -> &Self::UserApplicationService;
}