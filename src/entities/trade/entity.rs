use crate::entities::trade::types::{TradeAction, TradeStatus};
use crate::schema::trades;
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, Selectable};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Selectable, Queryable)]
pub struct Trade {
    pub id: i64,
    pub position_id: i64,
    pub target_trade_id: Option<i64>,
    pub wallet: String,
    pub signature: String,
    pub action: TradeAction,
    pub status: TradeStatus,
    pub mint: String,
    pub amount: Decimal,
    pub price: Decimal,
    pub trade_fee: Decimal,
    pub tx_fee: Decimal,
    pub amm: String,
    pub slot: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Clone, Debug, Deserialize, Insertable)]
#[diesel(table_name = trades)]
pub struct NewTrade {
    pub position_id: i64,
    pub target_trade_id: Option<i64>,
    pub wallet: String,
    pub signature: String,
    pub action: TradeAction,
    pub status: TradeStatus,
    pub mint: String,
    pub amount: Decimal,
    pub price: Decimal,
    pub trade_fee: Decimal,
    pub tx_fee: Decimal,
    pub amm: String,
    pub slot: i32,
}
