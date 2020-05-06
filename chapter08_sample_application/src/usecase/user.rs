use anyhow::Result;
use common::MyError;

use crate::domain::{exists, HaveUserRepository, Name, User, UserRepository};
use crate::usecase::{CreateUserCommand, DeleteUserCommand, UpdateUserCommand, UserData};

pub trait UserApplicationService: HaveUserRepository + std::marker::Sized {
    fn register(&self, cmd: CreateUserCommand) -> Result<()> {
        let user = User::new(cmd.name().clone(), cmd.mail_address().clone());
        if exists(self, &user)? {
            bail!(MyError::internal_server_error("user already exist"))
        }
        self.provide_user_repository().save(user)
    }

    fn get_by_name(&self, name: Name) -> Result<UserData> {
        let user = self.provide_user_repository()
            .find_by_name(name)?
            .ok_or_else(|| MyError::internal_server_error("user can't find"))?;

        Ok(user.into())
    }

    fn update(&self, cmd: UpdateUserCommand) -> Result<()> {
        let mut user = self.provide_user_repository()
            .find_by_id(cmd.id().clone())?
            .ok_or_else(|| MyError::internal_server_error("user can't find"))?;

        if let Some(name) = cmd.name() {
            user.change_name(name.clone());
            if exists(self, &user)? {
                bail!(MyError::internal_server_error("user already exist"));
            }
        }

        if let Some(mail_address) = cmd.mail_address() {
            user.change_mail_address(mail_address.clone());
        }

        self.provide_user_repository().save(user)?;
        Ok(())
    }

    fn delete(&self, cmd: DeleteUserCommand) -> Result<()> {
        let target_id = *cmd.id();
        let user = self.provide_user_repository()
            .find_by_id(target_id)?
            .ok_or_else(|| MyError::internal_server_error("user can't find"))?;
        self.provide_user_repository().delete(user)?;
        Ok(())
    }
}

impl<T: HaveUserRepository> UserApplicationService for T {}

pub trait HaveUserApplicationService {
    type UserApplicationService: UserApplicationService;

    fn provide_user_service(&self) -> &Self::UserApplicationService;
}