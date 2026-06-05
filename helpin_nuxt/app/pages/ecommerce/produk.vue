<template>
  <div class="min-h-screen bg-kopSurface flex flex-col font-sans">
    <header class="fixed w-full top-0 z-40 bg-white/80 backdrop-blur-md border-b border-kopLight/50 transition-all duration-300">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 h-16 flex items-center justify-between">
        <NuxtLink to="../">
            <div class="flex items-center cursor-pointer">
                <img src="/assets/helpin_light_logo.png" alt="Logo Helpin Services" class="h-10 w-auto" />
            </div>
        </NuxtLink>
        
        <nav class="hidden md:flex gap-8 font-medium text-sm text-gray-600">
          <a href="beranda" class="hover:text-kopPrimary transition-colors">Beranda</a>
          <a href="produk" class="text-kopPrimary font-semibold transition-colors">Produk</a>
          <a href="pesanan" class="hover:text-kopPrimary transition-colors">Pesanan</a>
        </nav>

        <div class="flex items-center gap-4">
          <button @click="openCart" class="p-2 text-gray-600 hover:text-kopPrimary transition-colors relative group">
            <Icon name="lucide:shopping-cart" class="text-xl group-hover:scale-110 transition-transform" />
            <span v-if="cartTotalItems > 0" class="absolute top-0 right-0 bg-orange-500 text-white text-[10px] font-bold w-4 h-4 rounded-full flex items-center justify-center animate-pulse-once">
              {{ cartTotalItems }}
            </span>
          </button>
          <button class="hidden sm:flex bg-kopPrimary hover:bg-kopDark text-white px-5 py-2 rounded-full text-sm font-semibold transition-all hover:shadow-glow active:scale-95">
            Masuk
          </button>
        </div>
      </div>
    </header>

    <main class="flex-grow pt-24 pb-16">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        
        <div class="mb-8" v-motion-fade-visible-once>
          <nav class="flex text-sm text-gray-500 mb-3" aria-label="Breadcrumb">
            <ol class="inline-flex items-center space-x-1 md:space-x-3">
              <li class="inline-flex items-center hover:text-kopPrimary cursor-pointer transition-colors">Beranda</li>
              <li><Icon name="lucide:chevron-right" class="text-gray-400 text-sm" /></li>
              <li class="text-kopDark font-semibold">Produk</li>
            </ol>
          </nav>
          <h1 class="text-3xl md:text-4xl font-bold text-kopDark tracking-tight">Eksplorasi Hasil Bumi</h1>
          <p class="text-gray-500 mt-2">Pilih kategori dan temukan produk terbaik langsung dari tangan pertama.</p>
        </div>

        <div class="flex flex-col lg:flex-row gap-8">
          
          <aside class="lg:w-1/4 flex-shrink-0 space-y-6">
            <div class="bg-white p-5 rounded-2xl shadow-sm border border-gray-100" v-motion :initial="{ opacity: 0, x: -20 }" :enter="{ opacity: 1, x: 0, transition: { duration: 400 } }">
              <h3 class="font-semibold text-kopDark mb-3">Cari Produk</h3>
              <div class="relative">
                <input 
                  v-model="searchQuery" 
                  type="text" 
                  placeholder="Misal: Beras, Susu..." 
                  class="w-full pl-10 pr-4 py-2.5 bg-kopSurface border border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-kopPrimary/50 focus:border-kopPrimary text-sm transition-all"
                >
                <Icon name="lucide:search" class="absolute left-3.5 top-3 text-gray-400" />
              </div>
            </div>

            <div class="bg-white p-5 rounded-2xl shadow-sm border border-gray-100" v-motion :initial="{ opacity: 0, x: -20 }" :enter="{ opacity: 1, x: 0, transition: { duration: 400, delay: 100 } }">
              <h3 class="font-semibold text-kopDark mb-4">Kategori</h3>
              <ul class="space-y-2">
                <li>
                  <button 
                    @click="activeCategory = null"
                    :class="['w-full flex items-center justify-between px-3 py-2 rounded-lg text-sm transition-colors', activeCategory === null ? 'bg-kopPrimary text-white font-medium shadow-soft' : 'text-gray-600 hover:bg-kopLight/50']"
                  >
                    <span class="flex items-center gap-2"><Icon name="lucide:layout-grid" /> Semua Produk</span>
                    <span v-if="activeCategory === null" class="bg-white/20 px-2 py-0.5 rounded text-xs">{{ productData.length }}</span>
                  </button>
                </li>
                <li v-for="cat in categoryData" :key="cat.id">
                  <button 
                    @click="activeCategory = cat.id"
                    :class="['w-full flex items-center justify-between px-3 py-2 rounded-lg text-sm transition-colors', activeCategory === cat.id ? 'bg-kopPrimary text-white font-medium shadow-soft' : 'text-gray-600 hover:bg-kopLight/50']"
                  >
                    <span class="flex items-center gap-2"><Icon :name="cat.icon" /> {{ cat.name }}</span>
                  </button>
                </li>
              </ul>
            </div>
          </aside>

          <div class="lg:w-3/4">
            <div class="flex flex-col sm:flex-row justify-between items-center mb-6 gap-4 bg-white p-4 rounded-xl shadow-sm border border-gray-100">
              <div class="text-sm text-gray-500">
                Menampilkan <span class="font-bold text-kopDark">{{ filteredProducts.length }}</span> produk
              </div>
              <div class="flex items-center gap-3">
                <span class="text-sm text-gray-500">Urutkan:</span>
                <select v-model="sortBy" class="bg-kopSurface border border-gray-200 text-gray-700 text-sm rounded-lg focus:ring-kopPrimary focus:border-kopPrimary block p-2 outline-none">
                  <option value="newest">Terbaru</option>
                  <option value="price_asc">Harga: Rendah ke Tinggi</option>
                  <option value="price_desc">Harga: Tinggi ke Rendah</option>
                </select>
              </div>
            </div>

            <div v-if="filteredProducts.length === 0" class="bg-white rounded-2xl border border-gray-100 p-12 text-center flex flex-col items-center">
              <Icon name="lucide:package-open" class="text-6xl text-gray-300 mb-4" />
              <h3 class="text-lg font-bold text-kopDark">Produk tidak ditemukan</h3>
              <p class="text-gray-500 mt-2">Coba gunakan kata kunci pencarian atau kategori lain.</p>
              <button @click="resetFilters" class="mt-6 text-kopPrimary font-medium hover:underline">Reset Semua Filter</button>
            </div>

            <div v-else class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
              <div 
                v-for="(product, index) in filteredProducts" :key="product.id"
                @click="openProductModal(product)"
                v-motion :initial="{ opacity: 0, scale: 0.95 }" :enter="{ opacity: 1, scale: 1, transition: { duration: 300 } }"
                class="bg-white rounded-2xl p-4 shadow-sm border border-gray-100 group flex flex-col h-full hover:shadow-soft transition-all cursor-pointer"
              >
                <div class="h-48 rounded-xl bg-kopSurface relative overflow-hidden mb-4">
                  <img :src="product.image" :alt="product.name" class="w-full h-full object-cover mix-blend-multiply group-hover:scale-110 transition-transform duration-500" />
                  <div class="absolute top-3 left-3 bg-white/90 backdrop-blur-sm text-[10px] font-bold px-2 py-1 rounded text-kopDark uppercase tracking-wide">
                    {{ product.type }}
                  </div>
                  <div class="absolute inset-0 bg-kopDark/10 opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center">
                    <button class="bg-white text-kopPrimary w-10 h-10 rounded-full flex items-center justify-center shadow-lg hover:bg-kopPrimary hover:text-white transition-colors translate-y-4 group-hover:translate-y-0 duration-300">
                      <Icon name="lucide:eye" class="text-xl" />
                    </button>
                  </div>
                </div>
                
                <div class="flex-grow flex flex-col">
                  <div class="flex items-center gap-1 text-xs text-gray-400 mb-2">
                    <Icon name="lucide:map-pin" class="text-[10px]" /> {{ product.location }}
                  </div>
                  <h3 class="font-semibold text-kopDark text-base leading-tight mb-1 group-hover:text-kopPrimary transition-colors">{{ product.name }}</h3>
                  <p class="text-xs text-kopPrimary font-medium mb-4">{{ product.seller }}</p>
                  
                  <div class="mt-auto flex items-center justify-between pt-3 border-t border-gray-50">
                    <span class="font-bold text-lg text-kopDark">{{ formatRupiah(product.price) }}</span>
                    <span class="text-xs text-gray-400">/ {{ product.unit }}</span>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </main>

    <footer class="bg-kopDark text-white py-12 mt-auto">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 text-center md:text-left flex flex-col md:flex-row justify-between items-center gap-4">
        <div class="flex items-center gap-2">
          <Icon name="lucide:leaf" class="text-2xl text-kopLight" />
          <span class="text-xl font-bold tracking-tight">Helpin<span class="text-kopLight">Kop</span></span>
        </div>
        <p class="text-kopLight/60 text-sm">&copy; 2026 Helpin Koperasi. Tata kelola modern untuk petani dan peternak.</p>
      </div>
    </footer>

    <ClientOnly>
      <Teleport to="body">
        <Transition name="fade">
          <div v-if="isProductModalOpen" class="fixed inset-0 z-[100] flex items-center justify-center p-4 sm:p-6 md:p-12">
            <div class="absolute inset-0 bg-kopDark/60 backdrop-blur-sm" @click="closeProductModal"></div>
            <div class="relative bg-white rounded-3xl shadow-2xl w-full max-w-4xl overflow-hidden flex flex-col md:flex-row z-10 max-h-[90vh]">
              <button @click="closeProductModal" class="absolute top-4 right-4 z-20 bg-white/80 hover:bg-red-500 hover:text-white text-gray-600 rounded-full p-2 backdrop-blur-md transition-all shadow-sm">
                <Icon name="lucide:x" class="text-xl block" />
              </button>

              <div class="md:w-1/2 h-64 md:h-auto bg-kopSurface relative flex-shrink-0">
                <img :src="selectedProduct?.image" class="w-full h-full object-cover mix-blend-multiply p-4" />
                <div class="absolute top-6 left-6 bg-white/90 backdrop-blur-sm text-xs font-bold px-3 py-1.5 rounded-lg text-kopDark uppercase tracking-wide shadow-sm">
                  {{ selectedProduct?.type }}
                </div>
              </div>

              <div class="md:w-1/2 p-6 md:p-8 flex flex-col overflow-y-auto">
                <h2 class="text-2xl md:text-3xl font-bold text-kopDark mb-4">{{ selectedProduct?.name }}</h2>
                <div class="flex flex-wrap items-center gap-4 text-sm text-gray-600 mb-6 pb-6 border-b border-gray-100">
                  <span class="flex items-center gap-1.5 font-medium text-kopPrimary"><Icon name="lucide:store" class="text-lg" /> {{ selectedProduct?.seller }}</span>
                  <span class="flex items-center gap-1.5"><Icon name="lucide:map-pin" class="text-lg text-gray-400" /> {{ selectedProduct?.location }}</span>
                </div>
                <div class="text-3xl font-bold text-kopDark mb-6">
                  {{ formatRupiah(selectedProduct?.price || 0) }} <span class="text-base font-normal text-gray-400">/ {{ selectedProduct?.unit }}</span>
                </div>
                <div class="mb-8 flex-grow">
                  <h3 class="font-semibold text-gray-800 mb-3 text-lg">Deskripsi Produk</h3>
                  <p class="text-gray-600 text-sm leading-relaxed">{{ selectedProduct?.description }}</p>
                </div>
                
                <div class="flex flex-col sm:flex-row items-center gap-4 mt-auto pt-4">
                  <div class="flex items-center justify-between border border-gray-200 rounded-full px-2 py-1 w-full sm:w-32 bg-gray-50/50">
                    <button @click="selectedQty > 1 ? selectedQty-- : null" class="w-10 h-10 flex items-center justify-center text-gray-500 hover:text-kopPrimary" :disabled="selectedQty <= 1"><Icon name="lucide:minus" /></button>
                    <span class="font-semibold text-kopDark">{{ selectedQty }}</span>
                    <button @click="selectedQty++" class="w-10 h-10 flex items-center justify-center text-gray-500 hover:text-kopPrimary"><Icon name="lucide:plus" /></button>
                  </div>
                  <button @click="addToCart(selectedProduct!, selectedQty)" class="flex-grow w-full bg-kopPrimary hover:bg-kopDark text-white font-semibold py-3.5 px-6 rounded-full flex items-center justify-center gap-2 transition-all shadow-soft active:scale-[0.98]">
                    <Icon name="lucide:shopping-cart" class="text-xl" /> Tambah ke Keranjang
                  </button>
                </div>
              </div>
            </div>
          </div>
        </Transition>
      </Teleport>
    </ClientOnly>

    <ClientOnly>
      <Teleport to="body">
        <Transition name="fade">
          <div v-if="isCartOpen" class="fixed inset-0 z-[110] bg-kopDark/50 backdrop-blur-sm" @click="closeCart"></div>
        </Transition>
        
        <Transition name="slide-right">
          <div v-if="isCartOpen" class="fixed top-0 right-0 h-full w-full sm:w-[400px] bg-white shadow-2xl z-[120] flex flex-col">
            <div class="px-6 py-5 border-b border-gray-100 flex items-center justify-between bg-kopSurface">
              <h2 class="text-xl font-bold text-kopDark flex items-center gap-2">
                <Icon name="lucide:shopping-bag" class="text-kopPrimary" /> Keranjang Saya
              </h2>
              <button @click="closeCart" class="text-gray-400 hover:text-red-500 hover:bg-red-50 p-2 rounded-full transition-colors">
                <Icon name="lucide:x" class="text-xl block" />
              </button>
            </div>

            <div class="flex-grow overflow-y-auto p-6">
              <div v-if="cart.length === 0" class="h-full flex flex-col items-center justify-center text-center opacity-60">
                <Icon name="lucide:shopping-cart" class="text-6xl text-gray-300 mb-4" />
                <p class="text-gray-500 font-medium">Keranjang masih kosong.</p>
                <p class="text-sm text-gray-400">Mari mulai belanja hasil bumi terbaik.</p>
              </div>

              <div v-else class="space-y-6">
                <div v-for="item in cart" :key="item.product.id" class="flex gap-4 group">
                  <div class="w-20 h-20 rounded-xl bg-kopSurface flex-shrink-0 overflow-hidden border border-gray-100">
                    <img :src="item.product.image" class="w-full h-full object-cover mix-blend-multiply" />
                  </div>
                  
                  <div class="flex-grow flex flex-col justify-between">
                    <div class="flex justify-between items-start">
                      <div>
                        <h4 class="font-semibold text-kopDark text-sm leading-tight">{{ item.product.name }}</h4>
                        <p class="text-xs text-gray-400 mt-0.5">{{ formatRupiah(item.product.price) }} / {{ item.product.unit }}</p>
                      </div>
                      <button @click="removeFromCart(item.product.id)" class="text-gray-300 hover:text-red-500 transition-colors px-1">
                        <Icon name="lucide:trash-2" class="text-lg" />
                      </button>
                    </div>
                    
                    <div class="flex items-center justify-between mt-2">
                      <div class="flex items-center bg-gray-50 rounded-lg border border-gray-100">
                        <button @click="updateCartQty(item.product.id, -1)" class="w-7 h-7 flex items-center justify-center text-gray-500 hover:text-kopPrimary disabled:opacity-30" :disabled="item.quantity <= 1">-</button>
                        <span class="text-xs font-semibold w-6 text-center">{{ item.quantity }}</span>
                        <button @click="updateCartQty(item.product.id, 1)" class="w-7 h-7 flex items-center justify-center text-gray-500 hover:text-kopPrimary">+</button>
                      </div>
                      <span class="font-bold text-kopPrimary text-sm">{{ formatRupiah(item.product.price * item.quantity) }}</span>
                    </div>
                  </div>
                </div>
              </div>
            </div>

            <div v-if="cart.length > 0" class="border-t border-gray-100 p-6 bg-white shadow-[0_-10px_20px_-10px_rgba(0,0,0,0.05)]">
              <div class="flex items-center justify-between mb-4 text-sm">
                <span class="text-gray-500">Total Belanja ({{ cartTotalItems }} barang)</span>
                <span class="font-bold text-xl text-kopDark">{{ formatRupiah(cartTotalPrice) }}</span>
              </div>
              <button class="w-full bg-kopDark hover:bg-kopPrimary text-white font-bold py-4 rounded-xl transition-colors shadow-lg active:scale-[0.98]">
                Lanjut ke Pembayaran
              </button>
            </div>
          </div>
        </Transition>
      </Teleport>
    </ClientOnly>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'

