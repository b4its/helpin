<template>
  <div class="flex h-screen w-full bg-[#f4f7f5] font-sans overflow-hidden relative text-slate-900">

    <SidebarPeternak :isOpen="isSidebarOpen" @close="isSidebarOpen = false" />

    <main class="flex-1 flex flex-col overflow-y-auto relative w-full no-scrollbar">
      <header class="flex justify-between items-center px-6 md:px-10 py-6 border-b border-gray-200 bg-white shadow-sm z-10 sticky top-0">
        <div class="flex items-center gap-4">
          <button @click="isSidebarOpen = true" class="md:hidden p-2 text-gray-600 hover:bg-gray-100 rounded-lg transition"><MenuIcon class="w-6 h-6" /></button>
          <div>
            <h1 class="text-xl md:text-2xl font-black text-gray-800 leading-tight">Dashboard Peternak</h1>
            <p class="text-xs md:text-sm text-gray-500 font-medium mt-0.5">Ringkasan real-time dari database</p>
          </div>
        </div>
        <div class="flex items-center gap-2 bg-green-100 px-3 py-1.5 md:px-4 md:py-2 rounded-lg border border-green-200">
          <div class="w-2 h-2 rounded-full bg-green-600 shrink-0 animate-pulse"></div>
          <span class="text-xs md:text-sm font-bold text-green-600">ONLINE</span>
        </div>
      </header>

      <div class="p-4 md:p-10 space-y-8">
        <div v-if="loading" class="py-20 flex justify-center"><div class="animate-spin rounded-full h-8 w-8 border-b-2 border-[#1a402d]"></div></div>

        <template v-else>
          <!-- Stat cards -->
          <section class="grid grid-cols-2 lg:grid-cols-4 gap-4">
            <div v-for="s in stats" :key="s.label" class="bg-white p-6 rounded-3xl border border-gray-100 shadow-sm">
              <p class="text-[10px] font-black text-gray-400 uppercase tracking-[0.2em] mb-2">{{ s.label }}</p>
              <div class="flex items-end justify-between">
                <h3 class="text-3xl font-black text-[#1a402d]">{{ s.value }}</h3>
                <span class="text-[10px] font-black px-2 py-1 rounded-lg bg-green-50 text-green-600">{{ s.sub }}</span>
              </div>
            </div>
          </section>

          <section class="grid grid-cols-1 lg:grid-cols-3 gap-6">
            <div class="lg:col-span-2 bg-white p-8 rounded-[32px] border border-gray-100 shadow-sm">
              <h3 class="text-lg font-black text-gray-800 mb-6 uppercase tracking-widest">Distribusi Jenis Ternak</h3>
              <div v-if="totalLivestock === 0" class="h-64 flex items-center justify-center text-gray-300 font-bold">Belum ada data ternak</div>
              <div v-else class="h-64 w-full"><Bar :data="categoryChart" :options="barOptions" /></div>
            </div>
            <div class="bg-white p-8 rounded-[32px] border border-gray-100 shadow-sm flex flex-col items-center justify-center">
              <h3 class="text-lg font-black text-gray-800 mb-6">Status Kesehatan</h3>
              <div class="relative h-48 w-48 flex items-center justify-center">
                <Doughnut :data="healthChart" :options="doughnutOptions" />
                <div class="absolute inset-0 flex flex-col items-center justify-center">
                  <span class="text-4xl font-black text-gray-800">{{ healthyPercent }}%</span>
                  <span class="text-[10px] text-green-600 font-black uppercase">Sehat</span>
                </div>
              </div>
              <div class="flex gap-4 mt-4 text-[11px] font-bold">
                <span class="flex items-center gap-1"><span class="w-3 h-3 rounded-full bg-green-500"></span>Sehat {{ healthCounts.sehat }}</span>
                <span class="flex items-center gap-1"><span class="w-3 h-3 rounded-full bg-orange-400"></span>Perhatian {{ healthCounts.perhatian }}</span>
                <span class="flex items-center gap-1"><span class="w-3 h-3 rounded-full bg-red-500"></span>Sakit {{ healthCounts.sakit }}</span>
              </div>
            </div>
          </section>

          <!-- Okupansi kandang -->
          <section class="bg-white rounded-[32px] border border-gray-100 shadow-sm p-8">
            <h3 class="text-lg font-black text-gray-800 mb-6 uppercase tracking-widest">Okupansi Kandang</h3>
            <div v-if="pens.length === 0" class="text-gray-300 font-bold text-center py-6">Belum ada kandang</div>
            <div v-else class="space-y-4">
              <div v-for="p in pens" :key="p.id" class="flex items-center gap-4">
                <div class="w-40 shrink-0">
                  <p class="font-black text-gray-800 uppercase truncate">{{ p.name }}</p>
                  <p class="text-[10px] text-gray-400 font-bold">{{ p.pen_type }} · {{ p.condition }}</p>
                </div>
                <div class="flex-1 h-3 bg-gray-100 rounded-full overflow-hidden">
                  <div :class="['h-full rounded-full', (p.occupancy/Math.max(p.capacity,1))>=1 ? 'bg-red-500' : 'bg-[#1a402d]']" :style="`width:${Math.min((p.occupancy/Math.max(p.capacity,1))*100,100)}%`"></div>
                </div>
                <span class="w-16 text-right font-black text-slate-700">{{ p.occupancy }}/{{ p.capacity }}</span>
              </div>
            </div>
          </section>

          <!-- Log rekomendasi pakan -->
          <section class="bg-white rounded-[40px] shadow-sm border border-gray-100 overflow-hidden mb-10">
            <div class="p-8 border-b border-gray-100">
              <h2 class="text-xl font-black text-gray-800 tracking-tight">Riwayat Rekomendasi Pakan</h2>
            </div>
            <div class="overflow-x-auto no-scrollbar">
              <table class="w-full text-left border-collapse">
                <thead>
                  <tr class="bg-gray-50/50 text-[11px] font-black text-gray-400 uppercase tracking-[0.15em] border-b border-gray-100">
                    <th class="px-8 py-5">Tag / Ras</th>
                    <th class="px-8 py-5">Pakan</th>
                    <th class="px-8 py-5 text-center">Efisiensi</th>
                  </tr>
                </thead>
                <tbody class="divide-y divide-gray-50">
                  <tr v-if="feedLogs.length === 0"><td colspan="3" class="py-12 text-center text-gray-300 font-bold">Belum ada riwayat rekomendasi pakan</td></tr>
                  <tr v-for="item in feedLogs" :key="item.id" class="hover:bg-green-50/20 transition-all">
                    <td class="px-8 py-5 font-black text-gray-800 uppercase">{{ item.tag_id }} <small class="text-[10px] text-gray-400 normal-case">{{ item.breed }}</small></td>
                    <td class="px-8 py-5 font-bold text-gray-600">{{ item.feed_name || '-' }}</td>
                    <td class="px-8 py-5 text-center"><span class="px-3 py-1 rounded-full text-[10px] font-black bg-green-50 text-green-600">{{ item.efficiency || '-' }}</span></td>
                  </tr>
                </tbody>
              </table>
            </div>
          </section>
        </template>
      </div>
    </main>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { MenuIcon } from 'lucide-vue-next'
