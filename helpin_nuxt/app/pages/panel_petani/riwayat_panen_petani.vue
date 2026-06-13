<template>
  <div class="flex h-screen w-full bg-[#f4f7f5] font-sans overflow-hidden relative text-slate-900">
    
<SidebarPetani :isOpen="isSidebarOpen" @close="isSidebarOpen = false" />

    <main class="flex-1 flex flex-col overflow-y-auto relative w-full no-scrollbar">
      <header class="flex justify-between items-center px-6 md:px-10 py-6 border-b border-gray-200 bg-white/90 backdrop-blur-md z-10 sticky top-0">
        <div class="flex items-center gap-4">
          <button @click="isSidebarOpen = true" class="md:hidden p-2 text-gray-600 border border-gray-100 rounded-lg">
            <MenuIcon class="w-6 h-6" />
          </button>
          <div>
            <h1 class="text-2xl font-black text-gray-800 tracking-tight">Riwayat Panen</h1>
            <p class="text-sm text-gray-500 font-medium mt-0.5">Data historis keseluruhan dari setiap lahan</p>
          </div>
        </div>
        <div>
          <div class="flex items-center gap-2 bg-green-100 px-3 py-1.5 md:px-4 md:py-2 rounded-lg border border-green-200">
            <div class="w-2 h-2 rounded-full bg-green-600 shrink-0 animate-pulse"></div>
            <span class="text-xs md:text-sm font-bold text-green-600 hidden sm:block">STATUS ONLINE</span>
          </div>
        </div>
      </header>

      <div v-if="loading" class="flex-1 flex items-center justify-center">
        <div class="animate-spin rounded-full h-10 w-10 border-b-2 border-[#1a402d]"></div>
      </div>

      <div v-else class="p-4 md:p-10 space-y-8">
        
        <!-- Summary Charts -->
        <section class="grid grid-cols-1 lg:grid-cols-3 gap-6">
          <div class="lg:col-span-2 bg-white p-8 rounded-[32px] border border-gray-100 shadow-sm relative overflow-hidden">
            <div class="flex justify-between items-center mb-8 relative z-10">
              <div>
                <h3 class="text-lg font-black text-gray-800">Tren Hasil Panen</h3>
                <p class="text-xs text-gray-400 font-bold uppercase tracking-widest">Timeline Produksi per Siklus</p>
              </div>
              <div class="bg-blue-50 text-blue-600 p-3 rounded-2xl"><TrendingUpIcon class="w-6 h-6" /></div>
            </div>
            <div class="h-72 w-full">
              <Line v-if="trendChartData.labels.length > 0" :data="trendChartData" :options="chartOptions" />
              <div v-else class="flex items-center justify-center h-full text-gray-400 text-sm">Belum ada data panen</div>
            </div>
          </div>
          
          <div class="bg-white p-8 rounded-[32px] border border-gray-100 shadow-sm flex flex-col items-center justify-center text-center">
            <h3 class="text-lg font-black text-gray-800 mb-6">Total Produksi</h3>
            <div class="relative h-56 w-56 flex items-center justify-center">
              <Doughnut v-if="qualityDistribution.datasets[0].data.some(d => d > 0)" :data="qualityDistribution" :options="doughnutOptions" />
            </div>
            <div class="mt-4 grid grid-cols-3 gap-3 w-full">
              <div class="bg-green-50 rounded-lg p-2 text-center">
                <p class="text-[10px] font-black text-green-700 uppercase">Grade A</p>
                <p class="font-bold text-sm">{{ qualityStats.gradeA }} Kg</p>
              </div>
              <div class="bg-yellow-50 rounded-lg p-2 text-center">
                <p class="text-[10px] font-black text-yellow-700 uppercase">Grade B</p>
                <p class="font-bold text-sm">{{ qualityStats.gradeB }} Kg</p>
              </div>
              <div class="bg-red-50 rounded-lg p-2 text-center">
                <p class="text-[10px] font-black text-red-700 uppercase">Lainnya</p>
                <p class="font-bold text-sm">{{ qualityStats.other }} Kg</p>
              </div>
            </div>
          </div>
        </section>

        <!-- Search -->
        <section class="flex flex-col md:flex-row gap-4">
          <div class="relative flex-1 w-full">
            <SearchIcon class="absolute left-5 top-1/2 -translate-y-1/2 w-5 h-5 text-gray-400" />
            <input 
              v-model="searchQuery" 
              type="text" 
              placeholder="Cari berdasarkan lahan atau tanaman..." 
              class="w-full pl-14 pr-6 py-4 bg-white border border-gray-100 rounded-2xl outline-none focus:ring-4 focus:ring-green-500/10 transition-all font-bold text-sm" 
            />
          </div>
        </section>

        <!-- Harvest History Table -->
        <section class="bg-white rounded-[32px] border border-gray-100 shadow-sm overflow-hidden mb-10">
          <div class="overflow-x-auto no-scrollbar">
            <table class="w-full text-left border-collapse min-w-[900px]">
              <thead>
                <tr class="bg-gray-50/50 border-b border-gray-100 text-[11px] font-black text-gray-400 uppercase tracking-[0.15em]">
                  <th class="px-6 py-5">ID Panen</th>
                  <th class="px-6 py-5">Lahan & Tanaman</th>
                  <th class="px-6 py-5">Tanggal Panen</th>
                  <th class="px-6 py-5 text-right">Berat (Kg)</th>
                  <th class="px-6 py-5">Grade</th>
                  <th class="px-6 py-5">Status</th>
                  <th class="px-6 py-5 text-center">Detail</th>
                </tr>
              </thead>
              <tbody class="divide-y divide-gray-50">
                <tr v-if="filteredHistory.length === 0">
                  <td colspan="7" class="text-center py-12 text-gray-400 font-medium">
                    {{ searchQuery ? 'Tidak ada data yang cocok' : 'Belum ada riwayat panen' }}
                  </td>
                </tr>
                <tr v-for="log in filteredHistory" :key="log.id" class="hover:bg-green-50/30 transition-all group">
                  <td class="px-6 py-5">
                    <span class="font-black text-gray-700 text-sm group-hover:text-green-800 transition">{{ log.displayId }}</span>
                    <span class="block text-[10px] text-gray-400 font-bold uppercase mt-0.5">{{ log.unit }}</span>
                  </td>
                  <td class="px-6 py-5">
                    <div class="flex items-center gap-3">
                      <div class="w-10 h-10 rounded-xl bg-green-100 text-green-700 flex items-center justify-center font-black text-lg">
                        {{ (log.landName || 'L').charAt(0) }}
                      </div>
                      <div>
                        <span class="font-bold text-gray-700 block">{{ log.landName }}</span>
                        <span class="text-xs text-gray-400 font-medium">{{ log.cropName || 'Tanaman' }}</span>
                      </div>
                    </div>
                  </td>
                  <td class="px-6 py-5">
                    <span class="font-bold text-gray-700">{{ formatDate(log.harvested_at) }}</span>
                  </td>
                  <td class="px-6 py-5 text-right">
                    <span class="text-lg font-black text-[#1a402d]">{{ parseFloat(log.quantity).toFixed(1) }}</span>
                  </td>
                  <td class="px-6 py-5">
                    <span class="px-2.5 py-1 rounded-full text-xs font-bold"
                      :class="{
                        'bg-green-100 text-green-700': log.quality_grade === 'A',
                        'bg-yellow-100 text-yellow-700': log.quality_grade === 'B',
                        'bg-red-100 text-red-700': log.quality_grade === 'C' || !log.quality_grade
                      }">
                      Grade {{ log.quality_grade || '-' }}
                    </span>
                  </td>
                  <td class="px-6 py-5">
                    <span class="px-2.5 py-1 rounded-full text-xs font-bold"
                      :class="{
                        'bg-green-100 text-green-700': log.feasibility === 'Layak',
                        'bg-red-100 text-red-700': log.feasibility === 'Tidak Layak'
                      }">
                      {{ log.feasibility || 'Layak' }}
                    </span>
                  </td>
                  <td class="px-6 py-5 text-center">
                    <button @click="viewDetail(log)" class="p-3 bg-gray-50 text-gray-400 rounded-xl hover:bg-[#1a402d] hover:text-white transition shadow-sm">
                      <EyeIcon class="w-5 h-5" />
                    </button>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </section>
      </div>
    </main>

    <!-- Detail Modal -->
    <div v-if="isDetailModalOpen" class="fixed inset-0 z-50 flex items-center justify-center p-4">
      <div class="absolute inset-0 bg-[#0c1a13]/80 backdrop-blur-lg" @click="isDetailModalOpen = false"></div>
      
      <div class="bg-white rounded-[32px] w-full max-w-3xl max-h-[90vh] overflow-hidden shadow-2xl relative z-10 flex flex-col animate-in">
        
        <div class="bg-gradient-to-br from-[#1a402d] via-[#1a402d] to-[#2d5c41] p-8 text-white relative">
          <button @click="isDetailModalOpen = false" class="absolute top-6 right-6 p-2 bg-white/10 hover:bg-white/20 rounded-full transition">
            <XIcon class="w-5 h-5" />
          </button>
          <div class="flex items-center gap-3 mb-3">
            <span class="px-3 py-1 bg-green-400 text-[#1a402d] text-[10px] font-black rounded-full uppercase">Riwayat Panen</span>
            <span class="text-white/50 font-mono text-sm">{{ selectedLog?.displayId }}</span>
          </div>
          <h2 class="text-3xl font-black tracking-tight">{{ selectedLog?.landName }}</h2>
          <p class="text-sm text-white/60 font-medium mt-1">{{ selectedLog?.cropName }} • {{ formatDate(selectedLog?.harvested_at) }}</p>
        </div>

        <div class="p-8 overflow-y-auto space-y-6">
          
          <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
            <div class="bg-gray-50 rounded-xl p-4 text-center">
              <p class="text-[10px] font-black text-gray-400 uppercase mb-1">Berat Total</p>
              <p class="text-xl font-black text-[#1a402d]">{{ parseFloat(selectedLog?.quantity || 0).toFixed(1) }} Kg</p>
            </div>
            <div class="bg-gray-50 rounded-xl p-4 text-center">
              <p class="text-[10px] font-black text-gray-400 uppercase mb-1">Grade</p>
              <p class="text-xl font-black text-blue-700">{{ selectedLog?.quality_grade || '-' }}</p>
            </div>
            <div class="bg-gray-50 rounded-xl p-4 text-center">
              <p class="text-[10px] font-black text-gray-400 uppercase mb-1">Status</p>
              <p class="text-sm font-black" :class="selectedLog?.feasibility === 'Layak' ? 'text-green-700' : 'text-red-700'">
                {{ selectedLog?.feasibility || 'Layak' }}
              </p>
            </div>
            <div class="bg-gray-50 rounded-xl p-4 text-center">
              <p class="text-[10px] font-black text-gray-400 uppercase mb-1">Satuan</p>
              <p class="text-sm font-black text-gray-700">{{ selectedLog?.unit || 'Kg' }}</p>
            </div>
          </div>

          <div v-if="selectedLog?.prediction" class="bg-blue-50 rounded-xl p-5 space-y-3">
            <h4 class="text-xs font-black text-blue-700 uppercase tracking-wider">Data Prediksi Terkait</h4>
            <div class="grid grid-cols-2 gap-3 text-sm">
              <div>
                <span class="text-gray-500">Sumber:</span>
                <span class="font-bold ml-1 uppercase">{{ selectedLog.prediction.source }}</span>
              </div>
              <div>
                <span class="text-gray-500">Confidence:</span>
                <span class="font-bold ml-1">{{ (selectedLog.prediction.confidence * 100).toFixed(1) }}%</span>
              </div>
              <div>
                <span class="text-gray-500">Estimasi Revenue:</span>
                <span class="font-bold ml-1">Rp {{ formatCurrency(selectedLog.prediction.revenue) }}</span>
              </div>
              <div>
                <span class="text-gray-500">Status Nutrisi:</span>
                <span class="font-bold ml-1">{{ selectedLog.prediction.nutrient_status }}</span>
              </div>
            </div>
            <div v-if="selectedLog.prediction.recommendations?.length" class="mt-3 pt-3 border-t border-blue-200">
              <p class="text-[10px] font-bold text-blue-600 uppercase mb-2">Rekomendasi:</p>
              <ul class="text-xs text-gray-600 space-y-1">
                <li v-for="r in selectedLog.prediction.recommendations" :key="r" class="flex items-start gap-1">
                  <span class="text-green-500 mt-0.5">•</span> {{ r }}
                </li>
              </ul>
            </div>
          </div>

          <div class="text-center pt-4">
            <p class="text-xs text-gray-400 italic mb-4">Data ini bersifat Read-Only dan merupakan arsip riwayat panen.</p>
            <button @click="isDetailModalOpen = false" class="px-8 py-3 bg-[#1a402d] text-white rounded-2xl font-bold shadow-lg hover:bg-[#143222] transition">
              Tutup
            </button>
          </div>
        </div>
      </div>
    </div>

  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { 
  MenuIcon, XIcon, SearchIcon, EyeIcon, TrendingUpIcon
} from 'lucide-vue-next'