// ==========================================
// 1. ARSITEKTUR DATA
// ==========================================
interface ICategory {
  id: string;
  name: string;
  icon: string;
  itemCount: number;
}

interface IProduct {
  id: number;
  categoryId: string; // Relasi ke Kategori
  name: string;
  price: number;
  unit: string;
  type: string;
  seller: string;
  location: string;
  image: string;
  description: string;
  createdAt: Date; // Untuk pengurutan "Terbaru"
}

// Nantinya state keranjang ini disimpan ke tabel 'queue_keranjang' secara persisten di database
interface ICartItem {
  product: IProduct;
  quantity: number;
}

// ==========================================
// 2. MOCK API DATA
// ==========================================
const categoryData = ref<ICategory[]>([
  { id: 'c1', name: 'Sayur Mayur', icon: 'lucide:carrot', itemCount: 124 },
  { id: 'c2', name: 'Hasil Ternak', icon: 'lucide:beef', itemCount: 85 },
  { id: 'c3', name: 'Bibit & Pupuk', icon: 'lucide:sprout', itemCount: 42 },
  { id: 'c4', name: 'Alat Tani', icon: 'lucide:tractor', itemCount: 18 },
])

const productData = ref<IProduct[]>([
  { id: 1, categoryId: 'c1', name: 'Tomat Cherry Segar Organik', price: 18500, unit: '500g', type: 'Pertanian', seller: 'Kop. Makmur Jaya', location: 'Samarinda', image: 'https://images.unsplash.com/photo-1592924357228-91a4daadcfea?q=80&w=400&auto=format&fit=crop', description: 'Tomat cherry organik ditanam tanpa pestisida kimia. Dipanen pagi hari.', createdAt: new Date('2026-06-05T08:00:00') },
  { id: 2, categoryId: 'c2', name: 'Telur Ayam Kampung Asli', price: 32000, unit: '1 Kg', type: 'Peternakan', seller: 'Pak Budi Farm', location: 'Tenggarong', image: 'https://images.unsplash.com/photo-1587486913049-53fc88980cfc?q=80&w=400&auto=format&fit=crop', description: 'Telur ayam kampung dari ayam yang diumbar bebas. Kaya omega-3.', createdAt: new Date('2026-06-04T10:00:00') },
  { id: 3, categoryId: 'c1', name: 'Beras Merah Pulen Kualitas 1', price: 75000, unit: '5 Kg', type: 'Pertanian', seller: 'Kelompok Tani Harapan', location: 'Kutai Kartanegara', image: 'https://images.unsplash.com/photo-1586201375761-83865001e31c?q=80&w=400&auto=format&fit=crop', description: 'Beras merah pilihan dengan indeks glikemik rendah. Tekstur pulen.', createdAt: new Date('2026-06-03T14:00:00') },
  { id: 4, categoryId: 'c2', name: 'Susu Sapi Murni Pasteurisasi', price: 15000, unit: '1 Liter', type: 'Peternakan', seller: 'Sukamaju Dairy', location: 'Balikpapan', image: 'https://images.unsplash.com/photo-1550583724-b2692b85b150?q=80&w=400&auto=format&fit=crop', description: 'Susu sapi perah segar tanpa bahan pengawet. Pasteurisasi modern.', createdAt: new Date('2026-06-05T06:00:00') },
])

