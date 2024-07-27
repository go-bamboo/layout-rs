//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "cex_binance_kline_csv")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub event: String,
    pub symbol: String,
    pub interval: String,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
