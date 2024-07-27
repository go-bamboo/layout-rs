//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "cex_bot_position")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub bot_id: i64,
    pub isolated: i16,
    pub leverage: String,
    pub initial_margin: String,
    pub maint_margin: String,
    pub open_order_initial_margin: String,
    pub position_initial_margin: String,
    pub symbol: String,
    pub unrealized_profit: String,
    pub entry_price: String,
    pub max_notional: String,
    pub side: String,
    pub amount: String,
    pub notional: String,
    pub bid_notional: String,
    pub ask_notional: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}