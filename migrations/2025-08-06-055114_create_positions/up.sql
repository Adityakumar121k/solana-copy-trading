CREATE TABLE positions
(
    id                 BIGSERIAL       NOT NULL,
    target_position_id BIGINT,
    wallet             VARCHAR         NOT NULL,
    mint               VARCHAR         NOT NULL,
    amount_total       DECIMAL(19, 9)  NOT NULL,
    amount_left        DECIMAL(19, 9)  NOT NULL,
    avg_buy_price      DECIMAL(18, 15) NOT NULL,
    avg_sell_price     DECIMAL(18, 15),
    total_fee          DECIMAL(18, 15) NOT NULL,
    realized_pnl       DECIMAL(18, 15),
    amm                VARCHAR         NOT NULL,
    status             VARCHAR         NOT NULL,
    created_at         TIMESTAMP(3)    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    closed_at          TIMESTAMP(3),
    updated_at         TIMESTAMP(3)    NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT positions_pkey PRIMARY KEY (id)
);