// ==========================================
// 3. LOGIKA FILTER & PENCARIAN (Manual Programmatic)
// ==========================================
const searchQuery = ref('')
const activeCategory = ref<string | null>(null)
const sortBy = ref('newest')

const resetFilters = () => {
  searchQuery.value = ''
  activeCategory.value = null
  sortBy.value = 'newest'
}

// Logika terprogram manual untuk filtering array produk
const filteredProducts = computed(() => {
  let result = productData.value

  // 1. Filter Kategori
  if (activeCategory.value !== null) {
    result = result.filter(p => p.categoryId === activeCategory.value)
  }

  // 2. Filter Pencarian Teks
  if (searchQuery.value.trim() !== '') {
    const query = searchQuery.value.toLowerCase()
    result = result.filter(p => p.name.toLowerCase().includes(query))
  }

  // 3. Sorting
  return result.slice().sort((a, b) => {
    if (sortBy.value === 'price_asc') return a.price - b.price
    if (sortBy.value === 'price_desc') return b.price - a.price
    // Default: newest
    return b.createdAt.getTime() - a.createdAt.getTime()
  })
})

// ==========================================
// 4. LOGIKA MODAL DETAIL PRODUK
// ==========================================
const isProductModalOpen = ref(false)
const selectedProduct = ref<IProduct | null>(null)
const selectedQty = ref(1)

