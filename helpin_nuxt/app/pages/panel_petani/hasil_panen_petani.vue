<template>
  <div class="flex h-screen w-full bg-[#f4f7f5] font-sans overflow-hidden relative">
    
<SidebarPetani :isOpen="isSidebarOpen" @close="isSidebarOpen = false" />

    <main class="flex-1 flex flex-col overflow-y-auto relative w-full">
      <header class="flex justify-between items-center px-6 md:px-10 py-4 md:py-6 border-b border-gray-200 bg-white shadow-sm z-10 sticky top-0">
        <div class="flex items-center gap-4">
          <button @click="isSidebarOpen = true" class="md:hidden p-2 -ml-2 text-gray-600 hover:bg-gray-100 rounded-lg transition">
            <MenuIcon class="w-6 h-6" />
          </button>
          <div>
            <h1 class="text-xl md:text-2xl font-extrabold text-gray-800 leading-tight">Hasil Panen</h1>
            <p class="text-xs md:text-sm text-gray-500 font-medium mt-0.5">Analitik dan catatan produksi pascapanen</p>
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

      <div v-else class="p-4 md:p-10 flex flex-col w-full max-w-[100vw] gap-6">
        
        <!-- Charts Section -->
        <section class="grid grid-cols-1 lg:grid-cols-3 gap-6 w-full">
          <div class="bg-white rounded-2xl shadow-sm border border-gray-100 p-5 lg:col-span-2">
            <h3 class="text-sm font-bold text-gray-800 mb-4 flex items-center gap-2">
              <HistoryIcon class="w-4 h-4 text-green-700" />
              Tren Volume Panen (Kg)
            </h3>
            <div class="h-64 w-full relative">
              <Line v-if="chartDataLine.labels.length > 0" :data="chartDataLine" :options="chartOptionsLine" />
              <div v-else class="flex items-center justify-center h-full text-gray-400 text-sm">Belum ada data</div>
            </div>
          </div>

          <div class="bg-white rounded-2xl shadow-sm border border-gray-100 p-5">
            <h3 class="text-sm font-bold text-gray-800 mb-4 flex items-center gap-2">
              <LeafIcon class="w-4 h-4 text-green-700" />
              Distribusi Tanaman
            </h3>
            <div class="h-64 w-full relative flex justify-center">
              <Pie v-if="chartDataPie.labels.length > 0" :data="chartDataPie" :options="chartOptionsPie" />
              <div v-else class="flex items-center justify-center h-full text-gray-400 text-sm">Belum ada data</div>
            </div>
          </div>
        </section>

        <!-- Table -->
        <section class="bg-white rounded-2xl shadow-sm border border-gray-100 p-4 md:p-6 w-full flex flex-col">
          <div class="flex items-center gap-2 mb-6">
            <div class="w-2 h-2 rounded-full bg-[#1a402d]"></div>
            <h2 class="text-base md:text-lg font-bold text-gray-800">Daftar Panen Keseluruhan</h2>
            <span class="text-xs bg-gray-100 text-gray-500 px-2 py-0.5 rounded-full font-bold">{{ panenList.length }} record</span>
          </div>
          
          <div class="overflow-x-auto -mx-4 md:mx-0 px-4 md:px-0">
            <table class="w-full text-sm text-left min-w-[800px]">
              <thead class="text-xs text-green-800 uppercase bg-[#e1f0e5] font-bold">
                <tr>
                  <th scope="col" class="px-4 md:px-6 py-4 rounded-l-lg">Lahan & Tanaman</th>
                  <th scope="col" class="px-4 md:px-6 py-4">Tgl Panen</th>
                  <th scope="col" class="px-4 md:px-6 py-4">Berat (Kg)</th>
                  <th scope="col" class="px-4 md:px-6 py-4">Grade</th>
                  <th scope="col" class="px-4 md:px-6 py-4">Status</th>
                  <th scope="col" class="px-4 md:px-6 py-4 rounded-r-lg text-center">Detail</th>
                </tr>
              </thead>
              <tbody>
                <tr v-if="panenList.length === 0">
                  <td colspan="6" class="text-center py-10 text-gray-500 font-medium">Belum ada pencatatan hasil panen.</td>
                </tr>
                <tr v-for="item in panenList" :key="item.id" class="border-b border-gray-50 last:border-0 hover:bg-gray-50 transition-colors">
                  <td class="px-4 md:px-6 py-4">
                    <span class="font-bold text-gray-800 block">{{ item.cropName }}</span>
                    <span class="text-xs font-medium text-gray-500 flex items-center gap-1 mt-0.5">
                      <MapPinIcon class="w-3 h-3" /> {{ item.landName }}
                    </span>
                  </td>
                  <td class="px-4 md:px-6 py-4 font-semibold text-gray-600">{{ formatDate(item.harvested_at) }}</td>
                  <td class="px-4 md:px-6 py-4">
                    <span class="font-extrabold text-[#1a402d] text-base">{{ parseFloat(item.quantity).toFixed(1) }}</span>
                  </td>
                  <td class="px-4 md:px-6 py-4">
                    <span class="px-2.5 py-1 rounded-full text-xs font-bold"
                      :class="{
                        'bg-green-100 text-green-700': item.quality_grade === 'A',
                        'bg-yellow-100 text-yellow-700': item.quality_grade === 'B',
                        'bg-red-100 text-red-700': !item.quality_grade || item.quality_grade === 'C'
                      }">
                      {{ item.quality_grade || '-' }}
                    </span>
                  </td>
                  <td class="px-4 md:px-6 py-4">
                    <span class="px-2.5 py-1 rounded-full text-xs font-bold"
                      :class="parseFloat(item.quantity) > 0 ? 'bg-green-100 text-green-700' : 'bg-red-100 text-red-700'">
                      {{ parseFloat(item.quantity) > 0 ? 'Layak' : 'Tidak Layak' }}
                    </span>
                  </td>
                  <td class="px-4 md:px-6 py-4 flex justify-center">
                    <button @click="viewDetail(item)" class="bg-[#3b82f6] hover:bg-blue-600 text-white p-2 rounded-md transition shadow-sm" title="Lihat Detail">
                      <EyeIcon class="w-4 h-4" />
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
    <div v-if="isDetailModalOpen" class="fixed inset-0 z-50 flex items-center justify-center p-2 md:p-4">
      <div class="absolute inset-0 bg-black/60 backdrop-blur-sm" @click="isDetailModalOpen = false"></div>
      
      <div class="bg-white rounded-2xl w-full max-w-lg shadow-2xl relative z-10 flex flex-col max-h-[95vh] animate-in p-6">
        <div class="flex justify-between items-center mb-6">
          <h2 class="text-lg font-extrabold text-gray-800">Detail Panen</h2>
          <button @click="isDetailModalOpen = false" class="p-2 text-gray-400 hover:bg-gray-100 rounded-lg transition">
            <XIcon class="w-5 h-5" />
          </button>
        </div>

        <div class="space-y-4 overflow-y-auto">
          <div class="grid grid-cols-2 gap-4">
            <div class="bg-gray-50 rounded-xl p-3">
              <p class="text-[10px] font-bold text-gray-400 uppercase">Lahan</p>
              <p class="font-bold text-gray-800">{{ selectedItem?.landName }}</p>
            </div>
            <div class="bg-gray-50 rounded-xl p-3">
              <p class="text-[10px] font-bold text-gray-400 uppercase">Tanaman</p>
              <p class="font-bold text-gray-800">{{ selectedItem?.cropName }}</p>
            </div>
            <div class="bg-gray-50 rounded-xl p-3">
              <p class="text-[10px] font-bold text-gray-400 uppercase">Tanggal Panen</p>
              <p class="font-bold text-gray-800">{{ formatDate(selectedItem?.harvested_at) }}</p>
            </div>
            <div class="bg-green-50 rounded-xl p-3">
              <p class="text-[10px] font-bold text-green-600 uppercase">Berat Total</p>
              <p class="font-black text-[#1a402d] text-xl">{{ parseFloat(selectedItem?.quantity || 0).toFixed(1) }} Kg</p>
            </div>
            <div class="bg-blue-50 rounded-xl p-3">
              <p class="text-[10px] font-bold text-blue-600 uppercase">Grade Kualitas</p>
              <p class="font-black text-blue-800 text-lg">{{ selectedItem?.quality_grade || '-' }}</p>
            </div>
            <div class="rounded-xl p-3" :class="parseFloat(selectedItem?.quantity) > 0 ? 'bg-green-50' : 'bg-red-50'">
              <p class="text-[10px] font-bold uppercase" :class="parseFloat(selectedItem?.quantity) > 0 ? 'text-green-600' : 'text-red-600'">Status</p>
              <p class="font-black" :class="parseFloat(selectedItem?.quantity) > 0 ? 'text-green-700' : 'text-red-700'">
                {{ parseFloat(selectedItem?.quantity) > 0 ? 'Layak' : 'Tidak Layak' }}
              </p>
            </div>
          </div>
        </div>

        <div class="mt-6 pt-4 border-t border-gray-100 flex justify-end">
          <button @click="isDetailModalOpen = false" class="px-6 py-2.5 font-bold text-white bg-[#1a402d] rounded-lg shadow-md hover:bg-[#143222] transition">
            Tutup
          </button>
        </div>
      </div>
    </div>

  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { 
  ArchiveIcon, MapIcon, LeafIcon, WheatIcon, 
  HistoryIcon, MenuIcon, XIcon, MapPinIcon, EyeIcon
} from 'lucide-vue-next'

