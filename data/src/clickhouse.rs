use std::str::FromStr;
use async_trait::async_trait;
use chrono::{DateTime, TimeDelta, Utc};
use clickhouse_rs::{row, types::Complex, Block, Pool};
use futures_util::StreamExt;
use rust_decimal::Decimal;
use ecode::Result;

use crate::{
    domain::{ForceOrderEvent, KlineEvent, AggTrade},
    entity::{cex_market, cex_market_symbol},
};

#[derive(Debug)]
pub struct ClickhouseQuery {
    db: Pool,
}

impl ClickhouseQuery {
    pub fn new() -> Self {
        let database_url =
            "tcp://default:123456@localhost:9300/helloworld?compression=lz4&ping_timeout=42ms";
        let pool = Pool::new(database_url);
        ClickhouseQuery { db: pool }
    }
}

#[async_trait]
impl crate::AggTradeRepo for ClickhouseQuery {
    async fn fetch_agg_trade_limit(
        &self,
        symbol: &cex_market_symbol::Model,
        limit: u32,
    ) -> Result<Vec<AggTrade>> {
        let table = format!(
            "{}_{}_aggtrade_{}",
            symbol.exchange, symbol.market, symbol.symbol
        )
            .to_ascii_lowercase();
        let sql = format!("SELECT * FROM {} order by id desc LIMIT 0,{}", table, limit);
        let mut client = self.db.get_handle().await?;
        let mut stream = client.query(sql).stream();
        let mut res = vec![];
        while let Some(Ok(row)) = stream.next().await {
            let mut ev = AggTrade::default();
            ev.time = row.get("time").unwrap();
            ev.event = row.get("event").unwrap();
            ev.symbol = row.get("symbol").unwrap();
            ev.first_trade_id = row.get("first_trade_id").unwrap();
            ev.last_trade_id = row.get("last_trade_id").unwrap();
            res.push(ev);
        }
        Ok(res)
    }

    async fn fetch_all(&self, symbol: &str, interval: u32) -> Result<Block<Complex>> {
        let mut sql = format!(
            "SELECT close FROM kline_1m WHERE match(symbol,'{}')",
            symbol
        );
        if symbol == "ETHUSDT" {
            sql = format!(
                "SELECT close FROM kline_ethusdt_1m WHERE id / 60000 % {} = 0",
                interval * 60
            );
        }
        let mut client = self.db.get_handle().await?;
        let block = client.query(sql).fetch_all().await?;
        Ok(block)
    }
}

#[async_trait]
impl crate::KlineRepo for ClickhouseQuery {
    async fn fetch_kline_limit(
        &self,
        table: &str,
        limit: u32,
    ) -> Result<Vec<KlineEvent>> {
        let sql = format!("SELECT * FROM {} order by id desc LIMIT 0,{}", table, limit);
        let mut client = self.db.get_handle().await?;
        let mut stream = client.query(sql).stream();
        let mut res = vec![];
        while let Some(Ok(row)) = stream.next().await {
            let mut ev = KlineEvent::default();
            ev.id = row.get("id").unwrap();
            ev.event = row.get("event").unwrap();
            ev.symbol = row.get("symbol").unwrap();
            ev.start_time = row.get("start_time").unwrap();
            ev.end_time = row.get("end_time").unwrap();
            ev.interval = row.get("interval")?;
            ev.first_trade_id = row.get("first_trade_id").unwrap();
            ev.last_trade_id = row.get("last_trade_id").unwrap();
            ev.open = row.get("open").unwrap();
            ev.close = row.get("close").unwrap();
            ev.high = row.get("high")?;
            ev.low = row.get("low")?;
            ev.volume = row.get("volume")?;
            ev.trade_num = row.get("trade_num")?;
            ev.quote_volume = row.get("quote_volume")?;
            ev.active_buy_volume = row.get("active_buy_volume")?;
            ev.active_buy_quote_volume = row.get("active_buy_volume")?;
            ev.ema7 = row.get("ema7")?;
            ev.ema25 = row.get("ema25")?;
            ev.macd = row.get("macd")?;
            ev.rsi = row.get("rsi")?;
            res.push(ev);
        }
        Ok(res)
    }

    async fn fetch_kline_time_limit(
        &self,
        table: &str,
        start: &DateTime<Utc>,
        to: &DateTime<Utc>,
    ) -> Result<Vec<KlineEvent>> {
        let sql = format!("SELECT * FROM {} WHERE id > {} AND id < {} order by id desc", table, start.timestamp_millis(), to.timestamp_millis());
        let mut client = self.db.get_handle().await?;
        let mut stream = client.query(sql).stream();
        let mut res = vec![];
        while let Some(Ok(row)) = stream.next().await {
            let mut ev = KlineEvent::default();
            ev.id = row.get("id").unwrap();
            ev.event = row.get("event").unwrap();
            ev.symbol = row.get("symbol").unwrap();
            ev.start_time = row.get("start_time").unwrap();
            ev.end_time = row.get("end_time").unwrap();
            ev.interval = row.get("interval")?;
            ev.first_trade_id = row.get("first_trade_id").unwrap();
            ev.last_trade_id = row.get("last_trade_id").unwrap();
            ev.open = row.get("open").unwrap();
            ev.close = row.get("close").unwrap();
            ev.high = row.get("high")?;
            ev.low = row.get("low")?;
            ev.volume = row.get("volume")?;
            ev.trade_num = row.get("trade_num")?;
            ev.quote_volume = row.get("quote_volume")?;
            ev.active_buy_volume = row.get("active_buy_volume")?;
            ev.active_buy_quote_volume = row.get("active_buy_volume")?;
            ev.ema7 = row.get("ema7")?;
            ev.ema25 = row.get("ema25")?;
            ev.macd = row.get("macd")?;
            ev.rsi = row.get("rsi")?;
            res.push(ev);
        }
        Ok(res)
    }

