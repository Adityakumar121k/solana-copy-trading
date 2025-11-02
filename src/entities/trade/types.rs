use diesel::deserialize::FromSql;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::sql_types::Text;
use diesel::{AsExpression, FromSqlRow};
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow, Serialize, Deserialize)]
#[diesel(sql_type = Text)]
pub enum TradeAction {
    Buy,
    Sell,
}

impl ToSql<Text, Pg> for TradeAction {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        let s = match self {
            TradeAction::Buy => "buy",
            TradeAction::Sell => "sell",
        };
        out.write_all(s.as_bytes())?;
        Ok(IsNull::No)
    }
}

impl FromSql<Text, Pg> for TradeAction {
    fn from_sql(bytes: PgValue<'_>) -> diesel::deserialize::Result<Self> {
        match std::str::from_utf8(bytes.as_bytes())? {
            "buy" => Ok(TradeAction::Buy),
            "sell" => Ok(TradeAction::Sell),
            other => Err(format!("Unknown TradeAction: {other}").into()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow, Serialize, Deserialize)]
#[diesel(sql_type = Text)]
pub enum TradeStatus {
    Success,
    Failed,
}

impl ToSql<Text, Pg> for TradeStatus {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        let s = match self {
            TradeStatus::Success => "success",
            TradeStatus::Failed => "failed",
        };
        out.write_all(s.as_bytes())?;
        Ok(IsNull::No)
    }
}

impl FromSql<Text, Pg> for TradeStatus {
    fn from_sql(bytes: PgValue<'_>) -> diesel::deserialize::Result<Self> {
        match std::str::from_utf8(bytes.as_bytes())? {
            "success" => Ok(TradeStatus::Success),
            "failed" => Ok(TradeStatus::Failed),
            other => Err(format!("Unknown TradeStatus: {other}").into()),
        }
    }
}
