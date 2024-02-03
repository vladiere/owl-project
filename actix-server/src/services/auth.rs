use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use argonautica::{Hasher, Verifier};
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::Sha256;
use sqlx::{self, FromRow};
use tracing::debug;

use crate::middleware::TokenClaims;
use crate::{appstate::AppState, server_config, AuthAdmin, RoleUser};

#[derive(Deserialize)]
struct RegisterAdminBody {
    firstname: String,
    lastname: String,
    middlename: String,
    tower: String,
    occupation: String,
    position: String,
    email_address: String,
    contact_number: String,
    username: String,
    password: String,
}

#[derive(Serialize, FromRow)]
struct AdminNoPassword {
    id: i64,
}

#[derive(Serialize, FromRow)]
struct AdminId {
    id: i64,
}

#[derive(Deserialize)]
struct LoginAdmin {
    username: String,
    password: String,
}

#[derive(Serialize, Debug)]
struct ResponseToken {
    access_token: String,
}

#[derive(Serialize, Debug)]
struct ErrorStatus {
    message: String,
    status: u32,
}

#[post("/register")]
pub async fn register_admin(
    state: Data<AppState>,
    body: Json<RegisterAdminBody>,
) -> impl Responder {
    // -- SQL Queries
    let search_query = "select id from user_login where username = $1";
    let query1 =
        "insert into user_info (firstname, middlename, lastname) values ($1,$2,$3) returning id";
    let query2 = "insert into user_login (user_id,username,password) values ($1,$2,$3)";
    let query3 =
        "insert into user_contact_info (user_id,email_address,contact_number) values ($1,$2,$3)";
    let query4 = "insert into user_job (user_id,occupation,position) values ($1,$2,$3)";
    let query5 = "insert into user_picture (user_id,img_path) values ($1, 'https://media.istockphoto.com/id/1312136351/photo/3d-illustration-of-cute-cartoon-man-with-eyeglasses-in-blue-shirt-with-arms-crossed-close-up.jpg?b=1&s=612x612&w=0&k=20&c=Ml61cAXSJuPreQqOtuYd2FVHGB1nbDN4TuqLFConXf4=')";
    let query6 = "insert into building (user_id,tower,unit) values ($1,$2,'-Admin')";

    let user: RegisterAdminBody = body.into_inner();

    debug!("{:<12} - register_admin: {}", user.username, "ATTEMPTING");

    match sqlx::query_as::<_, AdminId>(search_query)
        .bind(&user.username)
        .fetch_one(&state.db)
        .await
    {
        Ok(_) => HttpResponse::InternalServerError().json(format!("Username already exist")),
        Err(_) => {
            let mut hasher = Hasher::default();
            let hash = hasher
                .with_password(user.password)
                .with_secret_key(&server_config().HASH_SECRET)
                .hash()
                .unwrap();

            match sqlx::query_as::<_, AdminNoPassword>(query1)
                .bind(user.firstname)
                .bind(user.middlename)
                .bind(user.lastname)
                .fetch_one(&state.db)
                .await
            {
                Ok(user_res) => {
                    match sqlx::query(query2)
                        .bind(&user_res.id)
                        .bind(user.username)
                        .bind(hash)
                        .execute(&state.db)
                        .await
                    {
                        Ok(_) => {
                            match sqlx::query(query3)
                                .bind(&user_res.id)
                                .bind(user.email_address)
                                .bind(user.contact_number)
                                .execute(&state.db)
                                .await
                            {
                                Ok(_) => {
                                    match sqlx::query(query4)
                                        .bind(&user_res.id)
                                        .bind(user.occupation)
                                        .bind(user.position)
                                        .execute(&state.db)
                                        .await
                                    {
                                        Ok(_) => {
                                            match sqlx::query(query5)
                                                .bind(&user_res.id)
                                                .execute(&state.db)
                                                .await
                                            {
                                                Ok(_) => {
                                                    match sqlx::query(query6)
                                                        .bind(user_res.id)
                                                        .bind(user.tower)
                                                        .execute(&state.db)
                                                        .await
                                                    {
                                                        Ok(_) => {
                                                            let result = json!({
                                                                "message": "created successfully"
                                                            });
                                                            HttpResponse::Created().json(result)
                                                        }
                                                        Err(error) => {
                                                            debug!(
                                                                "{:<12} - query6 Error: {error:?}",
                                                                "ERROR"
                                                            );
                                                            HttpResponse::InternalServerError()
                                                                .json(format!("{:?}", error))
                                                        }
                                                    }
                                                }
                                                Err(error) => {
                                                    debug!(
                                                        "{:<12} - query5 Error: {error:?}",
                                                        "ERROR"
                                                    );
                                                    HttpResponse::InternalServerError()
                                                        .json(format!("{:?}", error))
                                                }
                                            }
                                        }
                                        Err(error) => {
                                            debug!("{:<12} - query4 Error: {error:?}", "ERROR");
                                            HttpResponse::InternalServerError()
                                                .json(format!("{:?}", error))
                                        }
                                    }
                                }
                                Err(error) => {
                                    debug!("{:<12} - query3 Error: {error:?}", "ERROR");
                                    HttpResponse::InternalServerError().json(format!("{:?}", error))
                                }
                            }
                        }
                        Err(error) => {
                            debug!("{:<12} - query2 Error: {error:?}", "ERROR");
                            HttpResponse::InternalServerError().json(format!("{:?}", error))
                        }
                    }
                }
                Err(error) => {
                    debug!("{:<12} - query1 Error: {error:?}", "ERROR");
                    HttpResponse::InternalServerError().json(format!("{:?}", error))
                }
            }
        }
    }
}

