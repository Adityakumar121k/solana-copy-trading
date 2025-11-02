CREATE TABLE trades
(
    id              BIGSERIAL       NOT NULL,
    position_id     BIGINT          NOT NULL,
    target_trade_id BIGINT,
    wallet          VARCHAR         NOT NULL,
    signature       VARCHAR         NOT NULL,
    action          VARCHAR         NOT NULL,
    status          VARCHAR         NOT NULL,
    mint            VARCHAR         NOT NULL,
    amount          DECIMAL(18, 9)  NOT NULL,
    price           DECIMAL(18, 15) NOT NULL,
    trade_fee       DECIMAL(18, 15) NOT NULL,
    tx_fee          DECIMAL(18, 15) NOT NULL,
    amm             VARCHAR         NOT NULL,
    slot            INTEGER         NOT NULL,
    created_at      TIMESTAMP(3)    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP(3)    NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT trades_pkey PRIMARY KEY (id)
);

CREATE UNIQUE INDEX trades_signature_key ON trades (signature);