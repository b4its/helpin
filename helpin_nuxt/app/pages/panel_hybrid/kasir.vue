<template>
  <div class="flex h-screen w-full bg-[#f4f7f5] font-sans overflow-hidden relative">
    
    <SidebarHybrid :isOpen="isSidebarOpen" @close="isSidebarOpen = false" activeMenu="/panel_hybrid/kasir" />

    <main class="flex-1 flex flex-col overflow-y-auto relative w-full no-scrollbar">
      
      <header class="bg-[#1a402d] text-white px-6 md:px-10 py-6 sticky top-0 z-20 shadow-md border-b-4 border-green-500 flex flex-col md:flex-row justify-between items-start md:items-center gap-4 shrink-0">
        <div class="flex items-center gap-4">
          <button @click="isSidebarOpen = true" class="md:hidden p-2 -ml-2 text-white hover:bg-white/10 rounded-lg transition">
            <MenuIcon class="w-6 h-6" />
          </button>
          <div class="w-12 h-12 bg-white rounded-full flex items-center justify-center text-[#1a402d] shadow-inner">
            <BuildingIcon class="w-6 h-6" />
          </div>
          <div>
            <h1 class="text-xl md:text-2xl font-black tracking-tight">Koperasi Agro Helpin Terpadu</h1>
            <p class="text-xs md:text-sm text-green-300 font-medium mt-0.5">Badan Hukum: 123.45/BH/KDK/2026 | Saldo Kas: Rp 124.500.000</p>
          </div>
        </div>
        <div class="flex items-center gap-3 bg-[#143222] px-4 py-2 rounded-xl border border-white/10">
          <ClockIcon class="w-5 h-5 text-green-400" />
          <div class="text-right">
            <p class="text-xs text-gray-400 uppercase font-bold tracking-wider">Shift Kasir</p>
            <p class="text-sm font-black text-white">Admin Suki SUPER</p>
          </div>
        </div>
      </header>

      <div class="flex-1 flex flex-col lg:flex-row p-4 md:p-6 gap-6 h-full overflow-hidden">
        
        <section class="flex-1 flex flex-col h-full overflow-hidden bg-white rounded-3xl shadow-sm border border-gray-100">
          <div class="p-4 border-b border-gray-100 flex gap-2 overflow-x-auto no-scrollbar shrink-0">
            <button 
              v-for="cat in categories" :key="cat.id"
              @click="activeCategory = cat.id"
              :class="[
                'px-6 py-2.5 rounded-full text-sm font-bold transition-all whitespace-nowrap border',
                activeCategory === cat.id 
                  ? 'bg-[#1a402d] text-white border-[#1a402d] shadow-md' 
                  : 'bg-gray-50 text-gray-500 border-gray-200 hover:bg-green-50 hover:text-green-700'
              ]"
            >
              {{ cat.label }}
            </button>
            <div class="ml-auto relative w-full max-w-xs shrink-0">
              <SearchIcon class="w-4 h-4 absolute left-3 top-1/2 -translate-y-1/2 text-gray-400" />
              <input 
                v-model="searchQuery" 
                type="text" 
                placeholder="Cari SKU atau Nama..." 
                class="w-full pl-9 pr-4 py-2 bg-gray-50 border border-gray-200 rounded-full text-sm focus:outline-none focus:border-green-500 focus:ring-1 focus:ring-green-500 transition-all font-medium"
              >
            </div>
          </div>

          <div class="flex-1 overflow-y-auto p-4 bg-gray-50/50">
            <div class="grid grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
              <div 
                v-for="product in filteredProducts" :key="product.id"
                @click="addToCart(product)"
                class="bg-white p-3 rounded-2xl border border-gray-100 shadow-sm hover:shadow-md hover:border-green-500 transition-all cursor-pointer group flex flex-col relative overflow-hidden"
              >
                <div class="relative w-full h-32 mb-3 rounded-xl overflow-hidden bg-gray-100 shrink-0">
                  <img :src="product.image" :alt="product.name" class="w-full h-full object-cover group-hover:scale-110 transition-transform duration-500" />
                  
                  <div class="absolute top-2 right-2 px-2 py-1 bg-white/90 backdrop-blur text-[9px] font-black uppercase tracking-wider rounded-md border border-white/20 shadow-sm"
                    :class="product.category === 'pangan' ? 'text-green-600' : 'text-orange-500'"
                  >
                    {{ product.category }}
                  </div>
                </div>
                
                <div class="flex-1 flex flex-col justify-between">
                  <div class="mb-3">
                    <span class="text-[10px] font-black uppercase tracking-wider text-gray-400 block mb-0.5">{{ product.id }}</span>
                    <h3 class="text-sm font-black text-gray-800 leading-snug group-hover:text-[#1a402d] transition-colors line-clamp-2">{{ product.name }}</h3>
                  </div>
                  <div class="mt-auto flex justify-between items-end">
                    <div>
                      <p class="text-[11px] font-bold text-gray-500 mb-0.5">Stok: <span :class="product.stock < 10 ? 'text-red-500' : 'text-green-600'">{{ product.stock }} {{ product.unit }}</span></p>
                      <p class="text-base font-black text-[#1a402d]">{{ formatRupiah(product.price) }}</p>
                    </div>
                    <div class="w-8 h-8 rounded-full bg-gray-50 flex items-center justify-center text-gray-400 group-hover:bg-green-500 group-hover:text-white transition-colors">
                      <PlusIcon class="w-5 h-5" />
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </section>

        <aside class="w-full lg:w-[400px] flex flex-col bg-white rounded-3xl shadow-sm border border-gray-100 overflow-hidden shrink-0">
          <div class="p-5 border-b border-gray-100 bg-gray-50 flex justify-between items-center shrink-0">
            <h2 class="text-lg font-black text-gray-800 flex items-center gap-2">
              <ShoppingCartIcon class="w-5 h-5 text-[#1a402d]" />
              Draft Transaksi
            </h2>
            <button @click="clearCart" class="text-xs font-bold text-red-500 hover:bg-red-50 px-3 py-1.5 rounded-lg transition">Kosongkan</button>
          </div>

          <div class="flex-1 overflow-y-auto p-4 space-y-3 bg-white no-scrollbar">
            <div v-if="cart.length === 0" class="h-full flex flex-col items-center justify-center text-gray-400">
              <ArchiveIcon class="w-12 h-12 mb-3 opacity-20" />
              <p class="text-sm font-bold">Keranjang kosong</p>
              <p class="text-xs">Silakan pilih produk koperasi</p>
            </div>

            <div 
              v-for="item in cart" :key="item.id"
              class="flex items-center gap-3 p-3 rounded-xl border border-gray-100 hover:border-gray-200 bg-gray-50/50 transition-colors"
            >
              <img :src="item.image" class="w-10 h-10 rounded-lg object-cover border border-gray-200 shrink-0" />
              
              <div class="flex-1 min-w-0">
                <h4 class="text-sm font-black text-gray-800 truncate">{{ item.name }}</h4>
                <p class="text-xs font-bold text-gray-500">{{ formatRupiah(item.price) }}</p>
              </div>
              
              <div class="flex items-center gap-2 bg-white border border-gray-200 rounded-lg p-1 shrink-0">
                <button @click="updateQty(item, -1)" class="w-6 h-6 flex items-center justify-center rounded bg-gray-50 text-gray-600 hover:bg-red-50 hover:text-red-500 transition font-bold">-</button>
                <span class="w-6 text-center text-xs font-black">{{ item.qty }}</span>
                <button @click="updateQty(item, 1)" class="w-6 h-6 flex items-center justify-center rounded bg-gray-50 text-gray-600 hover:bg-green-50 hover:text-green-600 transition font-bold">+</button>
              </div>
              
              <div class="text-right min-w-[70px] shrink-0">
                <p class="text-sm font-black text-[#1a402d]">{{ formatRupiah(item.price * item.qty) }}</p>
              </div>
            </div>
          </div>

          <div class="p-5 border-t border-gray-100 bg-gray-50 shrink-0">
            <div class="space-y-3 mb-5">
              <div class="flex justify-between text-sm font-bold text-gray-500">
                <span>Subtotal</span>
                <span>{{ formatRupiah(subtotal) }}</span>
              </div>
              <div class="flex justify-between text-sm font-bold text-gray-500">
                <span>Pajak Koperasi (2%)</span>
                <span>{{ formatRupiah(tax) }}</span>
              </div>
              <div class="h-px w-full bg-gray-200 my-2"></div>
              <div class="flex justify-between items-center">
                <span class="text-sm font-black text-gray-800 uppercase tracking-widest">Total Bayar</span>
                <span class="text-2xl font-black text-[#1a402d]">{{ formatRupiah(total) }}</span>
              </div>
            </div>
            
            <button 
              :disabled="cart.length === 0"
              @click="openPaymentModal"
              class="w-full py-4 rounded-xl font-black text-white text-lg tracking-wide transition-all shadow-lg flex items-center justify-center gap-2"
              :class="cart.length === 0 ? 'bg-gray-300 cursor-not-allowed shadow-none' : 'bg-[#1a402d] hover:bg-[#143222] hover:-translate-y-1 hover:shadow-xl'"
            >
              <CreditCardIcon class="w-6 h-6" />
              Proses Pembayaran
            </button>
          </div>
        </aside>

      </div>
    </main>

    <div v-if="isPaymentModalOpen" class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm p-4">
      <div class="bg-white rounded-2xl shadow-2xl w-full max-w-lg overflow-hidden flex flex-col animate-in fade-in zoom-in duration-200">
        
        <div class="bg-[#1a402d] p-5 flex justify-between items-center text-white shrink-0">
          <h3 class="text-xl font-black flex items-center gap-2">
            <CreditCardIcon class="w-6 h-6" />
            Detail Pembayaran Kasir
          </h3>
          <button @click="closePaymentModal" class="text-white/70 hover:text-white transition p-1 rounded-lg hover:bg-white/10">
            <XIcon class="w-6 h-6" />
          </button>
        </div>

        <div class="p-6 overflow-y-auto max-h-[70vh] no-scrollbar">
          
          <div class="space-y-3 mb-6">
            <h4 class="text-xs font-black text-gray-400 uppercase tracking-widest mb-2 border-b border-gray-100 pb-2">Rincian Produk</h4>
            <div v-for="item in cart" :key="item.id" class="flex justify-between items-center text-sm">
              <div class="flex-1 pr-4">
                <p class="font-bold text-gray-800 truncate">{{ item.name }}</p>
                <p class="text-xs text-gray-500">{{ item.qty }} x {{ formatRupiah(item.price) }}</p>
              </div>
              <p class="font-black text-gray-800 shrink-0">{{ formatRupiah(item.price * item.qty) }}</p>
            </div>
          </div>

          <div class="bg-gray-50 rounded-xl p-4 space-y-2 mb-6 border border-gray-100">
            <div class="flex justify-between text-sm text-gray-500 font-bold">
              <span>Subtotal</span>
              <span>{{ formatRupiah(subtotal) }}</span>
            </div>
            <div class="flex justify-between text-sm text-gray-500 font-bold">
              <span>Pajak Koperasi (2%)</span>
              <span>{{ formatRupiah(tax) }}</span>
            </div>
            <div class="flex justify-between items-end pt-3 mt-3 border-t border-gray-200">
              <span class="text-sm font-black text-gray-800 uppercase tracking-widest">Total Tagihan</span>
              <span class="text-2xl font-black text-[#1a402d]">{{ formatRupiah(total) }}</span>
            </div>
          </div>

          <div class="space-y-2">
            <label class="text-sm font-black text-gray-700 block uppercase tracking-wide">Nominal Uang Diterima</label>
            <div class="relative">
              <span class="absolute left-4 top-1/2 -translate-y-1/2 font-black text-gray-500 text-lg">Rp</span>
              <input 
                v-model="formattedAmountTendered" 
                type="text" 
                class="w-full pl-12 pr-4 py-4 bg-white border-2 border-gray-200 rounded-xl font-black text-xl text-gray-800 focus:outline-none focus:border-[#1a402d] focus:ring-4 focus:ring-green-500/20 transition-all"
                placeholder="0"
              />
            </div>
          </div>

          <div class="mt-4 p-4 rounded-xl flex justify-between items-center transition-colors duration-300 border"
               :class="changeAmount >= 0 ? 'bg-green-50 border-green-200 text-green-800' : 'bg-red-50 border-red-200 text-red-800'">
            <span class="font-black text-sm uppercase tracking-wide">Kembalian</span>
            <span class="font-black text-2xl">
              {{ amountTendered === null || amountTendered === 0 ? 'Rp 0' : (changeAmount >= 0 ? formatRupiah(changeAmount) : 'Uang Kurang') }}
            </span>
          </div>

        </div>

        <div class="p-6 border-t border-gray-100 bg-gray-50 flex gap-4 shrink-0">
          <button @click="closePaymentModal" class="px-6 py-4 font-black text-gray-600 bg-white border-2 border-gray-200 rounded-xl hover:bg-gray-50 transition hover:border-gray-300">
            Batal
          </button>
          <button 
            @click="confirmPayment"
            :disabled="changeAmount < 0 || !amountTendered"
            class="flex-1 py-4 font-black text-white text-lg tracking-wide rounded-xl shadow-lg transition-all flex items-center justify-center gap-2"
            :class="changeAmount < 0 || !amountTendered ? 'bg-gray-300 cursor-not-allowed shadow-none' : 'bg-[#1a402d] hover:bg-[#143222] hover:-translate-y-1 hover:shadow-xl'"
          >
            Selesaikan Transaksi
          </button>
        </div>

      </div>
    </div>

  </div>
