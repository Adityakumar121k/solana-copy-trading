use crate::entities::position::types::PositionStatus;
use crate::modules::builder::config::POSITION_CLOSE_PERCENT;
use rust_decimal::Decimal;

impl PositionStatus {
    pub fn predict_from_amounts(amount_left: Decimal, amount_total: Decimal) -> Self {
        if amount_total.is_zero() {
            return PositionStatus::Closed;
        }

        let lhs = amount_left.max(Decimal::ZERO) * Decimal::from(100);
        let rhs = amount_total * *POSITION_CLOSE_PERCENT;

        if lhs < rhs {
            PositionStatus::Closed
        } else {
            PositionStatus::Opened
        }
    }
}