import { Line, Pie } from 'vue-chartjs'
import { 
  Chart as ChartJS, Title, Tooltip, Legend, BarElement, 
  CategoryScale, LinearScale, PointElement, LineElement, ArcElement
} from 'chart.js'

ChartJS.register(Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale, PointElement, LineElement, ArcElement)

const isSidebarOpen = ref(false)
const loading = ref(false)
const isDetailModalOpen = ref(false)
const selectedItem = ref(null)

const { list: fetchLands, getHarvests } = useLand()

const panenList = ref([])

onMounted(async () => {
  loading.value = true
  try {
    const lands = await fetchLands()
    const allHarvests = []
    
    for (const land of (lands || [])) {
      try {
        const harvests = await getHarvests(land.id)
        if (harvests && harvests.length > 0) {
          harvests.forEach(h => {
            allHarvests.push({
              ...h,
              landName: land.name || land.code || 'Lahan',
              cropName: h.plant_type || 'Tanaman'
            })
          })
        }
      } catch {}
    }
    panenList.value = allHarvests
  } catch (e) {
    console.error('Failed to load harvest data:', e)
    panenList.value = []
  } finally {
    loading.value = false
  }
})

// Charts
const chartDataLine = computed(() => {
  const sorted = [...panenList.value].sort((a, b) => new Date(a.harvested_at || 0) - new Date(b.harvested_at || 0))
  return {
    labels: sorted.map(item => formatDate(item.harvested_at)),
    datasets: [{
      label: 'Total Panen (Kg)',
      data: sorted.map(item => parseFloat(item.quantity) || 0),
      borderColor: '#1a402d',
      backgroundColor: 'rgba(26, 64, 45, 0.2)',
      tension: 0.4,
      fill: true
    }]
  }
})

