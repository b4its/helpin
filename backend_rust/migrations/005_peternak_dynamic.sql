-- Migration 005: Fitur dinamis panel peternak
-- ============================================================

-- Usia ternak (dalam bulan)
ALTER TABLE livestock ADD COLUMN IF NOT EXISTS age_months INTEGER;

-- Kondisi kandang (di-generate otomatis oleh backend dari okupansi & kesehatan)
ALTER TABLE pens ADD COLUMN IF NOT EXISTS condition VARCHAR(50);

-- Kandungan/nutrisi produk (untuk produk kategori Pakan) — fleksibel JSONB
ALTER TABLE products ADD COLUMN IF NOT EXISTS nutrition JSONB;
