use async_trait::async_trait;
use diesel::result;

#[async_trait]
pub trait Repository {
    type Id: Send + Sync;
    type Item;
    type NewItem<'a>;
    type UpdateForm<'a>;

    async fn get_all(&self) -> Result<Option<Vec<Self::Item>>, result::Error>;
    async fn get_by_id(&self, id: Self::Id) -> Result<Option<Self::Item>, result::Error>;
    async fn add<'a>(&self, item: Self::NewItem<'a>) -> Result<(), result::Error>;
    async fn update<'a>(
        &self,
        id: Self::Id,
        item: Self::UpdateForm<'a>,
    ) -> Result<(), result::Error>;
    async fn delete(&self, id: Self::Id) -> Result<(), result::Error>;
}
