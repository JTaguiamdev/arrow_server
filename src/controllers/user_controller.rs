use axum::http::StatusCode;
use axum::Json;
use axum::body::Body;
use axum::response::{IntoResponse, Response};
use crate::controllers::dto::user_dto::NewUserDTO;
use crate::data::models::user::NewUser;
use crate::data::repos::implementors::user_repo::UserRepo;
use crate::data::repos::traits::repository::Repository;
use crate::services::auth_service::AuthService;

pub async fn register_user(Json(new_user): Json<NewUserDTO>) -> impl IntoResponse {
    let auth = AuthService::new();
    let repo = UserRepo::new();

    let hashed_password = match auth.hash_password(&new_user.password).await {
        Ok(h) => h,
        Err(e) => {
            eprintln!("Error hashing password: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to process password").into_response();
        }
    };

    let new_user = NewUserDTO {
        username: new_user.username,
        password: hashed_password,
    };

    match repo
        .add(NewUser::from(&new_user))
        .await
        {
            Ok(_) => {
                Response::builder()
                    .status(StatusCode::CREATED)
                    .body(Body::from("User created"))
                    .unwrap()
            },
            Err(e) => {
                eprintln!("Error creating user: {}", e);
                Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from("Failed to create user"))
                    .unwrap()
            }
        }
}
