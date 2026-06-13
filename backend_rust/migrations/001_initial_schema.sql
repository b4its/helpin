-- HELPIN Backend Initial Schema Migration
-- All tables ordered by foreign key dependencies

-- ============================================================
-- 1. Users & Authentication
-- ============================================================
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    name VARCHAR(255) NOT NULL,
    password_hash TEXT NOT NULL,
    role VARCHAR(50) NOT NULL CHECK (role IN ('admin', 'supplier', 'pembeli', 'petani', 'peternak')),
    private_key TEXT,  -- Encrypted private key for crypto signing
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- ============================================================
-- 2. Pens (Kandang) — depends on users
-- ============================================================
CREATE TABLE pens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    owner_id UUID NOT NULL REFERENCES users(id),
    name VARCHAR(100) NOT NULL,
    capacity INTEGER NOT NULL,
    pen_type VARCHAR(50) NOT NULL,
    location TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- ============================================================
-- 3. Livestock (Ternak) — depends on users, pens
-- ============================================================
CREATE TABLE livestock (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    owner_id UUID NOT NULL REFERENCES users(id),
    tag_id VARCHAR(50) UNIQUE NOT NULL,
    category VARCHAR(20) NOT NULL CHECK (category IN ('Sapi', 'Kambing', 'Domba')),
    breed VARCHAR(100) NOT NULL,
    weight DECIMAL(10,2) NOT NULL,
    gender VARCHAR(10) NOT NULL CHECK (gender IN ('Jantan', 'Betina')),
    health_status VARCHAR(20) DEFAULT 'Sehat',
    health_score INTEGER DEFAULT 100 CHECK (health_score BETWEEN 0 AND 100),
    pen_id UUID REFERENCES pens(id),
    entry_date DATE NOT NULL DEFAULT CURRENT_DATE,
    biometrics JSONB DEFAULT '[]',
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- ============================================================
-- 4. Health Records — depends on livestock
-- ============================================================
CREATE TABLE health_records (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    livestock_id UUID NOT NULL REFERENCES livestock(id) ON DELETE CASCADE,
    symptoms TEXT,
    body_temp DECIMAL(4,1),
    heart_rate INTEGER,
    respiratory_rate INTEGER,
    notes TEXT,
    health_score INTEGER,
    recorded_at TIMESTAMPTZ DEFAULT NOW()
);

-- ============================================================
-- 5. Feed Recommendations (ML Audit Log) — depends on livestock
-- ============================================================
CREATE TABLE feed_recommendations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    livestock_id UUID REFERENCES livestock(id),
    tag_id VARCHAR(50) NOT NULL,
    breed VARCHAR(100) NOT NULL,
    weight DECIMAL(10,2) NOT NULL,
    feed_name VARCHAR(200),
    efficiency DECIMAL(5,2),
    digestibility DECIMAL(5,2),
    gain_prediction DECIMAL(5,2),
    nutrition JSONB NOT NULL,
    ingredients JSONB NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- ============================================================
-- 6. Lands (Lahan) — depends on users
-- ============================================================
CREATE TABLE lands (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    owner_id UUID NOT NULL REFERENCES users(id),
    code VARCHAR(20) UNIQUE NOT NULL,  -- Format: LHN-XXX
    name VARCHAR(200) NOT NULL,
    area_hectare DECIMAL(10,2) NOT NULL CHECK (area_hectare > 0),
    soil_type VARCHAR(50) NOT NULL,
    status VARCHAR(50) NOT NULL CHECK (status IN ('Aktif Ditanami', 'Persiapan', 'Masa Bera')),
    crop_type VARCHAR(100),  -- komoditas utama lahan
    location TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- ============================================================
-- 7. Inventories — depends on lands and users
-- ============================================================
CREATE TABLE inventories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    owner_id UUID REFERENCES users(id),  -- pemilik inventori
    land_id UUID REFERENCES lands(id) ON DELETE CASCADE,  -- lahan (opsional)
    category VARCHAR(100) NOT NULL,
    name VARCHAR(200) NOT NULL,
    quantity DECIMAL(10,2) NOT NULL,
    unit VARCHAR(50) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- ============================================================
-- 8. Plants (Tanaman) — depends on lands
-- ============================================================
CREATE TABLE plants (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    land_id UUID NOT NULL REFERENCES lands(id) ON DELETE CASCADE,
    plant_type VARCHAR(100) NOT NULL,
    plant_date DATE NOT NULL,
    estimated_harvest DATE,
    actual_harvest DATE,
    status VARCHAR(50) DEFAULT 'Ditanam' CHECK (status IN ('Ditanam', 'Tumbuh', 'Panen', 'Selesai')),
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- ============================================================
-- 9. Harvests (Hasil Panen) — depends on plants, lands
-- ============================================================
CREATE TABLE harvests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    plant_id UUID NOT NULL REFERENCES plants(id),
    land_id UUID NOT NULL REFERENCES lands(id),
    quantity DECIMAL(10,2) NOT NULL,
    unit VARCHAR(50) NOT NULL,
    quality_grade VARCHAR(10),
    progress_percent INTEGER DEFAULT 0 CHECK (progress_percent IN (0, 50, 100)),
    harvested_at TIMESTAMPTZ DEFAULT NOW()
);

-- ============================================================
-- 10. Categories (E-Commerce)
-- ============================================================
CREATE TABLE categories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    icon VARCHAR(100),
    item_count INTEGER DEFAULT 0
);

-- ============================================================
-- 11. Products (E-Commerce) — depends on users, categories
-- ============================================================
CREATE TABLE products (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    seller_id UUID NOT NULL REFERENCES users(id),
    category_id UUID REFERENCES categories(id),
    name VARCHAR(200) NOT NULL,
    price BIGINT NOT NULL CHECK (price > 0),
    stock INTEGER NOT NULL DEFAULT 0,
    unit VARCHAR(50) NOT NULL,
    product_type VARCHAR(50),
    location VARCHAR(200),
    image_url TEXT,
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- ============================================================
-- 12. Cart Items — depends on users, products
-- ============================================================
CREATE TABLE cart_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    product_id UUID NOT NULL REFERENCES products(id),
    quantity INTEGER NOT NULL CHECK (quantity > 0),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE(user_id, product_id)
);

-- ============================================================
-- 13. Orders — depends on users
-- ============================================================
CREATE TABLE orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    buyer_id UUID NOT NULL REFERENCES users(id),
    status VARCHAR(50) DEFAULT 'Menunggu Pembayaran' CHECK (status IN ('Menunggu Pembayaran', 'Diproses', 'Dikirim', 'Selesai', 'Dibatalkan')),
    total_amount BIGINT NOT NULL,
    shipping_address TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- ============================================================
-- 14. Order Items — depends on orders, products
-- ============================================================
CREATE TABLE order_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    order_id UUID NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    product_id UUID NOT NULL REFERENCES products(id),
    quantity INTEGER NOT NULL,
    price_at_purchase BIGINT NOT NULL
);

