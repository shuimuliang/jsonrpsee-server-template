// use chrono::NaiveDate;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "slot", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub slot: i64,
    pub parent: Option<i64>,
    pub status: String,
    #[sea_orm(column_type = "Date")]
    pub updated_on: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