import { Line, Doughnut } from 'vue-chartjs'
import { 
  Chart as ChartJS, Title, Tooltip, Legend, BarElement, 
  CategoryScale, LinearScale, PointElement, LineElement, ArcElement, Filler 
} from 'chart.js'

ChartJS.register(Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale, PointElement, LineElement, ArcElement, Filler)

const isSidebarOpen = ref(false)
const isDetailModalOpen = ref(false)
const selectedLog = ref(null)
const searchQuery = ref('')
const loading = ref(false)

const { list: fetchLands, getHarvests } = useLand()

const historyData = ref([])

onMounted(async () => {
  loading.value = true
  try {
    const lands = await fetchLands()
    const allHarvests = []
    
    for (const land of (lands || [])) {
      try {
        const harvests = await getHarvests(land.id)
        if (harvests && harvests.length) {
          harvests.forEach(harvest => {
            allHarvests.push({
              ...harvest,
              landName: land.name || land.code || 'Lahan',
              cropName: harvest.plant_type || 'Tanaman',
              displayId: harvest.id ? harvest.id.substring(0, 8).toUpperCase() : `PNN-${Date.now()}`,
              feasibility: parseFloat(harvest.quantity) > 0 ? 'Layak' : 'Tidak Layak',
              prediction: null // Will be populated when prediction data exists
            })
          })
        }
      } catch {}
    }
    
    historyData.value = allHarvests
  } catch (e) {
    console.error('Failed to load harvest history:', e)
    historyData.value = []
  } finally {
    loading.value = false
  }
})