const toggleBodyScroll = (lock: boolean) => {
  if (typeof document !== 'undefined') {
    document.body.style.overflow = lock ? 'hidden' : 'auto'
  }
}

const openProductModal = (product: IProduct) => {
  selectedProduct.value = product
  selectedQty.value = 1 
  isProductModalOpen.value = true
  toggleBodyScroll(true)
}

const closeProductModal = () => {
  isProductModalOpen.value = false
  toggleBodyScroll(false)
  setTimeout(() => { selectedProduct.value = null }, 300)
}

// ==========================================
// 5. LOGIKA KERANJANG
// ==========================================
const cart = ref<ICartItem[]>([])
const isCartOpen = ref(false)

const openCart = () => {
  isCartOpen.value = true
  toggleBodyScroll(true)
}

const closeCart = () => {
  isCartOpen.value = false
  toggleBodyScroll(false)
}

const addToCart = (product: IProduct, qty: number) => {
  const existingItem = cart.value.find(item => item.product.id === product.id)
  if (existingItem) {
    existingItem.quantity += qty
  } else {
    cart.value.push({ product, quantity: qty })
  }
  closeProductModal()
  openCart()
}

const updateCartQty = (productId: number, change: number) => {
  const item = cart.value.find(item => item.product.id === productId)
  if (item) {
    const newQty = item.quantity + change
    if (newQty > 0) item.quantity = newQty
  }
}

