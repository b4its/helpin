-- Migration 006: Ternak lebih fleksibel
-- Jenis ternak bebas (unggas, mamalia, serangga, dll), gender & weight opsional.
-- ============================================================

-- Lepas batasan kategori & gender lama
ALTER TABLE livestock DROP CONSTRAINT IF EXISTS livestock_category_check;
ALTER TABLE livestock DROP CONSTRAINT IF EXISTS livestock_gender_check;

-- Perlebar kolom kategori untuk jenis ternak yang lebih bervariasi
ALTER TABLE livestock ALTER COLUMN category TYPE VARCHAR(50);

-- Gender & weight tidak lagi wajib
ALTER TABLE livestock ALTER COLUMN gender DROP NOT NULL;
ALTER TABLE livestock ALTER COLUMN weight DROP NOT NULL;
