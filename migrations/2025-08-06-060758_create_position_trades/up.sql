CREATE TABLE "position_trades"
(
    "A" BIGINT NOT NULL,
    "B" BIGINT NOT NULL,
    CONSTRAINT "position_trades_AB_pkey" PRIMARY KEY ("A", "B")
);

CREATE INDEX "position_trades_B_index" ON "position_trades" ("B");

ALTER TABLE "position_trades"
    ADD CONSTRAINT "position_trades_A_fkey"
        FOREIGN KEY ("A") REFERENCES "positions" ("id")
            ON DELETE CASCADE ON UPDATE CASCADE;

ALTER TABLE "position_trades"
    ADD CONSTRAINT "position_trades_B_fkey"
        FOREIGN KEY ("B") REFERENCES "trades" ("id")
            ON DELETE CASCADE ON UPDATE CASCADE;