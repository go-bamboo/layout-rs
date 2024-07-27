//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "cex_market_symbol")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub exchange: String,
    pub market: String,
    pub symbol: String,
    pub interval: String,
    pub base: String,
    pub quote: String,
    pub status: i32,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