// Quality stats from real data
const qualityStats = computed(() => {
  let gradeA = 0, gradeB = 0, other = 0
  historyData.value.forEach(h => {
    const qty = parseFloat(h.quantity) || 0
    if (h.quality_grade === 'A') gradeA += qty
    else if (h.quality_grade === 'B') gradeB += qty
    else other += qty
  })
  return {
    gradeA: gradeA.toFixed(0),
    gradeB: gradeB.toFixed(0),
    other: other.toFixed(0)
  }
})

// Charts
const trendChartData = computed(() => {
  const sorted = [...historyData.value].sort((a, b) => 
    new Date(a.harvested_at || 0) - new Date(b.harvested_at || 0)
  )
  return {
    labels: sorted.map(h => formatDate(h.harvested_at)),
    datasets: [
      {
        label: 'Hasil Panen (Kg)',
        data: sorted.map(h => parseFloat(h.quantity) || 0),
        borderColor: '#10b981',
        backgroundColor: 'rgba(16, 185, 129, 0.1)',
        fill: true,
        tension: 0.4
      }
    ]
  }
})

const qualityDistribution = computed(() => {
  let gradeA = 0, gradeB = 0, other = 0
  historyData.value.forEach(h => {
    const qty = parseFloat(h.quantity) || 0
    if (h.quality_grade === 'A') gradeA += qty
    else if (h.quality_grade === 'B') gradeB += qty
    else other += qty
  })
  return {
    labels: ['Grade A', 'Grade B', 'Lainnya'],
    datasets: [{
      data: [gradeA, gradeB, other],
      backgroundColor: ['#16a34a', '#eab308', '#ef4444'],
      borderWidth: 0
    }]
  }
})

