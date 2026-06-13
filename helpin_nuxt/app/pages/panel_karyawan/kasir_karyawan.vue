<template>
  <div class="flex h-screen w-full bg-[#F4FBF7] font-sans overflow-hidden relative">
    <SidebarKaryawan :isOpen="isSidebarOpen" @close="isSidebarOpen = false" />
    <main class="flex-1 flex flex-col overflow-hidden relative w-full">
      <header class="flex justify-between items-center px-6 md:px-10 py-5 border-b border-gray-200 bg-white shadow-sm z-10">
        <div class="flex items-center gap-4">
          <button @click="isSidebarOpen = true" class="md:hidden p-2 -ml-2 text-gray-600 hover:bg-gray-100 rounded-lg transition"><MenuIcon class="w-6 h-6" /></button>
          <div>
            <h1 class="text-xl md:text-2xl font-black text-[#19462D] tracking-tight">Kasir (POS)</h1>
            <p class="text-xs md:text-sm text-gray-500 font-medium mt-0.5">Penjualan langsung — otomatis tercatat sebagai pemasukan kas</p>
          </div>
        </div>
      </header>

      <div class="flex-1 flex flex-col lg:flex-row overflow-hidden">
        <div class="flex-1 overflow-y-auto p-4 md:p-6 no-scrollbar">
          <div class="relative mb-4">
            <SearchIcon class="w-5 h-5 absolute left-4 top-1/2 -translate-y-1/2 text-gray-400" />
            <input v-model="search" placeholder="Cari produk..." class="w-full pl-12 pr-4 py-3 border border-gray-200 rounded-xl outline-none focus:border-[#19462D] bg-white text-sm">
          </div>
          <div v-if="loading" class="py-16 text-center text-gray-400 font-bold">Memuat produk...</div>
          <div v-else class="grid grid-cols-2 sm:grid-cols-3 xl:grid-cols-4 gap-3">
            <button v-for="p in filtered" :key="p.id" @click="addToCart(p)" :disabled="p.stock <= 0"
              class="bg-white border border-gray-100 rounded-2xl p-3 text-left hover:border-[#19462D] hover:shadow transition disabled:opacity-40">
              <div class="w-full h-20 bg-gray-50 rounded-xl mb-2 flex items-center justify-center overflow-hidden">
                <img v-if="p.image_url" :src="p.image_url" class="w-full h-full object-contain" />
                <BoxIcon v-else class="w-8 h-8 text-gray-300" />
              </div>
              <p class="text-xs font-black text-gray-800 truncate">{{ p.name }}</p>
              <p class="text-[11px] text-green-600 font-bold">{{ rupiah(p.price) }}</p>
              <p class="text-[10px] text-gray-400">Stok: {{ formatNumber(p.stock) }}</p>
            </button>
          </div>
        </div>

        <div class="w-full lg:w-96 bg-white border-l border-gray-100 flex flex-col shrink-0">
          <div class="p-5 border-b border-gray-100 font-black text-[#19462D] flex items-center gap-2"><ShoppingCartIcon class="w-5 h-5" /> Keranjang</div>
          <div class="flex-1 overflow-y-auto p-4 no-scrollbar space-y-2">
            <p v-if="cart.length === 0" class="text-center text-gray-300 font-bold py-10">Keranjang kosong</p>
            <div v-for="c in cart" :key="c.product_id" class="flex items-center gap-2 bg-gray-50 rounded-xl p-2">
              <div class="flex-1 min-w-0">
                <p class="text-xs font-black text-gray-800 truncate">{{ c.name }}</p>
                <p class="text-[11px] text-gray-500">{{ rupiah(c.price) }}</p>
              </div>
              <div class="flex items-center gap-1">
                <button @click="dec(c)" class="w-6 h-6 bg-white border border-gray-200 rounded font-black">-</button>
                <span class="w-6 text-center text-sm font-bold">{{ c.quantity }}</span>
                <button @click="inc(c)" class="w-6 h-6 bg-white border border-gray-200 rounded font-black">+</button>
              </div>
            </div>
          </div>
          <div class="p-5 border-t border-gray-100 space-y-2">
            <div class="flex justify-between text-sm"><span class="text-gray-500">Subtotal</span><span class="font-bold">{{ rupiah(subtotal) }}</span></div>
            <div class="flex justify-between text-sm"><span class="text-gray-500">Pajak (2%)</span><span class="font-bold">{{ rupiah(tax) }}</span></div>
            <div class="flex justify-between text-base"><span class="font-black text-[#19462D]">Total</span><span class="font-black text-[#19462D]">{{ rupiah(total) }}</span></div>
            <input v-model="tenderedFmt" type="text" inputmode="numeric" placeholder="Uang dibayar..." class="w-full p-3 border border-gray-200 rounded-xl bg-gray-50 text-sm mt-2">
            <div v-if="tendered >= total && total > 0" class="flex justify-between text-sm"><span class="text-gray-500">Kembalian</span><span class="font-black text-green-600">{{ rupiah(tendered - total) }}</span></div>
            <button @click="pay" :disabled="paying || cart.length === 0 || tendered < total" class="w-full py-3 bg-[#19462D] text-white rounded-xl font-black uppercase text-sm disabled:opacity-50">{{ paying ? 'Memproses...' : 'Bayar' }}</button>
          </div>
        </div>
      </div>
    </main>
  </div>
