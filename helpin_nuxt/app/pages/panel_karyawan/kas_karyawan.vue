<template>
  <div class="flex h-screen w-full bg-[#F4FBF7] font-sans overflow-hidden relative">
    <SidebarKaryawan :isOpen="isSidebarOpen" @close="isSidebarOpen = false" />
    <main class="flex-1 flex flex-col overflow-y-auto relative w-full no-scrollbar">
      <header class="flex justify-between items-center px-6 md:px-10 py-5 border-b border-gray-200 bg-white/90 backdrop-blur-md shadow-sm z-10 sticky top-0">
        <div class="flex items-center gap-4">
          <button @click="isSidebarOpen = true" class="md:hidden p-2 -ml-2 text-gray-600 hover:bg-gray-100 rounded-lg transition"><MenuIcon class="w-6 h-6" /></button>
          <div>
            <h1 class="text-xl md:text-2xl font-black text-[#19462D] tracking-tight">Buku Kas</h1>
            <p class="text-xs md:text-sm text-gray-500 font-medium mt-0.5">Sirkulasi keuangan real-time</p>
          </div>
        </div>
      </header>

      <div class="p-4 md:p-8 flex flex-col gap-6 animate-fade">
        <section class="grid grid-cols-1 md:grid-cols-3 gap-6">
          <div class="bg-white p-6 rounded-2xl border border-gray-100 shadow-sm flex items-center gap-4">
            <div class="w-14 h-14 rounded-xl bg-green-50 flex items-center justify-center text-green-700"><WalletIcon class="w-7 h-7" /></div>
            <div><p class="text-[11px] font-bold text-gray-400 uppercase mb-1">Total Kas</p><h3 class="text-2xl font-black text-gray-800">{{ rupiah(balance) }}</h3></div>
          </div>
          <div class="bg-white p-6 rounded-2xl border border-gray-100 shadow-sm flex items-center gap-4">
            <div class="w-14 h-14 rounded-xl bg-blue-50 flex items-center justify-center text-blue-600"><TrendingUpIcon class="w-7 h-7" /></div>
            <div><p class="text-[11px] font-bold text-gray-400 uppercase mb-1">Uang Masuk</p><h3 class="text-2xl font-black text-gray-800">{{ rupiah(totalIn) }}</h3></div>
          </div>
          <div class="bg-white p-6 rounded-2xl border border-gray-100 shadow-sm flex items-center gap-4">
            <div class="w-14 h-14 rounded-xl bg-red-50 flex items-center justify-center text-red-500"><TrendingDownIcon class="w-7 h-7" /></div>
            <div><p class="text-[11px] font-bold text-gray-400 uppercase mb-1">Uang Keluar</p><h3 class="text-2xl font-black text-gray-800">{{ rupiah(totalOut) }}</h3></div>
          </div>
        </section>

        <section class="bg-white rounded-3xl shadow-sm border border-gray-100 p-6">
          <h3 class="text-lg font-bold text-[#19462D] flex items-center gap-2 mb-6"><div class="w-2 h-2 rounded-full bg-[#19462D]"></div> Catat Kas Manual</h3>
          <div class="flex flex-col md:flex-row gap-4 mb-6">
            <input v-model="form.description" placeholder="Keterangan..." class="w-full md:flex-[2] p-3 border border-gray-200 rounded-xl outline-none focus:border-[#19462D] bg-gray-50 text-sm">
            <select v-model="form.type" class="w-full md:flex-[1] p-3 border border-gray-200 rounded-xl outline-none focus:border-[#19462D] bg-gray-50 text-sm font-semibold text-gray-600">
              <option value="pemasukan">Uang Masuk</option>
              <option value="pengeluaran">Uang Keluar</option>
            </select>
            <input v-model="amountFmt" type="text" inputmode="numeric" placeholder="Nominal..." class="w-full md:flex-[1] p-3 border border-gray-200 rounded-xl outline-none focus:border-[#19462D] bg-gray-50 text-sm">
            <button @click="record" :disabled="saving" class="bg-blue-500 hover:bg-blue-600 text-white px-6 py-3 rounded-xl font-bold text-sm flex items-center justify-center gap-2 transition whitespace-nowrap disabled:opacity-50">
              <SaveIcon class="w-4 h-4" /> Catat
            </button>
          </div>

          <div v-if="loading" class="py-10 text-center text-gray-400 font-bold">Memuat...</div>
          <div v-else class="overflow-x-auto no-scrollbar">
            <table class="w-full text-sm text-left min-w-[700px]">
              <thead class="text-xs text-green-800 bg-green-50 uppercase tracking-widest border-b border-gray-100">
                <tr>
                  <th class="p-4 font-black rounded-tl-lg">Keterangan</th>
                  <th class="p-4 font-black">Kategori</th>
                  <th class="p-4 font-black">Jenis</th>
                  <th class="p-4 font-black">Nominal</th>
                  <th class="p-4 font-black rounded-tr-lg">Waktu</th>
                </tr>
              </thead>
              <tbody class="divide-y divide-gray-50">
                <tr v-if="records.length === 0"><td colspan="5" class="py-12 text-center text-gray-300 font-bold">Belum ada catatan kas</td></tr>
                <tr v-for="r in records" :key="r.id" class="hover:bg-gray-50/50 transition-colors">
                  <td class="p-4 font-bold text-gray-800">{{ r.description || '-' }}</td>
                  <td class="p-4 text-gray-500">{{ r.category || '-' }}</td>
                  <td class="p-4"><span :class="['px-3 py-1.5 text-[11px] font-black rounded-lg uppercase', r.record_type === 'pemasukan' ? 'bg-blue-500 text-white' : 'bg-red-500 text-white']">{{ r.record_type === 'pemasukan' ? 'Masuk' : 'Keluar' }}</span></td>
                  <td class="p-4 font-black text-gray-800">{{ rupiah(r.amount) }}</td>
                  <td class="p-4 font-bold text-gray-500">{{ formatDateTime(r.recorded_at) }}</td>
                </tr>
              </tbody>
            </table>
          </div>
        </section>
      </div>
    </main>
  </div>