const removeFromCart = (productId: number) => {
  cart.value = cart.value.filter(item => item.product.id !== productId)
}

const cartTotalItems = computed(() => cart.value.reduce((total, item) => total + item.quantity, 0))
const cartTotalPrice = computed(() => cart.value.reduce((total, item) => total + (item.product.price * item.quantity), 0))

// ==========================================
// 6. UTILS
// ==========================================
const formatRupiah = (price: number) => {
  return new Intl.NumberFormat('id-ID', {
    style: 'currency', currency: 'IDR', minimumFractionDigits: 0
  }).format(price)
}
</script>

<style>
/* Style Transisi Modal & Drawer */
.fade-enter-active,
.fade-leave-active { transition: opacity 0.3s ease; }
.fade-enter-from,
.fade-leave-to { opacity: 0; }

.fade-enter-active .relative.bg-white,
.fade-leave-active .relative.bg-white { transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1); }
.fade-enter-from .relative.bg-white,
.fade-leave-to .relative.bg-white { opacity: 0; transform: scale(0.95) translateY(10px); }

.slide-right-enter-active,
.slide-right-leave-active { transition: transform 0.4s cubic-bezier(0.4, 0, 0.2, 1); }
.slide-right-enter-from,
.slide-right-leave-to { transform: translateX(100%); }

@keyframes pulse-once {
  0% { transform: scale(1); }
  50% { transform: scale(1.3); }
  100% { transform: scale(1); }
}
.animate-pulse-once { animation: pulse-once 0.3s ease-in-out; }
</style>