</template>

<script setup>
import { ref, computed } from 'vue'
import { 
  MenuIcon, BuildingIcon, ClockIcon, SearchIcon, PlusIcon,
  ShoppingCartIcon, ArchiveIcon, CreditCardIcon, XIcon
} from 'lucide-vue-next'

import SidebarHybrid from '~/components/SidebarHybrid.vue'

const isSidebarOpen = ref(false)

const categories = ref([
  { id: 'semua', label: 'Semua Produk' },
  { id: 'pangan', label: 'Agro Pangan' },
  { id: 'ternak', label: 'Hasil Ternak' }
])
const activeCategory = ref('semua')
const searchQuery = ref('')

const products = ref([
  { id: 'PGN-001', name: 'Beras Organik Mapan', category: 'pangan', price: 75000, stock: 45, unit: '5kg', image: 'https://images.unsplash.com/photo-1586201375761-83865001e31c?auto=format&fit=crop&w=400&q=80' },
  { id: 'PGN-002', name: 'Jagung Manis Bonanza', category: 'pangan', price: 12000, stock: 120, unit: '1kg', image: 'https://images.unsplash.com/photo-1551754655-cd27e38d2076?auto=format&fit=crop&w=400&q=80' },
  { id: 'TRN-001', name: 'Telur Ayam Omega 3', category: 'ternak', price: 32000, stock: 80, unit: '1kg', image: 'https://images.unsplash.com/photo-1506976785307-8732e854ad03?auto=format&fit=crop&w=400&q=80' },
  { id: 'TRN-002', name: 'Pisang Segar', category: 'ternak', price: 135000, stock: 15, unit: '1kg', image: 'https://images.unsplash.com/photo-1528825871115-3581a5387919?auto=format&fit=crop&w=400&q=80' },
  { id: 'PGN-003', name: 'Cabai Rawit Merah', category: 'pangan', price: 45000, stock: 25, unit: '1kg', image: 'https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcR5Obf9469jpUOyLMX3Nk6kXXYc2-1ajlcVOA&s' },
  { id: 'TRN-003', name: 'Susu Sapi Segar Literan', category: 'ternak', price: 18000, stock: 30, unit: '1L', image: 'https://images.unsplash.com/photo-1563636619-e9143da7973b?auto=format&fit=crop&w=400&q=80' },
  { id: 'PGN-004', name: 'Pupuk Kompos Koperasi', category: 'pangan', price: 25000, stock: 200, unit: '10kg', image: 'https://images.unsplash.com/photo-1464226184884-fa280b87c399?auto=format&fit=crop&w=400&q=80' },
  { id: 'TRN-004', name: 'Pakan Ayam Petelur', category: 'ternak', price: 350000, stock: 10, unit: '50kg', image: 'https://images.unsplash.com/photo-1501430654243-c934cec2e1c0?auto=format&fit=crop&w=400&q=80' },
])

