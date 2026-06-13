-- Add owner_id column to inventories table
ALTER TABLE inventories ADD COLUMN IF NOT EXISTS owner_id UUID REFERENCES users(id);

-- Backfill owner_id from the land's owner
UPDATE inventories
SET owner_id = lands.owner_id
FROM lands
WHERE inventories.land_id = lands.id AND inventories.owner_id IS NULL;

-- Make owner_id NOT NULL after backfill
ALTER TABLE inventories ALTER COLUMN owner_id SET NOT NULL;

-- Add index
CREATE INDEX IF NOT EXISTS idx_inventories_owner_id ON inventories(owner_id);
