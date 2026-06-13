-- Migration 004: Junction lahan-inventori + dukungan estimasi panen & treatment
-- ============================================================

-- Junction table: inventori yang dipakai di sebuah lahan
CREATE TABLE IF NOT EXISTS land_inventories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    land_id UUID NOT NULL REFERENCES lands(id) ON DELETE CASCADE,
    inventory_id UUID NOT NULL REFERENCES inventories(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE (land_id, inventory_id)
);
CREATE INDEX IF NOT EXISTS idx_land_inventories_land_id ON land_inventories(land_id);

-- harvests: tambahan untuk membedakan estimasi vs hasil nyata + kapan terakhir diperbarui
ALTER TABLE harvests ADD COLUMN IF NOT EXISTS is_estimate BOOLEAN NOT NULL DEFAULT FALSE;
ALTER TABLE harvests ADD COLUMN IF NOT EXISTS predicted_harvest_date DATE;
ALTER TABLE harvests ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ DEFAULT NOW();

-- progress_percent estimasi boleh 0 (sudah default), tidak perlu ubah constraint
