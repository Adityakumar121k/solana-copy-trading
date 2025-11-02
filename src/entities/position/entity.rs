use crate::entities::position::types::PositionStatus;
use crate::schema::positions;
use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Selectable, Queryable)]
#[diesel(table_name = positions)]
pub struct Position {
    pub id: i64,
    pub target_position_id: Option<i64>,
    pub wallet: String,
    pub mint: String,
    pub amount_total: Decimal,
    pub amount_left: Decimal,
    pub avg_buy_price: Decimal,
    pub avg_sell_price: Option<Decimal>,
    pub total_fee: Decimal,
    pub realized_pnl: Option<Decimal>,
    pub amm: String,
    pub status: PositionStatus,
    pub created_at: NaiveDateTime,
    pub closed_at: Option<NaiveDateTime>,
    pub updated_at: NaiveDateTime,
}

#[derive(Clone, Debug, Deserialize, Insertable)]
#[diesel(table_name = positions)]
pub struct NewPosition {
    pub target_position_id: Option<i64>,
    pub wallet: String,
    pub mint: String,
    pub amount_total: Decimal,
    pub amount_left: Decimal,
    pub avg_buy_price: Decimal,
    pub avg_sell_price: Option<Decimal>,
    pub total_fee: Decimal,
    pub realized_pnl: Option<Decimal>,
    pub amm: String,
    pub status: PositionStatus,
}

#[derive(Clone, Debug, Default, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = positions)]
pub struct UpdatePosition {
    pub target_position_id: Option<i64>,
    pub wallet: Option<String>,
    pub mint: Option<String>,
    pub amount_total: Option<Decimal>,
    pub amount_left: Option<Decimal>,
    pub avg_buy_price: Option<Decimal>,
    pub avg_sell_price: Option<Decimal>,
    pub total_fee: Option<Decimal>,
    pub realized_pnl: Option<Decimal>,
    pub amm: Option<String>,
    pub status: Option<PositionStatus>,
    pub closed_at: Option<NaiveDateTime>,
}
