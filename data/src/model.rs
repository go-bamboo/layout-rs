use rust_decimal::Decimal;
use rust_decimal_macros::dec;

pub const HANDLING_FEE: Decimal = dec!(0.000);

#[derive(Default, Debug, Clone)]
pub struct AOrderStatusResponse {}

#[derive(Default, Debug, Clone)]
pub struct AOrderResponse {
    pub status: AOrderStatus,
    // pub base_quantity: f64,
    pub real_base_quantity: Decimal,
    // pub quote_quantity: f64,
    pub real_quote_quantity: Decimal,
}

#[derive(Default, Debug, Clone)]
pub struct ABalance {
    pub asset: String,
    pub free: Decimal,
    pub locked: Decimal,
}

#[derive(Debug, Clone, PartialEq)]
pub enum APositionSide {
    Both,
    Long,
    Short,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum AOrderStatus {
    #[default]
    None,
    XiaDan,
    Filled,
    Canceled,
    Locked, // 针对买的订单
    Selled, // 针对买的订单
}

pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Default, Debug, Clone)]
pub struct AAsset {
    pub asset: String,
    pub free: Decimal,
    pub locked: Decimal,
}

#[derive(Default, Debug, Clone)]
pub struct ABookTicker {
    pub best_bid_price: Decimal,
    pub best_bid_qty: Decimal,
    pub best_ask_price: Decimal,
    pub best_ask_qty: Decimal,
}

pub struct ALimitOrder {
    pub price: Decimal,
    pub qty: Decimal,
}
