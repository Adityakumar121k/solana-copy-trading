use crate::entities::trade::entity::{NewTrade, Trade};
use crate::kernel::db::connection::Db;
use crate::schema::trades;
use anyhow::{Context, Result};
use diesel::SelectableHelper;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

#[derive(Clone)]
pub struct TradeRepository;

impl TradeRepository {
    pub fn create(data: &NewTrade) -> Result<Trade> {
        diesel::insert_into(trades::table)
            .values(data)
            .returning(Trade::as_returning())
            .get_result(&mut Db::get_connection())
            .with_context(|| format!("Failed trade create\ndata = {data:#?}"))
    }

    pub fn get_all_by_position_id(position_id: i64) -> Result<Vec<Trade>> {
        trades::table
            .filter(trades::position_id.eq(position_id))
            .order(trades::id.desc())
            .select(Trade::as_select())
            .load(&mut Db::get_connection())
            .context("Failed get all trades by position id")
    }
}