#[post("/login")]
pub async fn login_admin(state: Data<AppState>, credentials: Json<LoginAdmin>) -> impl Responder {
    let query = "select id, username, password, token_salt from user_login where username = $1";
    let query2 = "select role_user from user_info where id = $1";

    let jwt_secret: Hmac<Sha256> =
        Hmac::new_from_slice(&server_config().JWT_SECRET.as_bytes()).unwrap();

    let user: LoginAdmin = credentials.into_inner();
    let LoginAdmin {
        username,
        password: pass,
    } = user;

    debug!("{:<12} - login_admin {username:?}", "ATTEMPTING");
    if !username.is_empty() && !pass.is_empty() {
        if !username.is_empty() {
            if !pass.is_empty() {
                match sqlx::query_as::<_, AuthAdmin>(query)
                    .bind(username)
                    .fetch_one(&state.db)
                    .await
                {
                    Ok(user) => {
                        let mut verifier = Verifier::default();
                        let is_valid = verifier
                            .with_hash(user.password)
                            .with_password(pass)
                            .with_secret_key(&server_config().HASH_SECRET)
                            .verify()
                            .unwrap();

                        if is_valid {
                            match sqlx::query_as::<_, RoleUser>(query2)
                                .bind(user.id)
                                .fetch_one(&state.db)
                                .await
                            {
                                Ok(res) => {
                                    let claims = TokenClaims {
                                        id: user.id,
                                        username: user.username,
                                        token_salt: user.token_salt,
                                        role_user: res.role_user,
                                    };
                                    let token_str = ResponseToken {
                                        access_token: claims.sign_with_key(&jwt_secret).unwrap(),
                                    };

                                    HttpResponse::Ok().json(token_str)
                                }
                                Err(error) => {
                                    debug!("{:<12} - query2 login {error:?}", "ERROR");
                                    HttpResponse::InternalServerError().json(format!("{error:?}"))
                                }
                            }
                        } else {
                            let error = ErrorStatus {
                                message: String::from("Incorrect username or password"),
                                status: 401,
                            };
                            HttpResponse::Unauthorized().json(error)
                        }
                    }
                    Err(error) => HttpResponse::InternalServerError().json(format!("{:?}", error)),
                }
            } else {
                let error = ErrorStatus {
                    message: String::from("Password is required"),
                    status: 401,
                };
                HttpResponse::Unauthorized().json(error)
            }
        } else {
            let error = ErrorStatus {
                message: String::from("Email address is required"),
                status: 401,
            };
            HttpResponse::Unauthorized().json(error)
        }
    } else {
        let error = ErrorStatus {
            message: String::from("Email address and password are required"),
            status: 401,
        };
        HttpResponse::Unauthorized().json(error)
    }
}