const filteredProducts = computed(() => {
  return products.value.filter(p => {
    const matchCat = activeCategory.value === 'semua' || p.category === activeCategory.value
    const matchSearch = p.name.toLowerCase().includes(searchQuery.value.toLowerCase()) || 
                        p.id.toLowerCase().includes(searchQuery.value.toLowerCase())
    return matchCat && matchSearch
  })
})

const cart = ref([])

const addToCart = (product) => {
  if (product.stock <= 0) return alert('Stok produk habis!')
  
  const existing = cart.value.find(item => item.id === product.id)
  if (existing) {
    if (existing.qty < product.stock) {
      existing.qty++
    }
  } else {
    cart.value.push({
      ...product,
      qty: 1
    })
  }
}

const updateQty = (item, amount) => {
  const index = cart.value.findIndex(i => i.id === item.id)
  if (index === -1) return

  const targetItem = cart.value[index]
  const newQty = targetItem.qty + amount

  if (newQty <= 0) {
    cart.value.splice(index, 1)
  } else if (newQty <= targetItem.stock) {
    targetItem.qty = newQty
  }
}

const clearCart = () => {
  cart.value = []
}

const subtotal = computed(() => {
  return cart.value.reduce((sum, item) => sum + (item.price * item.qty), 0)
})

const tax = computed(() => subtotal.value * 0.02)
const total = computed(() => subtotal.value + tax.value)

