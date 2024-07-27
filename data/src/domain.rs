use clickhouse_rs::types::Decimal;
use validator::{Validate, ValidationError};

fn validate_decimal(v: &Decimal) -> Result<(), ValidationError> {
    Ok(())
}

#[derive(Clone, Debug, PartialEq, Default, Validate)]
pub struct AggTrade {
    #[validate(range(min = 1))]
    pub time: i64,
    #[validate(length(min = 1))]
    pub event: String,
    #[validate(length(min = 1))]
    pub symbol: String,
    #[validate(range(min = 1))]
    pub agg_trade_id: i64,
    pub price: Decimal,
    pub quantity: Decimal,
    pub first_trade_id: i64,
    pub last_trade_id: i64,
    pub trade_time: i64,
    pub maker: bool,
}

#[derive(Clone, Debug, PartialEq, Default, Validate)]
pub struct KlineEvent {
    #[validate(length(min = 1))]
    pub exchange: String,
    #[validate(length(min = 1))]
    pub market: String,
    #[validate(range(min = 1))]
    pub id: i64,
    #[validate(length(min = 1))]
    pub event: String,
    #[validate(length(min = 1))]
    pub symbol: String,
    #[validate(range(min = 1))]
    pub start_time: i64,
    #[validate(range(min = 1))]
    pub end_time: i64,
    #[validate(length(min = 1))]
    pub interval: String,
    #[validate(range(min = 1))]
    pub first_trade_id: i64,
    #[validate(range(min = 1))]
    pub last_trade_id: i64,
    pub open: Decimal,
    pub close: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub volume: Decimal,
    #[validate(range(min = 0))]
    pub trade_num: i64,
    pub quote_volume: Decimal,
    pub active_buy_volume: Decimal,
    pub active_buy_quote_volume: Decimal,
    pub ema7: Decimal,
    pub ema25: Decimal,
    pub macd: Decimal,
    pub rsi: Decimal,
}

#[derive(Clone, Debug, PartialEq, Default, Validate)]
pub struct ForceOrderEvent {
    #[validate(length(min = 1))]
    pub exchange: String,
    #[validate(length(min = 1))]
    pub market: String,
    #[validate(range(min = 1))]
    pub id: i64,
    #[validate(length(min = 1))]
    pub event: String,
    #[validate(length(min = 1))]
    pub symbol: String,
    #[validate(length(min = 1))]
    pub side: String,
    #[validate(length(min = 1))]
    pub order_type: String,
    #[validate(length(min = 1))]
    pub time_in_force: String,
    pub orig_quantity: Decimal,
    pub price: Decimal,
    pub avg_price: Decimal,
    pub order_status: String,
    pub last_filled_qty: Decimal,
    pub accumulated_filled_qty: Decimal,
    #[validate(range(min = 0))]
    pub trade_time: i64,
}
