use anyhow::Result;
use common::MyError;
use derive_getters::Getters;
use derive_new::new;

use super::{IUserRepository, Name, User, Id, MailAddress, UserService};

#[derive(Clone, Debug, Getters)]
pub struct UserApplicationService<Repo> where Repo: IUserRepository {
    repo: Repo,
    user_service: UserService<Repo>,
}

impl<Repo> UserApplicationService<Repo> where Repo: IUserRepository {
    pub fn new(repo: Repo, user_service: UserService<Repo>) -> Self {
        Self { repo, user_service}
    }

    pub fn register(&self, name: Name, mail_address: MailAddress) -> Result<()> {
        let user = User::new(name, mail_address);
        if self.user_service.exists(&user)? {
            bail!(MyError::internal_server_error("this user already exist"))
        }

        self.repo.save(user)
    }

    // // Example of direct return of a domain object.
    // pub fn get(&self, id: Id) -> Result<User> {
    //     /*
    //     let res = match self.repo.find_by_id(id) {
    //         Ok(user) => {
    //             match user {
    //                 Some(user) => user,
    //                 _ => bail!(MyError::internal_server_error("user didn't find"))
    //             }
    //         },
    //         Err(e) => bail!(MyError::internal_server_error("user didn't find"))
    //     };
    //     */
    //     let res = self.repo.find_by_id(id)?
    //         .ok_or_else(|| MyError::internal_server_error("user didn't find"))?;

    //     Ok(res)
    // }

    // Example of not returning the domain object directly, but via DTO.
    pub fn get(&self, id: Id) -> Result<UserData> {
        let user = self.repo()
            .find_by_id(id)?
            .ok_or_else(|| MyError::internal_server_error("user didn't find"))?;
        Ok(user.into())
    }

    pub fn update(&self, command: UserUpdateCommand) -> Result<()> {
        let mut user = self
            .repo()
            .find_by_id(command.id().clone())?
            .ok_or_else(|| MyError::internal_server_error("User didn't find"))?;

        if let Some(name) = command.name() {
            user.change_name(name.clone());
            if self.user_service().exists(&user)? {
                bail!(MyError::internal_server_error("User already exist"))
            }
        }

        if let Some(mail_address) = command.mail_address() {
            user.change_mail_address(mail_address.clone());
        }

        self.repo().save(user)?;
        Ok(())
    }

    pub fn delete(&self, command: UserDeleteCommand) -> Result<()> {
        let target_id = command.id().clone();
        let user = self
            .repo()
            .find_by_id(target_id)?
            .ok_or_else(|| MyError::internal_server_error("User didn't find"))?;
        self.repo().delete(user)?;
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct UserData {
    id: String,
    name: String,
    mail_address: String,
}

impl From<User> for UserData {
    fn from(v: User) -> Self {
        Self {
            id: v.id().to_string(),
            name: v.name().to_string(),
            mail_address: v.mail_address().to_string(),
        }
    }
}

#[derive(Clone, Debug, Getters, new)]
pub struct UserUpdateCommand {
    id: Id,
    name: Option<Name>,
    mail_address: Option<MailAddress>,
}

#[derive(Clone, Debug, Getters, new)]
pub struct UserDeleteCommand {
    id: Id,
}