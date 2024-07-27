use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Write};
use std::net::AddrParseError;
use clickhouse_rs::errors::Error;
use polars::error::PolarsError;
use redis::RedisError;
use sea_orm::{DbErr, TransactionError};
use ta::errors::TaError;
use thiserror::Error as ThisError;
use tokio::task::{JoinError, JoinHandle};
use tonic::Code;

pub type Result<T, E = Status> = std::result::Result<T, E>;

// PublicError is public, but opaque and easy to keep compatible.
#[derive(ThisError, Debug)]
pub struct Status {
    pub code: i32,
    pub reason: Reason,
    pub message: String,
    pub metadata: HashMap<String, String>,
}

#[derive(PartialEq, Debug)]
pub enum Reason {
    IoError,
    AddrParseError,
    RedisError,
    DBError,
    DBTransactionError,
    ClickhouseError,
    HyperError,
    TonicError,
    PolarsError,
    TaError,
    AnyhowError,

    NotFoundKline,
    BoxErr,
    PriceZero,
    BalanceNotEnough,
    BinanceSpotInvalidApiSecret,
    CheckOrderFailed { id: i64 },
    AssetNotFoundError,
    SellFailed,
    BuyFailed,
}

impl PartialEq for Status {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code && self.reason == other.reason
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.message.as_str())
    }
}

impl From<std::io::Error> for Status {
    fn from(value: std::io::Error) -> Self {
        Status {
            code: 500,
            reason: Reason::IoError,
            message: value.to_string(),
            metadata: Default::default(),
        }
    }
}

impl From<AddrParseError> for Status {
    fn from(value: AddrParseError) -> Self {
        Status {
            code: 500,
            reason: Reason::ClickhouseError,
            message: value.to_string(),
            metadata: Default::default(),
        }
    }
}

impl From<redis::RedisError> for Status {
    fn from(value: RedisError) -> Self {
        Status {
            code: 500,
            reason: Reason::RedisError,
            message: value.to_string(),
            metadata: Default::default(),
        }
    }
}

impl From<sea_orm::DbErr> for Status {
    fn from(value: DbErr) -> Self {
        Status {
            code: 500,
            reason: Reason::DBError,
            message: value.to_string(),
            metadata: Default::default(),
        }
    }
}

impl From<sea_orm::TransactionError<sea_orm::DbErr>> for Status {
    fn from(value: TransactionError<DbErr>) -> Self {
        Status {
            code: 500,
            reason: Reason::DBTransactionError,
            message: value.to_string(),
            metadata: Default::default(),
        }
    }
}

impl From<clickhouse_rs::errors::Error> for Status {
    fn from(value: Error) -> Self {
        Status {
            code: 500,
            reason: Reason::ClickhouseError,
            message: value.to_string(),
            metadata: Default::default(),
        }
    }
}



impl From<hyper::Error> for Status {
    fn from(value: hyper::Error) -> Self {
        Status {
            code: 500,
            reason: Reason::HyperError,
            message: value.to_string(),
            metadata: Default::default(),
        }
    }
}

impl From<tonic::transport::Error> for Status {
    fn from(value: tonic::transport::Error) -> Self {
        Status {
            code: 500,
            reason: Reason::TonicError,
            message: value.to_string(),
            metadata: Default::default(),
        }
    }
}

impl From<PolarsError> for Status {
    fn from(value: PolarsError) -> Self {
        Status {
            code: 500,
            reason: Reason::PolarsError,
            message: value.to_string(),
            metadata: Default::default(),
        }
    }
}


impl From<TaError> for Status {
    fn from(value: TaError) -> Self {
        Status {
            code: 500,
            reason: Reason::TaError,
            message: value.to_string(),
            metadata: Default::default(),
        }
    }
}

impl From<anyhow::Error> for Status {
    fn from(value: anyhow::Error) -> Self {
        Status {
            code: 500,
            reason: Reason::AnyhowError,
            message: value.to_string(),
            metadata: Default::default(),
        }
    }
}

impl From<rust_decimal::Error> for Status {
    fn from(value: rust_decimal::Error) -> Self {
        Status {
            code: 500,
            reason: Reason::AnyhowError,
            message: value.to_string(),
            metadata: Default::default(),
        }
    }
}

impl From<tonic::Status> for Status {
    fn from(value: tonic::Status) -> Self {
        Status {
            code: 500,
            reason: Reason::TonicError,
            message: value.to_string(),
            metadata: Default::default(),
        }
    }
}

impl Into<tonic::Status> for Status {
    fn into(self) -> tonic::Status {
        tonic::Status::new(Code::Unknown, self.message)
    }
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