import { Bar, Doughnut } from 'vue-chartjs'
import { Chart as ChartJS, Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale, ArcElement } from 'chart.js'

ChartJS.register(Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale, ArcElement)

const isSidebarOpen = ref(false)
const loading = ref(false)
const livestock = ref([])
const pens = ref([])
const feedLogs = ref([])

const { list: fetchLivestock } = useLivestock()
const { list: fetchPens } = usePen()
const { getFeedHistory } = useMl()

onMounted(async () => {
  loading.value = true
  try {
    const [ls, pn, fh] = await Promise.allSettled([fetchLivestock(), fetchPens(), getFeedHistory()])
    livestock.value = ls.status === 'fulfilled' ? (ls.value || []) : []
    pens.value = pn.status === 'fulfilled' ? (pn.value || []) : []
    feedLogs.value = fh.status === 'fulfilled' ? (fh.value || []).slice(0, 10) : []
  } catch (e) { console.error(e) } finally { loading.value = false }
})

const totalLivestock = computed(() => livestock.value.length)
const totalPopulasi = computed(() => pens.value.reduce((a, p) => a + (p.occupancy || 0), 0))

const healthCounts = computed(() => {
  let sehat = 0, perhatian = 0, sakit = 0
  for (const l of livestock.value) {
    const s = l.health_status || 'Sehat'
    if (s === 'Sakit' || s === 'Buruk') sakit++
    else if (s === 'Perlu Perhatian' || s === 'Observasi') perhatian++
    else sehat++
  }
  return { sehat, perhatian, sakit }
})
const healthyPercent = computed(() => {
  const t = totalLivestock.value || 1
  return Math.round((healthCounts.value.sehat / t) * 100)
})

const stats = computed(() => [
  { label: 'Total Ternak', value: totalLivestock.value, sub: 'individu' },
  { label: 'Total Kandang', value: pens.value.length, sub: 'unit' },
  { label: 'Populasi di Kandang', value: totalPopulasi.value, sub: 'ekor' },
  { label: 'Health Index', value: healthyPercent.value + '%', sub: 'sehat' },
])

const categoryChart = computed(() => {
  const map = {}
  for (const l of livestock.value) { map[l.category] = (map[l.category] || 0) + 1 }
  const labels = Object.keys(map)
  return {
    labels,
    datasets: [{ label: 'Jumlah', data: labels.map(k => map[k]), backgroundColor: '#1a402d', borderRadius: 8 }]
  }
})

const healthChart = computed(() => ({
  labels: ['Sehat', 'Perhatian', 'Sakit'],
  datasets: [{ data: [healthCounts.value.sehat, healthCounts.value.perhatian, healthCounts.value.sakit], backgroundColor: ['#10b981', '#fb923c', '#ef4444'], borderWidth: 0 }]
}))

const barOptions = { responsive: true, maintainAspectRatio: false, plugins: { legend: { display: false } }, scales: { y: { beginAtZero: true, ticks: { precision: 0 } }, x: { grid: { display: false } } } }
const doughnutOptions = { responsive: true, maintainAspectRatio: false, cutout: '80%', plugins: { legend: { display: false } } }
</script>

<style scoped>
.no-scrollbar::-webkit-scrollbar { display: none; }
.no-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }
.animate-in { animation: fadeIn 0.5s ease-out forwards; }
@keyframes fadeIn { from { opacity: 0; transform: translateY(20px); } to { opacity: 1; transform: translateY(0); } }
</style>