const chartOptionsLine = {
  responsive: true,
  maintainAspectRatio: false,
  plugins: { legend: { position: 'bottom' } }
}

const chartDataPie = computed(() => {
  const cropTotals = {}
  panenList.value.forEach(item => {
    const name = item.cropName || 'Lainnya'
    cropTotals[name] = (cropTotals[name] || 0) + (parseFloat(item.quantity) || 0)
  })
  return {
    labels: Object.keys(cropTotals),
    datasets: [{
      data: Object.values(cropTotals),
      backgroundColor: ['#22c55e', '#eab308', '#3b82f6', '#f97316', '#a855f7'],
      borderWidth: 1
    }]
  }
})

const chartOptionsPie = {
  responsive: true,
  maintainAspectRatio: false,
  plugins: { legend: { position: 'right' } }
}

// Helpers
const formatDate = (d) => {
  if (!d) return '-'
  return new Date(d).toLocaleDateString('id-ID', { day: '2-digit', month: 'short', year: 'numeric' })
}

const viewDetail = (item) => {
  selectedItem.value = item
  isDetailModalOpen.value = true
}
</script>

<style scoped>
.animate-in { animation: animateIn 0.3s cubic-bezier(0.16, 1, 0.3, 1) forwards; }
@keyframes animateIn {
  from { opacity: 0; transform: scale(0.95) translateY(10px); }
  to { opacity: 1; transform: scale(1) translateY(0); }
}
</style>
