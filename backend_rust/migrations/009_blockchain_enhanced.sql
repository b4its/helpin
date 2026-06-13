-- Migration 009: Blockchain Enhanced Tracking
-- Memperluas tabel blockchain_transactions untuk mendukung Hyperledger Besu.
-- ============================================================

-- Tambah kolom untuk menyimpan data lengkap dari Besu
ALTER TABLE blockchain_transactions
    ADD COLUMN IF NOT EXISTS reference_type_detail VARCHAR(100),
    ADD COLUMN IF NOT EXISTS payload_hash          VARCHAR(66),   -- keccak256 hash
    ADD COLUMN IF NOT EXISTS contract_address      VARCHAR(42),   -- alamat smart contract
    ADD COLUMN IF NOT EXISTS from_address          VARCHAR(42),   -- wallet yang kirim tx
    ADD COLUMN IF NOT EXISTS gas_used              BIGINT,
    ADD COLUMN IF NOT EXISTS chain_id              INTEGER DEFAULT 1337,
    ADD COLUMN IF NOT EXISTS event_name            VARCHAR(100),  -- nama event yang ter-emit
    ADD COLUMN IF NOT EXISTS raw_data              JSONB;         -- data lengkap dari Besu

-- Tabel baru: activity_blockchain_log
-- Menyimpan mapping antara aktivitas MongoDB dan tx hash Besu
CREATE TABLE IF NOT EXISTS activity_blockchain_log (
    id               UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    activity_id      UUID NOT NULL,         -- UUID aktivitas (juga disimpan di MongoDB)
    action_type      VARCHAR(50) NOT NULL,  -- CREATED | UPDATED | DELETED | LOGIN | PAYMENT
    table_affected   VARCHAR(100) NOT NULL, -- nama tabel PostgreSQL
    username         VARCHAR(255),
    description      TEXT,
    blockchain_hash  VARCHAR(66),           -- tx hash dari Besu (0x...)
    old_data_hash    VARCHAR(66),           -- keccak256(old_data)
    new_data_hash    VARCHAR(66),           -- keccak256(new_data)
    block_number     BIGINT,
    ip_address       VARCHAR(50),
    created_at       TIMESTAMPTZ DEFAULT NOW()
);

-- Index untuk query cepat
CREATE INDEX IF NOT EXISTS idx_activity_blockchain_activity_id  ON activity_blockchain_log(activity_id);
CREATE INDEX IF NOT EXISTS idx_activity_blockchain_hash         ON activity_blockchain_log(blockchain_hash);
CREATE INDEX IF NOT EXISTS idx_activity_blockchain_table        ON activity_blockchain_log(table_affected);
CREATE INDEX IF NOT EXISTS idx_activity_blockchain_action       ON activity_blockchain_log(action_type);
CREATE INDEX IF NOT EXISTS idx_activity_blockchain_created_at   ON activity_blockchain_log(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_blockchain_tx_tx_hash            ON blockchain_transactions(tx_hash);
CREATE INDEX IF NOT EXISTS idx_blockchain_tx_contract           ON blockchain_transactions(contract_address);