const formatRupiah = (value) => {
  return new Intl.NumberFormat('id-ID', { style: 'currency', currency: 'IDR', minimumFractionDigits: 0 }).format(value)
}

// State untuk Data Asli
const isPaymentModalOpen = ref(false)
const amountTendered = ref(null)

// Computed Property Dua Arah (Two-Way Binding) untuk format Mata Uang
const formattedAmountTendered = computed({
  get() {
    if (!amountTendered.value) return ''
    return amountTendered.value.toLocaleString('id-ID')
  },
  set(newValue) {
    // Hapus semua karakter yang bukan angka
    const cleanValue = newValue.replace(/\D/g, '')
    amountTendered.value = cleanValue ? parseInt(cleanValue, 10) : null
  }
})

const changeAmount = computed(() => {
  const amount = Number(amountTendered.value) || 0
  return amount - total.value
})

const openPaymentModal = () => {
  if (cart.value.length === 0) return
  amountTendered.value = null 
  isPaymentModalOpen.value = true
}

const closePaymentModal = () => {
  isPaymentModalOpen.value = false
  amountTendered.value = null
}

const confirmPayment = () => {
  if (changeAmount.value < 0) return 

  const payload = {
    items: cart.value,
    subtotal: subtotal.value,
    tax: tax.value,
    total: total.value,
    amountTendered: Number(amountTendered.value),
    change: changeAmount.value,
    timestamp: new Date().toISOString()
  }
  
  console.log("PAYLOAD SIAP DI-COMMIT KE queue_keranjang:", JSON.stringify(payload, null, 2))
  
  clearCart()
  closePaymentModal()
}
</script>

<style scoped>
.no-scrollbar::-webkit-scrollbar { display: none; }
.no-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }
</style>