</template>

<script setup>
import { ref, reactive, computed, onMounted } from 'vue'
import { MenuIcon, WalletIcon, TrendingUpIcon, TrendingDownIcon, SaveIcon } from 'lucide-vue-next'
import SidebarKaryawan from '~/components/SidebarKaryawan.vue'

const isSidebarOpen = ref(false)
const { getBalance, recordIncome, recordExpense, getReport } = useFinance()
const { formatRupiah, formatNumber, formatDateTime, parseNumber } = useFormat()
const toast = useToast()
const balance = ref(0)
const records = ref([])
const loading = ref(false)
const saving = ref(false)
const form = reactive({ description: '', type: 'pemasukan', amount: null })

const amountFmt = computed({
  get: () => (form.amount === null || form.amount === '' ? '' : formatNumber(form.amount, 0)),
  set: (v) => { form.amount = v === '' ? null : parseNumber(v) },
})

const load = async () => {
  loading.value = true
  try {
    const [b, rep] = await Promise.allSettled([getBalance(), getReport(new Date(Date.now() - 365 * 864e5).toISOString(), new Date().toISOString())])
    balance.value = b.status === 'fulfilled' ? (b.value?.balance || 0) : 0
    records.value = rep.status === 'fulfilled' ? (rep.value || []) : []
  } catch (e) { console.error(e); toast.error('Gagal memuat kas', e?.data?.message) } finally { loading.value = false }
}
onMounted(load)

const totalIn = computed(() => records.value.filter(r => r.record_type === 'pemasukan').reduce((a, r) => a + (r.amount || 0), 0))
const totalOut = computed(() => records.value.filter(r => r.record_type === 'pengeluaran').reduce((a, r) => a + (r.amount || 0), 0))

const rupiah = (n) => formatRupiah(n)

const record = async () => {
  if (!form.amount || form.amount <= 0) { toast.warning('Nominal tidak valid', 'Nominal harus lebih dari 0'); return }
  saving.value = true
  try {
    const payload = { amount: Number(form.amount), category: 'Manual', description: form.description || null }
    if (form.type === 'pemasukan') await recordIncome(payload)
    else await recordExpense(payload)
    toast.success(form.type === 'pemasukan' ? 'Uang masuk dicatat' : 'Uang keluar dicatat', rupiah(form.amount))
    form.description = ''; form.amount = null
    await load()
  } catch (e) { console.error(e); toast.error('Gagal mencatat kas', e?.data?.message) }
  finally { saving.value = false }
}
</script>

<style scoped>
.no-scrollbar::-webkit-scrollbar { display: none; }
.no-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }
.animate-fade { animation: fadeIn 0.4s cubic-bezier(0.16, 1, 0.3, 1); }
@keyframes fadeIn { from { opacity: 0; transform: translateY(10px); } to { opacity: 1; transform: translateY(0); } }
</style>
