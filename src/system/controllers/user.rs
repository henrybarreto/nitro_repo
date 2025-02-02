use crate::api_response::APIResponse;
use crate::error::request_error::RequestError;
use crate::error::request_error::RequestError::{NotAuthorized, NotFound};
use crate::system::action::{delete_user_db, get_user_by_username, get_users, update_user};
use crate::system::models::User;
use crate::system::utils::{get_user_by_header, new_user, ModifyUser, NewPassword, NewUser};
use crate::utils::installed;
use crate::DbPool;
use actix_web::{get, post, web, HttpRequest};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListUsers {
    pub users: Vec<User>,
}

#[get("/api/admin/user/list")]
pub async fn list_users(
    pool: web::Data<DbPool>,
    r: HttpRequest,
) -> Result<APIResponse<ListUsers>, RequestError> {
    let connection = pool.get()?;
    installed(&connection)?;
    let admin = get_user_by_header(r.headers(), &connection)?.ok_or_else(|| NotAuthorized)?;
    if !admin.permissions.admin {
        return Err(NotAuthorized);
    }
    let vec = get_users(&connection)?;

    let response = ListUsers { users: vec };
    return Ok(APIResponse::new(true, Some(response)));
}

#[post("/api/admin/user/add")]
pub async fn add_user(
    pool: web::Data<DbPool>,
    r: HttpRequest,
    nc: web::Json<NewUser>,
) -> Result<APIResponse<User>, RequestError> {
    let connection = pool.get()?;
    installed(&connection)?;
    let _user = get_user_by_header(r.headers(), &connection)?.ok_or_else(|| NotAuthorized)?;
    let user = new_user(nc.0.clone(), &connection)?;
    return Ok(APIResponse::new(true, Some(user)));
}

#[post("/api/admin/user/{user}/modify")]
pub async fn modify_user(
    pool: web::Data<DbPool>,
    r: HttpRequest,
    web::Path(user): web::Path<String>,
    nc: web::Json<ModifyUser>,
) -> Result<APIResponse<User>, RequestError> {
    let connection = pool.get()?;
    installed(&connection)?;
    let admin = get_user_by_header(r.headers(), &connection)?.ok_or_else(|| NotAuthorized)?;
    if !admin.permissions.admin {
        return Err(NotAuthorized);
    }
    let mut user = get_user_by_username(user, &connection)?.ok_or(NotFound)?;
    user.update(nc.0.clone());
    update_user(&user, &connection)?;
    return Ok(APIResponse::new(true, Some(user)));
}

#[post("/api/admin/user/{user}/password")]
pub async fn change_password(
    pool: web::Data<DbPool>,
    r: HttpRequest,
    web::Path(user): web::Path<String>,
    nc: web::Json<NewPassword>,
) -> Result<APIResponse<User>, RequestError> {
    let connection = pool.get()?;
    installed(&connection)?;
    let admin = get_user_by_header(r.headers(), &connection)?.ok_or_else(|| NotAuthorized)?;
    if !admin.permissions.admin {
        return Err(NotAuthorized);
    }
    let mut user = get_user_by_username(user, &connection)?.ok_or(NotFound)?;
    let string = nc.0.hash().unwrap();
    println!("{}", &string);
    user.set_password(string);
    update_user(&user, &connection)?;
    return Ok(APIResponse::new(true, Some(user)));
}

#[get("/api/admin/user/{user}/delete")]
pub async fn delete_user(
    pool: web::Data<DbPool>,
    r: HttpRequest,
    web::Path(user): web::Path<String>,
) -> Result<APIResponse<bool>, RequestError> {
    let connection = pool.get()?;
    installed(&connection)?;

    let admin = get_user_by_header(r.headers(), &connection)?.ok_or_else(|| NotAuthorized)?;
    if !admin.permissions.admin {
        return Err(NotAuthorized);
    }
    let option = get_user_by_username(user, &connection)?.ok_or(NotFound)?;

    return Ok(APIResponse::<bool>::new(
        delete_user_db(option.id, &connection)?,
        None,
    ));
}
