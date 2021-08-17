use crate::system;
use actix_web::http::HeaderMap;
use diesel::MysqlConnection;
use crate::system::models::{User, UserPermissions};
use crate::apierror::APIError;
use actix_web::{HttpRequest, HttpMessage};
use crate::utils::{get_current_time};
use crate::apierror::APIError::MissingArgument;
use crate::system::action::{add_new_user, get_user_by_username};
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::rand_core::OsRng;
use serde::{Serialize, Deserialize};
pub fn get_user_by_cookie(
    http: &HttpRequest,
    conn: &MysqlConnection,
) -> Result<Option<User>, APIError> {
    let option = http.cookie("session");
    if option.is_none() {
        return Ok(None);
    }
    let x = option.as_ref().unwrap().value().clone();

    let result = system::action::get_user_from_auth_token(x.to_string(), conn)?;
    return Ok(result);
}

pub fn get_user_by_header(
    header_map: &HeaderMap,
    conn: &MysqlConnection,
) -> Result<Option<User>, APIError> {
    let option = header_map.get("Authorization");
    if option.is_none() {
        return Ok(None);
    }
    let x = option.unwrap().to_str();
    if x.is_err() {}
    let header = x.unwrap().to_string();

    let split = header.split(" ").collect::<Vec<&str>>();
    let option = split.get(0);
    if option.is_none() {
        return Ok(None);
    }
    let value = split.get(1);
    if value.is_none() {
        return Ok(None);
    }
    let value = value.unwrap().to_string();
    let key = option.unwrap().to_string();
    if key.eq("Bearer") {
        let result = system::action::get_user_from_auth_token(value, conn)?;
        return Ok(result);
    }
    Ok(None)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewUser {
    pub name: String,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<NewPassword>,
    pub permissions: UserPermissions,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewPassword {
    pub password: String,
    pub password_two: String,
}

impl NewPassword {
    pub fn hash(&self) -> Result<String, APIError> {
        if self.password != self.password_two {
            return Err(APIError::from("Mismatching Password"));
        }
        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password_simple(self.password.as_bytes(), salt.as_ref())
            .unwrap()
            .to_string();
        return Ok(password_hash);
    }
}

pub fn new_user(new_user: NewUser, conn: &MysqlConnection) -> Result<User, APIError> {
    let username = new_user
        .username
        .ok_or(MissingArgument("Username".into()))?;
    let option = system::action::get_user_by_username(username.clone(), &conn)?;
    if option.is_some() {
        return Err(APIError::Error("Username Already Exists".into()));
    }
    let email = new_user.email.ok_or(MissingArgument("Email".into()))?;
    let option = system::action::get_user_by_email(email.clone(), &conn)?;
    if option.is_some() {
        return Err(APIError::from("Email Already Exists"));
    }

    let user = User {
        id: 0,
        name: new_user.name.clone(),
        username: username.clone(),
        email: email.clone(),
        password: new_user
            .password
            .ok_or(MissingArgument("Missing Password".into()))?
            .hash()?,
        permissions: new_user.permissions.clone(),
        created: get_current_time(),
    };
    add_new_user(&user, &conn)?;
    return Ok(
        get_user_by_username(username, &conn)?.ok_or(APIError::from("Unable to find new user"))?
    );
}
