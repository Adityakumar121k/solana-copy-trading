use diesel::deserialize::FromSql;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{IsNull, Output, ToSql};
use diesel::sql_types::VarChar;
use diesel::{AsExpression, FromSqlRow};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    AsExpression,
    FromSqlRow,
    serde::Serialize,
    serde::Deserialize,
)]
#[diesel(sql_type = VarChar)]
pub enum PositionStatus {
    Opened,
    Closed,
}

impl ToSql<VarChar, Pg> for PositionStatus {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> diesel::serialize::Result {
        let s = match self {
            PositionStatus::Opened => "opened",
            PositionStatus::Closed => "closed",
        };
        use std::io::Write;
        out.write_all(s.as_bytes())?;
        Ok(IsNull::No)
    }
}

impl FromSql<VarChar, Pg> for PositionStatus {
    fn from_sql(bytes: PgValue<'_>) -> diesel::deserialize::Result<Self> {
        match std::str::from_utf8(bytes.as_bytes())? {
            "opened" => Ok(PositionStatus::Opened),
            "closed" => Ok(PositionStatus::Closed),
            other => Err(format!("Unknown PositionStatus: {other}").into()),
        }
    }
}
