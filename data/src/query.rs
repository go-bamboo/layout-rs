use std::sync::Arc;

use async_trait::async_trait;
use sea_orm::*;

use ecode::Result;
use crate::QueryMarketDao;
use crate::entity::{cex_market, cex_market_symbol, prelude::CexMarket, prelude::CexMarketSymbol};

#[derive(Debug)]
pub struct Query {
    db: DatabaseConnection,
}

impl Query {
    pub async fn new(c: &crate::Conf) -> Result<Query> {
        let url = format!("{}://{}", c.database.driver, c.database.source);
        let mut opt = ConnectOptions::new(url);
        opt.sqlx_logging(false);
        opt.sqlx_logging_level(log::LevelFilter::Warn);
        let db = Database::connect(opt).await?;
        Ok(Query { db })
    }
}

#[async_trait]
impl QueryMarketDao for Query {
    async fn batch_get_market_by_status(&self) -> Result<Vec<cex_market::Model>> {
        let res: Vec<cex_market::Model> = CexMarket::find()
            .filter(cex_market::Column::Status.eq(1))
            .all(&self.db)
            .await?;
        Ok(res)
    }

    async fn batch_get_market_symbol_by_status(
        &self,
        exchange: &str,
        market: &str,
    ) -> Result<Vec<cex_market_symbol::Model>> {
        let res: Vec<cex_market_symbol::Model> = CexMarketSymbol::find()
            .filter(cex_market_symbol::Column::Exchange.eq(exchange))
            .filter(cex_market_symbol::Column::Market.eq(market))
            .filter(cex_market_symbol::Column::Status.eq(1))
            .all(&self.db)
            .await?;
        Ok(res)
    }
}
