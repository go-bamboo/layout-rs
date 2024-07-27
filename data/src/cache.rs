use std::collections::BTreeMap;
use std::fmt::format;

use crate::domain::KlineEvent;
use api::keys::{key_depth_ask_h, key_depth_ask_z, key_depth_bid_h, key_depth_bid_z};
use ecode::Result;
use redis::aio::PubSub;
use redis::streams::StreamReadOptions;
use redis::streams::StreamReadReply;
use redis::AsyncCommands;
use rust_decimal::Decimal;

const ASK_SCRIPT1: &str = r"local zkey = KEYS[1]
local hkey = KEYS[2]
local priceList = redis.call('ZRANGE', zkey, 0, 3)
local ret = {}
if #priceList > 0 then
    for _, v in ipairs(priceList) do
        local quantity = redis.call('HGET', hkey, v)
        local it = {v, quantity}
        table.insert(ret, it)
    end
end
return ret";

const BID_SCRIPT1: &str = r"local zkey = KEYS[1]
local hkey = KEYS[2]
local priceList = redis.call('ZRANGE', zkey, 0, 3, 'REV')
local ret = {}
if #priceList > 0 then
    for _, v in ipairs(priceList) do
        local quantity = redis.call('HGET', hkey, v)
        local it = {v, quantity}
        table.insert(ret, it)
    end
end
return ret";

#[derive(Debug)]
pub struct RedisQuery {
    db: redis::Client,
}

impl RedisQuery {
    pub fn new() -> Result<Self> {
        let client = redis::Client::open("redis://127.0.0.1/")?;
        Ok(RedisQuery { db: client })
    }

    pub async fn range_ask(&mut self, market: &str, symbol: &str) -> Result<Vec<Vec<Decimal>>> {
        let s = symbol.to_uppercase();
        let zkey = key_depth_ask_z(market, &s);
        let hkey = key_depth_ask_h(market, &s);

        let script = redis::Script::new(ASK_SCRIPT1);
        let result: Vec<Vec<f64>> = script.key(zkey).key(hkey).invoke(&mut self.db)?;
        let mut list: Vec<Vec<Decimal>> = vec![];
        for it in result {
            let first = it[0];
            let seconde = it[1];
            let new_it = vec![
                Decimal::from_f64_retain(first).unwrap(),
                Decimal::from_f64_retain(seconde).unwrap(),
            ];
            list.push(new_it);
        }

        // tracing::info!("zrange {:?}", result);
        Ok(list)
    }

    pub async fn range_bid(&mut self, market: &str, symbol: &str) -> Result<Vec<Vec<Decimal>>> {
        let s = symbol.to_uppercase();
        let zkey = key_depth_bid_z(market, &s);
        let hkey = key_depth_bid_h(market, &s);

        let script = redis::Script::new(BID_SCRIPT1);
        let result: Vec<Vec<f64>> = script.key(zkey).key(hkey).invoke(&mut self.db)?;

        // tracing::info!("zrange {:?}", result);

        let mut list: Vec<Vec<Decimal>> = vec![];
        for it in result {
            let first = it[0];
            let seconde = it[1];
            let new_it = vec![
                Decimal::from_f64_retain(first).unwrap(),
                Decimal::from_f64_retain(seconde).unwrap(),
            ];
            list.push(new_it);
        }
        Ok(list)
    }

    pub async fn into_pubsub(&self) -> Result<PubSub> {
        let conn = self.db.get_async_connection().await?;
        Ok(conn.into_pubsub())
    }

    pub async fn xadd_kline(&self, key: &str, kline: KlineEvent) -> Result<()> {
        let mut conn = self.db.get_async_connection().await?;
        let mut map: BTreeMap<&str, String> = BTreeMap::new();
        map.insert("id", format!("{}", kline.id));
        map.insert("event", kline.event);
        conn.xadd_map(&[key], &["*"], map).await?;
        Ok(())
    }

    pub async fn xadd_force_order(&self, kline: api::event::EventForceOrder) -> Result<()> {
        let key = api::keys::key_ta_force_order(&kline.exchange, &kline.market, &kline.symbol);
        let mut conn = self.db.get_async_connection().await?;
        let mut map: BTreeMap<&str, String> = BTreeMap::new();
        map.insert("exchange", format!("{}", kline.exchange));
        map.insert("market", kline.market);
        map.insert("symbol", kline.symbol);
        map.insert("base", kline.base);
        map.insert("quote", kline.quote);
        map.insert(
            "total_buy_quantity",
            format!("{}", kline.total_buy_quantity),
        );
        map.insert(
            "total_sell_quantity",
            format!("{}", kline.total_sell_quantity),
        );
        conn.xadd_maxlen_map(
            &[key],
            redis::streams::StreamMaxlen::Approx(100),
            &["*"],
            map,
        )
            .await?;
        Ok(())
    }

    pub async fn mkgroup(&self, key: &str) -> Result<()> {
        let mut conn = self.db.get_async_connection().await?;
        conn.xgroup_create_mkstream(&[key], &["group-1"], "$")
            .await?;
        Ok(())
    }

    pub async fn into_stream(&self, key: &str) -> Result<StreamReadReply> {
        let mut conn = self.db.get_async_connection().await?;
        let opts = StreamReadOptions::default()
            .group("group-1", "consumer-1")
            .block(100)
            .count(200);
        // .noack();
        let res: StreamReadReply = conn.xread_options(&[key], &[">"], &opts).await?;
        Ok(res)
    }

    pub async fn xack(&self, key: &str, id: String) -> Result<()> {
        let mut conn = self.db.get_async_connection().await?;
        conn.xack(&[key], &["group-1"], &[id]).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_xgroup() {
        // Tests the following commands....
        // xgroup_create_mkstream
        // xgroup_destroy
        // xgroup_delconsumer

        // let con = RedisQuery::new().unwrap();

        // test xgroup create w/ mkstream @ 0
        // let mut biz = KlineEvent::default();
        // biz.id = 1;
        // let result = con.xadd_kline(biz).await;
        // assert!(result.is_ok());

        // group
        // let result = con.mkgroup().await;
        // print!("-------1 {:?}\n", result);
        // assert!(result.is_ok());

        // destroy this new stream group
        // let result = con.into_stream().await;
        // print!("-------1 {:?}\n", result);
        // assert!(result.is_ok());
    }
}
