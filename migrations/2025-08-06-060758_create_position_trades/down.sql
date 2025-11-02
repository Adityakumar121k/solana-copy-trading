ALTER TABLE "position_trades" DROP CONSTRAINT IF EXISTS "position_trades_B_fkey";
ALTER TABLE "position_trades" DROP CONSTRAINT IF EXISTS "position_trades_A_fkey";
DROP INDEX IF EXISTS "position_trades_B_index";
DROP TABLE IF EXISTS "position_trades";