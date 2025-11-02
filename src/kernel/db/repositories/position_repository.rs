use crate::entities::position::entity::{NewPosition, Position, UpdatePosition};
use crate::entities::position::types::PositionStatus;
use crate::kernel::db::connection::Db;
use crate::schema::positions;
use anyhow::{Context, Result};
use diesel::ExpressionMethods;
use diesel::{BoolExpressionMethods, OptionalExtension, QueryDsl};
use diesel::{RunQueryDsl, SelectableHelper};

#[derive(Clone)]
pub struct PositionRepository;

impl PositionRepository {
    pub fn create(data: &NewPosition) -> Result<Position> {
        diesel::insert_into(positions::table)
            .values(data)
            .returning(Position::as_returning())
            .get_result(&mut Db::get_connection())
            .with_context(|| format!("Failed position create\ndata = {data:#?}"))
    }

    pub fn update(id: i64, data: &UpdatePosition) -> Result<Position> {
        diesel::update(positions::table.find(id))
            .set(data)
            .returning(Position::as_returning())
            .get_result(&mut Db::get_connection())
            .with_context(|| format!("Failed position update\nid = {id}\ndata = {data:#?}"))
    }

    pub fn get_all_opened() -> Result<Vec<Position>> {
        positions::table
            .filter(positions::status.eq(PositionStatus::Opened))
            .select(Position::as_select())
            .order(positions::id.desc())
            .load(&mut Db::get_connection())
            .context("PositionRepository::get_all_opened")
    }

    pub fn get_opened(mint: &str, wallet: &str) -> Result<Option<Position>> {
        let filter = positions::mint
            .eq(mint)
            .and(positions::wallet.eq(wallet))
            .and(positions::status.eq(PositionStatus::Opened));

        positions::table
            .filter(filter)
            .select(Position::as_select())
            .order(positions::id.desc())
            .first(&mut Db::get_connection())
            .optional()
            .context("PositionRepository::get_all_opened")
    }
}
