use diesel::result::{DatabaseErrorKind, Error};
use crate::api::response::CategoryResponse;
use crate::data::models::categories::{Category, NewCategory, UpdateCategory};
use crate::data::models::product_category::NewProductCategory;
use crate::data::models::user_roles::RolePermissions;
use crate::data::repos::implementors::category_repo::CategoryRepo;
use crate::data::repos::implementors::product_category_repo::ProductCategoryRepo;
use crate::data::repos::implementors::product_repo::ProductRepo;
use crate::data::repos::implementors::user_role_repo::UserRoleRepo;
use crate::data::repos::traits::repository::Repository;
use crate::services::errors::ProductCategoryServiceError;

pub struct ProductCategoryService {}

impl ProductCategoryService {
    pub fn new() -> Self {
        ProductCategoryService {}
    }
    
    pub async fn get_categories(&self, role_id: i32) -> Result<Option<Vec<CategoryResponse>>, ProductCategoryServiceError>{
        if !self.has_permission(role_id, RolePermissions::Read).await?
            && !self.has_permission(role_id, RolePermissions::Admin).await?
        {
            Err(ProductCategoryServiceError::PermissionDenied)?
        }

        let is_admin = self.has_permission(role_id, RolePermissions::Admin).await?;

        let repo = CategoryRepo::new();

        let categories = repo.get_all()
            .await
            .map_err(|_| ProductCategoryServiceError::DatabaseError)?;
        
        let mut res = categories
            .map(|cats| {
                cats.into_iter()
                    .map(|c| c.into())
                    .collect()
            });

        if is_admin {
            Ok(res)
        } else {
            if let Some(categories) = res.as_mut() {
                categories
                    .into_iter()
                    .for_each(|cat| {
                        cat.category_id = None;
                        cat.created_at = None;
                        cat.updated_at = None;
                    });
            }

            Ok(res)
        }
    }

    pub async fn add_category(
        &self,
        role_id: i32,
        name: &str,
        description: Option<&str>,
    ) -> Result<i32, ProductCategoryServiceError> {
        if !self.has_permission(role_id, RolePermissions::Write).await?
            && !self.has_permission(role_id, RolePermissions::Admin).await?
        {
            Err(ProductCategoryServiceError::PermissionDenied)?
        }

        let repo = CategoryRepo::new();

        let new_category = NewCategory { name, description };

        repo.add(new_category)
            .await
            .map_err(|_| ProductCategoryServiceError::DatabaseError)?;

        repo.get_by_name(name)
            .await
            .map(|c| match c {
                Some(c) => c.category_id,
                None => -1
            })
            .and_then(|v| if v == -1 {
                Err(Error::DatabaseError(
                    DatabaseErrorKind::UnableToSendCommand,
                    Box::new(v.to_string())
                ))
            } else {
                Ok(v)
            })
            .map_err(|_| ProductCategoryServiceError::CategoryNotFound)
    }

    pub async fn add_product_to_category(
        &self,
        role_id: i32,
        category_name: &str,
        product_name: &str,
    ) -> Result<(), ProductCategoryServiceError> {
        if !self.has_permission(role_id, RolePermissions::Write).await?
            && !self.has_permission(role_id, RolePermissions::Admin).await?
        {
            Err(ProductCategoryServiceError::PermissionDenied)?
        }

        let product_repo = ProductRepo::new();
        let category_repo = CategoryRepo::new();
        let product_category_repo = ProductCategoryRepo::new();

        let product = product_repo
            .get_by_name(product_name)
            .await
            .map_err(|_| ProductCategoryServiceError::DatabaseError)?
            .ok_or(ProductCategoryServiceError::ProductNotFound)?;

        let category = category_repo
            .get_by_name(category_name)
            .await
            .map_err(|_| ProductCategoryServiceError::DatabaseError)?
            .ok_or(ProductCategoryServiceError::CategoryNotFound)?;

        let new_product_category = NewProductCategory {
            product_id: &product.product_id,
            category_id: &category.category_id,
        };

        product_category_repo.add(new_product_category)
            .await
            .map_err(|_| ProductCategoryServiceError::DatabaseError)
    }

    pub async fn edit_category(
        &self,
        role_id: i32,
        category_id: i32,
        name: Option<&str>,
        description: Option<&str>,
    ) -> Result<(), ProductCategoryServiceError> {
        if !self.has_permission(role_id, RolePermissions::Write).await?
            && !self.has_permission(role_id, RolePermissions::Admin).await?
        {
            Err(ProductCategoryServiceError::PermissionDenied)?
        }

        let repo = CategoryRepo::new();

        let updated_category = UpdateCategory { name, description };

        repo.update(category_id, updated_category)
            .await
            .map_err(|_| ProductCategoryServiceError::DatabaseError)
    }

    pub async fn delete_category(
        &self,
        role_id: i32,
        category_id: i32,
    ) -> Result<(), ProductCategoryServiceError> {
        if !self.has_permission(role_id, RolePermissions::Write).await?
            && !self.has_permission(role_id, RolePermissions::Admin).await?
        {
            Err(ProductCategoryServiceError::PermissionDenied)?
        }

        let repo = CategoryRepo::new();

        repo.delete(category_id)
            .await
            .map_err(|_| ProductCategoryServiceError::DatabaseError)
    }
    
    // TODO: Read operations for categories and product-category relationships

    pub async fn remove_product_from_category(
        &self,
        role_id: i32,
        category_name: &str,
        product_name: &str,
    ) -> Result<(), ProductCategoryServiceError> {
        if !self.has_permission(role_id, RolePermissions::Write).await?
            && !self.has_permission(role_id, RolePermissions::Admin).await?
        {
            Err(ProductCategoryServiceError::PermissionDenied)?
        }

        let product_repo = ProductRepo::new();
        let category_repo = CategoryRepo::new();
        let product_category_repo = ProductCategoryRepo::new();

        let product = product_repo
            .get_by_name(product_name)
            .await
            .map_err(|_| ProductCategoryServiceError::DatabaseError)?
            .ok_or(ProductCategoryServiceError::ProductNotFound)?;

        let category = category_repo
            .get_by_name(category_name)
            .await
            .map_err(|_| ProductCategoryServiceError::DatabaseError)?
            .ok_or(ProductCategoryServiceError::CategoryNotFound)?;

        product_category_repo
            .delete((product.product_id, category.category_id))
            .await
            .map_err(|_| ProductCategoryServiceError::DatabaseError)
    }

    async fn has_permission(
        &self,
        role_id: i32,
        required_permission: RolePermissions,
    ) -> Result<bool, ProductCategoryServiceError> {
        let role_repo = UserRoleRepo::new();
        if let Some(role) = role_repo
            .get_by_id(role_id)
            .await
            .map_err(|_| ProductCategoryServiceError::DatabaseError)?
            && let Some(perm) = role.permissions.and_then(|p| p.as_permission())
        {
            return Ok(perm == required_permission);
        }
        Ok(false)
    }
}

impl Default for ProductCategoryService {
    fn default() -> Self {
        Self::new()
    }
}
