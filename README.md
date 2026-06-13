# Hybrid System untuk Digitalisasi Koperasi, Petani, dan Peternak dengan Menerapkan Teknologi Blockchain dan Ensemble Learning 

# Teknologi yang digunakan
- Nuxt Js
- Rust (Axum)
- Python (Fast Api)
- PostgreSQL
- MongoDB


# command run in docker compose
```bash
cd helpin_nuxt && pnpm run build && cd .. && docker compose up --build -d
```
# 1. Build frontend dulu (sekali, di luar Docker)
cd helpin_nuxt && pnpm run build && cd ..

# 2. Jalankan SEMUA services
docker compose up --build -d

# 3. Cek status
docker compose ps

# 4. Lihat logs kalau ada masalah
docker compose logs -f backend_rust