-- ============================================================
-- 15. POS Transactions (Kasir) — depends on users
-- ============================================================
CREATE TABLE pos_transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    cashier_id UUID NOT NULL REFERENCES users(id),
    items JSONB NOT NULL,
    subtotal BIGINT NOT NULL,
    tax BIGINT NOT NULL,
    total BIGINT NOT NULL,
    amount_tendered BIGINT NOT NULL,
    change_amount BIGINT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- ============================================================
-- 16. Financial Records (Kas)
-- ============================================================
CREATE TABLE financial_records (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    record_type VARCHAR(20) NOT NULL CHECK (record_type IN ('pemasukan', 'pengeluaran')),
    amount BIGINT NOT NULL,
    category VARCHAR(100),
    description TEXT,
    reference_id UUID,  -- Links to pos_transaction or order
    balance_after BIGINT NOT NULL,
    recorded_at TIMESTAMPTZ DEFAULT NOW()
);

-- ============================================================
-- 17. Offline Sync Queue — depends on users
-- ============================================================
CREATE TABLE sync_queue (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    payload JSONB NOT NULL,
    signature TEXT NOT NULL,  -- Cryptographic signature
    status VARCHAR(20) DEFAULT 'pending' CHECK (status IN ('pending', 'processing', 'synced', 'failed')),
    error_message TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    synced_at TIMESTAMPTZ
);

-- ============================================================
-- 18. Blockchain Transaction Log
-- ============================================================
CREATE TABLE blockchain_transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    reference_type VARCHAR(50) NOT NULL,  -- 'pos_transaction', 'order'
    reference_id UUID NOT NULL,
    tx_hash VARCHAR(66),  -- Ethereum-style hash
    block_number BIGINT,
    status VARCHAR(20) DEFAULT 'pending' CHECK (status IN ('pending', 'confirmed', 'failed')),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    confirmed_at TIMESTAMPTZ
);

-- ============================================================
-- INDEXES for commonly queried columns
-- ============================================================
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_livestock_owner_id ON livestock(owner_id);
CREATE INDEX idx_livestock_tag_id ON livestock(tag_id);
CREATE INDEX idx_lands_owner_id ON lands(owner_id);
CREATE INDEX idx_lands_code ON lands(code);
CREATE INDEX idx_products_seller_id ON products(seller_id);
CREATE INDEX idx_products_category_id ON products(category_id);
CREATE INDEX idx_sync_queue_user_id ON sync_queue(user_id);
CREATE INDEX idx_sync_queue_status ON sync_queue(status);
CREATE INDEX idx_blockchain_transactions_reference_id ON blockchain_transactions(reference_id);
CREATE INDEX idx_blockchain_transactions_status ON blockchain_transactions(status);
