use std::ptr::hash;
use arrow_server_lib::data::database::*;
use diesel::result;
use diesel_async::RunQueryDsl;
use arrow_server_lib::data::models::user::NewUser;
use arrow_server_lib::data::repos::implementors::user_repo::UserRepo;
use arrow_server_lib::data::repos::traits::repository::Repository;
use arrow_server_lib::services::auth_service::AuthService;

async fn setup() -> Result<(), result::Error> {
    let db = Database::new().await;

    let mut conn = db
        .get_connection()
        .await
        .expect("Failed to get a database connection");

    use arrow_server_lib::data::models::schema::users::dsl::*;

    diesel::delete(users).execute(&mut conn).await?;

    Ok(())
}
// TODO: Implement tests for user repository methods
#[tokio::test]
#[serial_test::serial]
async fn test_create_user() {
    setup().await.expect("Setup failed");

    let auth = AuthService::new();

    let raw_password = "securepassword";
    let hashed = match auth.hash_password(raw_password).await {
        Ok(h) => h,
        Err(_) => panic!("Password hashing failed"),
    };

    let user = "testuser";

    let test_user = NewUser {
        username: user,
        password_hash: &hashed,
    };

    assert_eq!(match auth.verify_password(raw_password, &hashed).await {
        Ok(valid) => valid,
        Err(_) => panic!("Password verification failed"),
    }, true);
    
    let repo = UserRepo::new();

    match repo.add(test_user).await {
        Ok(_) => (),
        Err(_) => panic!("Failed to add test_user"),
    };

    let db_user = match repo.get_by_username(user).await {
        Ok(user) => {
            match user {
                Some(u) => u,
                None => panic!("test_user not found in database"),
            }
        }
        Err(_) => panic!("Failed to retrieve test_user"),
    };
    
    assert_eq!(db_user.username, user);
}

#[serial_test::serial]
#[tokio::test]
async fn test_get_all_users() {
    setup().await.expect("Setup failed");

    let repo = UserRepo::new();

    let users = match repo.get_all().await {
        Ok(u) => u,
        Err(_) => panic!("Failed to get all users"),
    };

    assert_eq!(users, None, "Expected no users in the database");
}