const chartOptions = { 
  responsive: true, 
  maintainAspectRatio: false, 
  plugins: { legend: { position: 'bottom', labels: { font: { weight: 'bold', size: 10 } } } } 
}
const doughnutOptions = { 
  responsive: true, 
  maintainAspectRatio: false, 
  cutout: '70%', 
  plugins: { legend: { display: false } } 
}

// Helpers
const formatDate = (d) => {
  if (!d) return '-'
  return new Date(d).toLocaleDateString('id-ID', { day: '2-digit', month: 'short', year: 'numeric' })
}

const formatCurrency = (amount) => {
  if (!amount) return '0'
  return new Intl.NumberFormat('id-ID').format(Math.round(amount))
}

const viewDetail = (log) => { 
  selectedLog.value = log
  isDetailModalOpen.value = true 
}

const filteredHistory = computed(() => {
  return historyData.value.filter(item => {
    const s = searchQuery.value.toLowerCase()
    return (item.landName || '').toLowerCase().includes(s) || 
           (item.cropName || '').toLowerCase().includes(s) ||
           (item.displayId || '').toLowerCase().includes(s)
  })
})
</script>

<style scoped>
.no-scrollbar::-webkit-scrollbar { display: none; }
.no-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }

.animate-in { animation: fadeIn 0.4s ease-out forwards; }
@keyframes fadeIn {
  from { opacity: 0; transform: translateY(20px); }
  to { opacity: 1; transform: translateY(0); }
}
</style>