    async fn fetch_kline_time_limit_open(
        &self,
        table: &str,
        start: &DateTime<Utc>,
        to: &DateTime<Utc>,
    ) -> Result<Decimal> {
        let sql = format!("SELECT `open` FROM {} WHERE id >= {} AND id < {} ORDER BY id LIMIT 1; ", table, start.timestamp_millis(), to.timestamp_millis());
        let mut client = self.db.get_handle().await?;
        let mut block = client.query(sql).fetch_all().await?;
        let open: clickhouse_rs::types::Decimal = block.get(1, "open").unwrap();
        let res = Decimal::from_str(open.to_string().as_str())?;
        Ok(res)
    }

    async fn fetch_kline_time_limit_high(
        &self,
        table: &str,
        start: &DateTime<Utc>,
        to: &DateTime<Utc>,
    ) -> Result<Decimal> {
        let sql = format!("SELECT MAX(`high`) as high FROM {} WHERE id >= {} AND id < {} ORDER BY id LIMIT 1; ", table, start.timestamp_millis(), to.timestamp_millis());
        let mut client = self.db.get_handle().await?;
        let mut block = client.query(sql).fetch_all().await?;
        let open: clickhouse_rs::types::Decimal = block.get(1, "high").unwrap();
        let res = Decimal::from_str(open.to_string().as_str())?;
        Ok(res)
    }

    async fn fetch_kline_time_limit_low(
        &self,
        table: &str,
        start: &DateTime<Utc>,
        to: &DateTime<Utc>,
    ) -> Result<Decimal> {
        let sql = format!("SELECT MIN(`low`) as low FROM {} WHERE id >= {} AND id < {} ORDER BY id LIMIT 1; ", table, start.timestamp_millis(), to.timestamp_millis());
        let mut client = self.db.get_handle().await?;
        let mut block = client.query(sql).fetch_all().await?;
        let open: clickhouse_rs::types::Decimal = block.get(1, "low").unwrap();
        let res = Decimal::from_str(open.to_string().as_str())?;
        Ok(res)
    }

    async fn fetch_kline_time_limit_close(
        &self,
        table: &str,
        start: &DateTime<Utc>,
        to: &DateTime<Utc>,
    ) -> Result<Decimal> {
        let sql = format!("SELECT `close` FROM {} WHERE id >= {} AND id < {} ORDER BY DESC id LIMIT 1; ", table, start.timestamp_millis(), to.timestamp_millis());
        let mut client = self.db.get_handle().await?;
        let mut block = client.query(sql).fetch_all().await?;
        let open: clickhouse_rs::types::Decimal = block.get(1, "close").unwrap();
        let res = Decimal::from_str(open.to_string().as_str())?;
        Ok(res)
    }

    async fn insert_kline(&self, table: &str, kline: KlineEvent) -> Result<()> {
        log::info!("{:?}", kline);
        let mut block = Block::new();
        let _ = block.push(row! {
            id: kline.id,
            event: kline.event,
            symbol: kline.symbol,
            start_time: kline.start_time,
            end_time: kline.end_time,
            interval: kline.interval,
            first_trade_id: kline.first_trade_id,
            last_trade_id: kline.last_trade_id,
            open: kline.open,
            close: kline.close,
            high: kline.high,
            low: kline.low,
            volume: kline.volume,
            trade_num: kline.trade_num,
            quote_volume: kline.quote_volume,
            active_buy_volume: kline.active_buy_volume,
            active_buy_quote_volume: kline.active_buy_quote_volume,
            ema7: kline.ema7,
            ema25: kline.ema25,
            macd: kline.macd,
            rsi: kline.rsi,
        })?;
        let mut client = self.db.get_handle().await?;
        Ok(client.insert(table, block).await?)
    }
}

#[async_trait]
impl crate::ForceOrderRepo for ClickhouseQuery {
    async fn fetch_force_order_limit(
        &self,
        symbol: &cex_market_symbol::Model,
        side: &str,
        start: &DateTime<Utc>,
    ) -> Result<Decimal> {
        let table = format!(
            "{}_{}_liquidation_order_{}",
            symbol.exchange, symbol.market, symbol.symbol
        ).to_ascii_lowercase();
        let sql = format!(
            "SELECT SUM(avg_value) as sum_value FROM {} WHERE id > {} AND side = '{}'",
            table, start.timestamp_micros(), side
        );
        let mut client = self.db.get_handle().await?;
        let mut blk = client.query(sql).fetch_all().await?;
        let sum_value_s : String = blk.get(0, "sum_value")?;
        let v = Decimal::from_str(sum_value_s.as_str())?;
        Ok(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let now = Utc::now();
        let ts: i64 = now.timestamp_millis();
        log::info!("ts: {}", &ts);
        println!("ts: {}", &ts);
        let pre = ts - TimeDelta::minutes(10).num_milliseconds();
        println!("pre: {}", pre)
    }
}
