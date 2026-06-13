-- Migration 008: Panel Admin (karyawan, supplier, produk harga beli)
-- ============================================================

-- Izinkan role 'karyawan' (karyawan = user dengan role karyawan)
ALTER TABLE users DROP CONSTRAINT IF EXISTS users_role_check;
ALTER TABLE users ADD COLUMN IF NOT EXISTS phone VARCHAR(50);

-- Supplier (data fix → PostgreSQL)
CREATE TABLE IF NOT EXISTS suppliers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(200) NOT NULL,
    contact VARCHAR(100),
    commodity VARCHAR(150),
    address TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Produk: harga beli (acuan pengeluaran), expired, tanggal masuk
ALTER TABLE products ADD COLUMN IF NOT EXISTS purchase_price BIGINT DEFAULT 0;
ALTER TABLE products ADD COLUMN IF NOT EXISTS expired_at DATE;
ALTER TABLE products ADD COLUMN IF NOT EXISTS entry_date DATE DEFAULT CURRENT_DATE;
