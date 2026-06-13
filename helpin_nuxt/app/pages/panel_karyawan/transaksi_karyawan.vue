<template>
  <div class="flex h-screen w-full bg-[#F4FBF7] font-sans overflow-hidden relative">
    <SidebarKaryawan :isOpen="isSidebarOpen" @close="isSidebarOpen = false" />
    <main class="flex-1 flex flex-col overflow-y-auto relative w-full no-scrollbar">
      <header class="flex justify-between items-center px-6 md:px-10 py-5 border-b border-gray-200 bg-white/90 backdrop-blur-md shadow-sm z-10 sticky top-0">
        <div class="flex items-center gap-4">
          <button @click="isSidebarOpen = true" class="md:hidden p-2 -ml-2 text-gray-600 hover:bg-gray-100 rounded-lg transition"><MenuIcon class="w-6 h-6" /></button>
          <div>
            <h1 class="text-xl md:text-2xl font-black text-[#19462D] tracking-tight">Riwayat Transaksi</h1>
            <p class="text-xs md:text-sm text-gray-500 font-medium mt-0.5">Transaksi kasir (POS)</p>
          </div>
        </div>
      </header>

      <div class="p-4 md:p-8 flex flex-col gap-6 animate-fade">
        <section class="grid grid-cols-2 md:grid-cols-3 gap-4">
          <div class="bg-white p-5 rounded-2xl border border-gray-100 shadow-sm"><p class="text-[10px] font-black text-gray-400 uppercase">Total Transaksi</p><p class="text-2xl font-black text-[#19462D]">{{ formatNumber(list.length) }}</p></div>
          <div class="bg-white p-5 rounded-2xl border border-gray-100 shadow-sm"><p class="text-[10px] font-black text-gray-400 uppercase">Total Omzet</p><p class="text-2xl font-black text-green-700">{{ formatRupiah(totalOmzet) }}</p></div>
          <div class="bg-white p-5 rounded-2xl border border-gray-100 shadow-sm"><p class="text-[10px] font-black text-gray-400 uppercase">Total Pajak</p><p class="text-2xl font-black text-gray-700">{{ formatRupiah(totalTax) }}</p></div>
        </section>

        <section class="bg-white rounded-3xl shadow-sm border border-gray-100 p-6">
          <div v-if="loading" class="py-16 text-center text-gray-400 font-bold">Memuat...</div>
          <div v-else class="overflow-x-auto no-scrollbar">
            <table class="w-full text-sm text-left min-w-[800px]">
              <thead class="text-xs text-green-800 bg-green-50 uppercase tracking-widest border-b border-gray-100">
                <tr>
                  <th class="p-4 font-black rounded-tl-lg">ID Transaksi</th>
                  <th class="p-4 font-black">Item</th>
                  <th class="p-4 font-black">Subtotal</th>
                  <th class="p-4 font-black">Pajak</th>
                  <th class="p-4 font-black">Total</th>
                  <th class="p-4 font-black">Waktu</th>
                  <th class="p-4 font-black rounded-tr-lg text-center">Aksi</th>
                </tr>
              </thead>
              <tbody class="divide-y divide-gray-50">
                <tr v-if="list.length === 0"><td colspan="7" class="py-12 text-center text-gray-300 font-bold">Belum ada transaksi</td></tr>
                <tr v-for="t in list" :key="t.id" class="hover:bg-gray-50/50 transition-colors">
                  <td class="p-4 font-mono text-xs text-gray-500">{{ String(t.id).slice(0, 8) }}</td>
                  <td class="p-4 font-bold text-gray-700">{{ formatNumber(itemCount(t)) }} item</td>
                  <td class="p-4 text-gray-600">{{ formatRupiah(t.subtotal) }}</td>
                  <td class="p-4 text-gray-600">{{ formatRupiah(t.tax) }}</td>
                  <td class="p-4 font-black text-green-700">{{ formatRupiah(t.total) }}</td>
                  <td class="p-4 font-bold text-gray-500">{{ formatDateTime(t.created_at) }}</td>
                  <td class="p-4 text-center">
                    <button @click="selected = t" class="inline-flex items-center gap-1.5 px-3 py-1.5 bg-blue-50 text-blue-600 rounded-lg text-xs font-bold hover:bg-blue-500 hover:text-white transition">
                      <EyeIcon class="w-4 h-4" /> Lihat
                    </button>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </section>
      </div>
    </main>

    <TransactionDetailModal :transaction="selected" :products="products" @close="selected = null" />
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { MenuIcon, EyeIcon } from 'lucide-vue-next'
import SidebarKaryawan from '~/components/SidebarKaryawan.vue'
import TransactionDetailModal from '~/components/TransactionDetailModal.vue'

const isSidebarOpen = ref(false)
const { listTransactions } = usePos()
const { list: fetchProducts } = useProductAdmin()
const { formatRupiah, formatNumber, formatDateTime } = useFormat()
const toast = useToast()

const list = ref([])
const products = ref([])
const loading = ref(false)
const selected = ref(null)

onMounted(async () => {
  loading.value = true
  try {
    const [tx, pr] = await Promise.allSettled([listTransactions(), fetchProducts()])
    list.value = tx.status === 'fulfilled' ? (tx.value || []) : []
    products.value = pr.status === 'fulfilled' ? (pr.value || []) : []
  } catch (e) {
    console.error(e)
    toast.error('Gagal memuat transaksi', e?.data?.message || e?.message)
  } finally {
    loading.value = false
  }
})

const totalOmzet = computed(() => list.value.reduce((a, t) => a + (t.total || 0), 0))
const totalTax = computed(() => list.value.reduce((a, t) => a + (t.tax || 0), 0))
const itemCount = (t) => Array.isArray(t.items) ? t.items.length : (t.items ? Object.keys(t.items).length : 0)
</script>

<style scoped>
.no-scrollbar::-webkit-scrollbar { display: none; }
.no-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }
.animate-fade { animation: fadeIn 0.4s ease; }
@keyframes fadeIn { from { opacity: 0; transform: translateY(10px); } to { opacity: 1; transform: translateY(0); } }
</style>
