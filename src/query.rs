use crate::entity::{Entity as SlotEntity, Model};
use sea_orm::EntityTrait;
use sea_orm::{DbConn, DbErr};

pub struct Query;

impl Query {
    pub async fn find_all(db: &DbConn) -> Result<Vec<Model>, DbErr> {
        SlotEntity::find().all(db).await
    }
    #[allow(dead_code)]
    pub async fn find_one(db: &DbConn, id: i64) -> Result<Option<Model>, DbErr> {
        SlotEntity::find_by_id(id).one(db).await
    }
}
