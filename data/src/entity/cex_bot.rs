//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "cex_bot")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub exchange: String,
    pub market: String,
    pub key: String,
    pub secret: String,
    pub passphrase: String,
    pub tag: String,
    pub status: i32,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
