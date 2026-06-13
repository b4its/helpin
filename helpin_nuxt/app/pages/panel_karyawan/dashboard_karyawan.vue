<template>
  <div class="flex h-screen w-full bg-[#F4FBF7] font-sans overflow-hidden relative">
    <SidebarKaryawan :isOpen="isSidebarOpen" @close="isSidebarOpen = false" />
    <main class="flex-1 flex flex-col overflow-y-auto relative w-full no-scrollbar">
      <header class="flex justify-between items-center px-6 md:px-10 py-5 border-b border-gray-200 bg-white/90 backdrop-blur-md shadow-sm z-10 sticky top-0">
        <div class="flex items-center gap-4">
          <button @click="isSidebarOpen = true" class="md:hidden p-2 -ml-2 text-gray-600 hover:bg-gray-100 rounded-lg transition"><MenuIcon class="w-6 h-6" /></button>
          <div>
            <h1 class="text-xl md:text-2xl font-black text-[#19462D] tracking-tight">Dashboard Karyawan</h1>
            <p class="text-xs md:text-sm text-gray-500 font-medium mt-0.5">Ringkasan real-time dari database</p>
          </div>
        </div>
      </header>

      <div class="p-4 md:p-8 flex flex-col gap-6 animate-fade">
        <div v-if="loading" class="py-20 text-center text-gray-400 font-bold">Memuat...</div>
        <template v-else>
          <section class="grid grid-cols-2 lg:grid-cols-4 gap-4">
            <div v-for="c in cards" :key="c.label" class="bg-white p-6 rounded-2xl border border-gray-100 shadow-sm">
              <div class="flex items-center gap-3 mb-2">
                <div :class="['w-10 h-10 rounded-xl flex items-center justify-center', c.bg]"><component :is="c.icon" class="w-5 h-5" /></div>
                <p class="text-[10px] font-black text-gray-400 uppercase">{{ c.label }}</p>
              </div>
              <h3 class="text-2xl font-black text-[#19462D]">{{ formatNumber(c.value) }}</h3>
            </div>
          </section>

          <section class="grid grid-cols-1 lg:grid-cols-3 gap-6">
            <div class="lg:col-span-2 bg-white p-8 rounded-3xl border border-gray-100 shadow-sm">
              <h3 class="text-lg font-black text-[#19462D] mb-6 uppercase tracking-widest">Arus Kas</h3>
              <div class="h-64"><Bar :data="cashChart" :options="barOptions" /></div>
            </div>
            <div class="bg-white p-8 rounded-3xl border border-gray-100 shadow-sm flex flex-col justify-center gap-4">
              <h3 class="text-lg font-black text-[#19462D] uppercase tracking-widest">Keuangan</h3>
              <div class="bg-green-50 p-5 rounded-2xl">
                <p class="text-[10px] font-black text-green-500 uppercase">Total Kas</p>
                <p class="text-3xl font-black text-green-700">{{ formatRupiah(stats.balance) }}</p>
              </div>
              <div class="grid grid-cols-2 gap-4">
                <div class="bg-blue-50 p-4 rounded-2xl"><p class="text-[10px] font-black text-blue-500 uppercase">Masuk</p><p class="text-lg font-black text-blue-700">{{ formatRupiah(stats.total_income) }}</p></div>
                <div class="bg-red-50 p-4 rounded-2xl"><p class="text-[10px] font-black text-red-500 uppercase">Keluar</p><p class="text-lg font-black text-red-600">{{ formatRupiah(stats.total_expense) }}</p></div>
              </div>
            </div>
          </section>
        </template>
      </div>
    </main>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { MenuIcon, UsersIcon, BoxIcon, TruckIcon } from 'lucide-vue-next'
import { Bar } from 'vue-chartjs'
import { Chart as ChartJS, Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale } from 'chart.js'
import SidebarKaryawan from '~/components/SidebarKaryawan.vue'

ChartJS.register(Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale)

const isSidebarOpen = ref(false)
const { stats: fetchStats } = useAdmin()
const { formatRupiah, formatNumber } = useFormat()
const toast = useToast()
const stats = ref({ total_users: 0, total_products: 0, total_suppliers: 0, total_employees: 0, total_pos_transactions: 0, balance: 0, total_income: 0, total_expense: 0 })
const loading = ref(false)

onMounted(async () => {
  loading.value = true
  try { stats.value = (await fetchStats()) || stats.value } catch (e) { console.error(e); toast.error('Gagal memuat statistik', e?.data?.message) } finally { loading.value = false }
})

const cards = computed(() => [
  { label: 'Total Pengguna', value: stats.value.total_users, icon: UsersIcon, bg: 'bg-purple-50 text-purple-600' },
  { label: 'Karyawan', value: stats.value.total_employees, icon: UsersIcon, bg: 'bg-blue-50 text-blue-600' },
  { label: 'Produk', value: stats.value.total_products, icon: BoxIcon, bg: 'bg-green-50 text-green-600' },
  { label: 'Supplier', value: stats.value.total_suppliers, icon: TruckIcon, bg: 'bg-orange-50 text-orange-600' },
])
const cashChart = computed(() => ({
  labels: ['Masuk', 'Keluar', 'Saldo'],
  datasets: [{ label: 'Rp', data: [stats.value.total_income, stats.value.total_expense, stats.value.balance], backgroundColor: ['#3b82f6', '#ef4444', '#19462D'], borderRadius: 8 }]
}))
const barOptions = { responsive: true, maintainAspectRatio: false, plugins: { legend: { display: false } }, scales: { y: { beginAtZero: true } } }
</script>

<style scoped>
.no-scrollbar::-webkit-scrollbar { display: none; }
.no-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }
.animate-fade { animation: fadeIn 0.4s ease; }
@keyframes fadeIn { from { opacity: 0; transform: translateY(10px); } to { opacity: 1; transform: translateY(0); } }
</style>
