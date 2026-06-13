-- Migration 003: Inventori milik owner (bukan lahan), Lahan punya crop_type

-- 1. Inventori: lepas dari land, jadi milik owner langsung
--    (land_id jadi optional reference/catatan penggunaan, bukan required)
ALTER TABLE inventories ADD COLUMN IF NOT EXISTS owner_id UUID REFERENCES users(id);
ALTER TABLE inventories ALTER COLUMN land_id DROP NOT NULL;

-- Backfill owner_id dari land
UPDATE inventories
SET owner_id = lands.owner_id
FROM lands
WHERE inventories.land_id = lands.id AND inventories.owner_id IS NULL;

-- Index owner
CREATE INDEX IF NOT EXISTS idx_inventories_owner_id ON inventories(owner_id);

-- 2. Lahan: tambah crop_type (komoditas tunggal dari bibit yang dipilih)
ALTER TABLE lands ADD COLUMN IF NOT EXISTS crop_type VARCHAR(100);

-- Index
CREATE INDEX IF NOT EXISTS idx_lands_crop_type ON lands(crop_type);

-- 3. Tabel junction: lahan punya banyak inventori (pupuk, bibit, alat)
CREATE TABLE IF NOT EXISTS land_inventories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    land_id UUID NOT NULL REFERENCES lands(id) ON DELETE CASCADE,
    inventory_id UUID NOT NULL REFERENCES inventories(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(land_id, inventory_id)
);

CREATE INDEX IF NOT EXISTS idx_land_inventories_land_id ON land_inventories(land_id);
CREATE INDEX IF NOT EXISTS idx_land_inventories_inventory_id ON land_inventories(inventory_id);
