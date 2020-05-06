use warp::{Rejection, Reply};

use crate::{
    domain::Name,
    usecase::{CreateUserCommand, HaveUserApplicationService, UserApplicationService},
};

pub async fn get_user_handler<T>(app: T, name: Name) -> Result<impl Reply, Rejection>
where
    T: HaveUserApplicationService,
{
    let service = app.provide_user_service();
    match service.get_by_name(name) {
        Ok(user) => Ok(warp::reply::json(&user)),
        Err(_) => Err(warp::reject::not_found()),
    }
}

pub async fn register_user_handler<T>(app: T, cmd: CreateUserCommand) -> Result<impl Reply, Rejection>
where
    T: HaveUserApplicationService,
{
    let service = app.provide_user_service();
    match service.register(cmd) {
        Ok(_) => Ok(warp::reply::with_status("success", warp::http::StatusCode::OK,)),
        Err(_) => Err(warp::reject::reject()),
    }
}