</template>

<script setup>
import { ref, reactive, computed, onMounted } from 'vue'
import { MenuIcon, SearchIcon, ShoppingCartIcon, BoxIcon } from 'lucide-vue-next'
import SidebarKaryawan from '~/components/SidebarKaryawan.vue'

const isSidebarOpen = ref(false)
const { list: fetchProducts } = useProductAdmin()
const { createTransaction } = usePos()
const { formatRupiah, formatNumber, parseNumber } = useFormat()
const toast = useToast()

const products = ref([])
const loading = ref(false)
const paying = ref(false)
const search = ref('')
const cart = reactive([])
const tendered = ref(null)

const tenderedFmt = computed({
  get: () => (tendered.value === null || tendered.value === '' ? '' : formatNumber(tendered.value, 0)),
  set: (v) => { tendered.value = v === '' ? null : parseNumber(v) },
})

const load = async () => {
  loading.value = true
  try { products.value = (await fetchProducts()) || [] } catch (e) { console.error(e); toast.error('Gagal memuat produk', e?.data?.message) } finally { loading.value = false }
}
onMounted(load)

const filtered = computed(() => products.value.filter(p => p.name?.toLowerCase().includes(search.value.toLowerCase())))
const subtotal = computed(() => cart.reduce((a, c) => a + c.price * c.quantity, 0))
const tax = computed(() => Math.round(subtotal.value * 0.02))
const total = computed(() => subtotal.value + tax.value)
const rupiah = (n) => formatRupiah(n)

const addToCart = (p) => {
  const ex = cart.find(c => c.product_id === p.id)
  if (ex) { if (ex.quantity < p.stock) ex.quantity++; else toast.warning('Stok tidak cukup', p.name) }
  else cart.push({ product_id: p.id, name: p.name, price: p.price, quantity: 1 })
}
const inc = (c) => { const p = products.value.find(x => x.id === c.product_id); if (!p || c.quantity < p.stock) c.quantity++ }
const dec = (c) => { c.quantity--; if (c.quantity <= 0) { const i = cart.findIndex(x => x.product_id === c.product_id); cart.splice(i, 1) } }

const pay = async () => {
  if (cart.length === 0 || tendered.value < total.value) return
  paying.value = true
  try {
    await createTransaction({
      items: cart.map(c => ({ product_id: c.product_id, quantity: c.quantity, price: c.price })),
      amount_tendered: Number(tendered.value),
    })
    toast.success('Pembayaran berhasil', `Kembalian: ${rupiah(tendered.value - total.value)}`)
    cart.splice(0, cart.length)
    tendered.value = null
    await load()
  } catch (e) { console.error(e); toast.error('Gagal memproses pembayaran', e?.data?.error?.message || e?.data?.message) }
  finally { paying.value = false }
}
</script>

<style scoped>
.no-scrollbar::-webkit-scrollbar { display: none; }
.no-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }
</style>
