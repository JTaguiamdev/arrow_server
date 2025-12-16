use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use crate::api::request::{AssignCategoryRequest, CreateCategoryRequest, UpdateCategoryRequest};
use crate::security::jwt::AccessClaims;
use crate::services::errors::ProductCategoryServiceError;
use crate::services::product_category_service::ProductCategoryService;

pub async fn get_categories(claims: AccessClaims) -> impl IntoResponse {
    let service =  ProductCategoryService::new();
    
    if claims.roles.is_none() {
        return (StatusCode::FORBIDDEN, "Permission denied").into_response();
    }
    // Iterate through roles and return categories for the first role that has permission
    for role in claims.roles.unwrap() {
        match service.get_categories(role as i32).await {
            Ok(categories) => {
                return (StatusCode::OK, Json(categories)).into_response();
            }
            Err(ProductCategoryServiceError::PermissionDenied) => continue,
            Err(_) => {
                return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
            }
        }
    }
    
    StatusCode::NOT_FOUND.into_response()
}

pub async fn add_category(
    claims: AccessClaims,
    Json(payload): Json<CreateCategoryRequest>,
) -> impl IntoResponse {
    let service = ProductCategoryService::new();
    
    if claims.roles.is_none() {
        tracing::error!("Roles is None");
        return (StatusCode::FORBIDDEN, "Permission denied").into_response();
    }
    
    for role in claims.roles.unwrap() { // Verify if unwrap is safe here
        match service.add_category(role as i32, &payload.name, payload.description.as_deref()).await {
            Ok(_) => {
                tracing::info!("Added category {}", payload.name);
                return (StatusCode::CREATED, "Category added successfully").into_response();
            }
            Err(ProductCategoryServiceError::PermissionDenied) => continue,
            Err(_) => {
                tracing::error!("Failed to add category {}", payload.name);
                return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
            }
        }
    }
    
    (StatusCode::FORBIDDEN, "Permission denied").into_response()
}

pub async fn edit_category(
    claims: AccessClaims,
    Path(category_id): Path<i32>,
    Json(payload): Json<UpdateCategoryRequest>,
) -> impl IntoResponse {
    let service = ProductCategoryService::new();

    if claims.roles.is_none() {
        tracing::error!("Roles is none");
        return (StatusCode::FORBIDDEN, "Permission denied").into_response();
    }

    for role in claims.roles.unwrap() {
        match service.edit_category(role as i32, category_id, payload.name.as_deref(), payload.description.as_deref()).await {
            Ok(_) => {
                tracing::info!("Edited category {}", category_id);
                return (StatusCode::CREATED, "Category edited successfully").into_response();
            },
            Err(ProductCategoryServiceError::PermissionDenied) => continue,
            Err(_) => {
                tracing::error!("Failed to edit category {}", category_id);
                return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
            }
        }
    }

    (StatusCode::FORBIDDEN, "Permission denied").into_response()
}

pub async fn add_product_to_category(
    claims: AccessClaims,
    Json(payload): Json<AssignCategoryRequest>
) -> impl IntoResponse {
    let service = ProductCategoryService::new();

    if claims.roles.is_none() {
        tracing::error!("Roles is none");
        return (StatusCode::FORBIDDEN, "Permission denied").into_response();
    }

    for role in claims.roles.unwrap() {
        match service.add_product_to_category(role as i32, &*payload.category, &*payload.product).await {
            Ok(_) => {
                tracing::info!("Assigned product {} to category {}", payload.product, payload.category);
                return (StatusCode::CREATED, "Product assigned to category successfully").into_response();
            },
            Err(ProductCategoryServiceError::PermissionDenied) => continue,
            Err(_) => {
                tracing::error!("Failed to assign product {} to category {}", payload.product, payload.category);
                return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
            }
        }
    }

    (StatusCode::FORBIDDEN, "Permission denied").into_response()
}

// TODO: Implement delete_category controller function
// TODO: Implement remove_product_from_category controller function
// TODO: Implement get_products_by_category controller function
// TODO: Generate tests and put them in tests/api/controllers/category_controller_tests.rs
// TODO: Edit ProductResponse to include categories
