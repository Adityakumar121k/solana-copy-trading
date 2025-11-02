// @generated automatically by Diesel CLI.

diesel::table! {
    position_trades (A, B) {
        A -> Int8,
        B -> Int8,
    }
}

diesel::table! {
    positions (id) {
        id -> Int8,
        target_position_id -> Nullable<Int8>,
        wallet -> Varchar,
        mint -> Varchar,
        amount_total -> Numeric,
        amount_left -> Numeric,
        avg_buy_price -> Numeric,
        avg_sell_price -> Nullable<Numeric>,
        total_fee -> Numeric,
        realized_pnl -> Nullable<Numeric>,
        amm -> Varchar,
        status -> Varchar,
        created_at -> Timestamp,
        closed_at -> Nullable<Timestamp>,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    trades (id) {
        id -> Int8,
        position_id -> Int8,
        target_trade_id -> Nullable<Int8>,
        wallet -> Varchar,
        signature -> Varchar,
        action -> Varchar,
        status -> Varchar,
        mint -> Varchar,
        amount -> Numeric,
        price -> Numeric,
        trade_fee -> Numeric,
        tx_fee -> Numeric,
        amm -> Varchar,
        slot -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(position_trades -> positions (A));
diesel::joinable!(position_trades -> trades (B));

diesel::allow_tables_to_appear_in_same_query!(position_trades, positions, trades, users,);
