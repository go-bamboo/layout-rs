use std::{format, vec};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use clickhouse_rs::Block;
use clickhouse_rs::types::Complex;
use serde::{Deserialize, Serialize};
use ecode::Result;
use crate::{
    domain::{AggTrade, ForceOrderEvent, KlineEvent},
    entity::{cex_market, cex_market_symbol},
};

pub mod entity;
pub mod clickhouse;
pub mod id;
pub mod model;
pub mod query;
pub mod cache;
pub mod domain;

pub use validator::Validate;
pub use redis::{
    streams::{StreamId, StreamKey, StreamReadReply},
    FromRedisValue,
};
pub use rust_decimal::Decimal;
pub use rust_decimal_macros::dec;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Database {
    pub driver: String,
    pub source: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Conf {
    pub database: Database,
}

#[async_trait]
pub trait QueryMarketDao {
    async fn batch_get_market_by_status(&self) -> Result<Vec<cex_market::Model>>;
    async fn batch_get_market_symbol_by_status(
        &self,
        exchange: &str,
        market: &str,
    ) -> ecode::Result<Vec<cex_market_symbol::Model>>;
}

#[async_trait]
pub trait AggTradeRepo {
    async fn fetch_agg_trade_limit(
        &self,
        symbol: &cex_market_symbol::Model,
        limit: u32,
    ) -> Result<Vec<AggTrade>>;

    async fn fetch_all(&self, symbol: &str, interval: u32) -> Result<Block<Complex>>;
}

#[async_trait]
pub trait KlineRepo {
    async fn fetch_kline_limit(
        &self,
        table: &str,
        limit: u32,
    ) -> Result<Vec<KlineEvent>>;

    async fn fetch_kline_time_limit(
        &self,
        table: &str,
        start: &DateTime<Utc>,
        to: &DateTime<Utc>,
    ) -> Result<Vec<KlineEvent>>;
    async fn fetch_kline_time_limit_open(
        &self,
        table: &str,
        start: &DateTime<Utc>,
        to: &DateTime<Utc>,
    ) -> Result<Decimal>;

    async fn fetch_kline_time_limit_high(
        &self,
        table: &str,
        start: &DateTime<Utc>,
        to: &DateTime<Utc>,
    ) -> Result<Decimal>;
    async fn fetch_kline_time_limit_low(
        &self,
        table: &str,
        start: &DateTime<Utc>,
        to: &DateTime<Utc>,
    ) -> Result<Decimal>;
    async fn fetch_kline_time_limit_close(
        &self,
        table: &str,
        start: &DateTime<Utc>,
        to: &DateTime<Utc>,
    ) -> Result<Decimal>;

    async fn insert_kline(&self, table: &str, kline: KlineEvent) -> Result<()>;
}

#[async_trait]
pub trait ForceOrderRepo {
    async fn fetch_force_order_limit(
        &self,
        symbol: &cex_market_symbol::Model,
        side: &str,
        start: &DateTime<Utc>,
    ) -> Result<Decimal>;
}


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
