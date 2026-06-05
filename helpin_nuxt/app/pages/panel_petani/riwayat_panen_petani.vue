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
            <p class="text-sm text-gray-500 font-medium mt-0.5">Data historis komprehensif berdasarkan input pakar dan sensor</p>
          </div>
        </div>
        <div>
          <div class="flex items-center gap-2 bg-green-100 px-3 py-1.5 md:px-4 md:py-2 rounded-lg border border-green-200">
            <div class="w-2 h-2 rounded-full bg-green-600 shrink-0 animate-pulse"></div>
            <span class="text-xs md:text-sm font-bold text-green-600 hidden sm:block">STATUS ONLINE</span>
            <span class="text-xs md:text-sm font-bold text-green-600 sm:hidden">ONLINE</span>
          </div>
        </div>
      </header>

      <div class="p-4 md:p-10 space-y-8">
        
        <section class="grid grid-cols-1 lg:grid-cols-3 gap-6">
          <div class="lg:col-span-2 bg-white p-8 rounded-[32px] border border-gray-100 shadow-sm relative overflow-hidden">
            <div class="flex justify-between items-center mb-8 relative z-10">
              <div>
                <h3 class="text-lg font-black text-gray-800">Historical Yield Performance</h3>
                <p class="text-xs text-gray-400 font-bold uppercase tracking-widest">Timeline Analisis Produksi</p>
              </div>
              <div class="bg-blue-50 text-blue-600 p-3 rounded-2xl"><TrendingUpIcon class="w-6 h-6" /></div>
            </div>
            <div class="h-72 w-full"><Line :data="trendChartData" :options="chartOptions" /></div>
          </div>
          
          <div class="bg-white p-8 rounded-[32px] border border-gray-100 shadow-sm flex flex-col items-center justify-center text-center">
            <h3 class="text-lg font-black text-gray-800 mb-6">Akurasi Prediksi</h3>
            <div class="relative h-56 w-56 flex items-center justify-center">
              <Doughnut :data="accuracyChartData" :options="doughnutOptions" />
              <div class="absolute inset-0 flex flex-col items-center justify-center">
                <span class="text-4xl font-black text-gray-800">92.4%</span>
                <p class="text-[10px] text-green-600 font-black uppercase">High Precision</p>
              </div>
            </div>
            <p class="mt-6 text-xs text-gray-400 font-medium italic">Berdasarkan data input sensor NPK & Algoritma HELP-IN Pakar</p>
          </div>
        </section>

        <section class="grid grid-cols-1 md:grid-cols-3 gap-6">
          <div class="bg-[#1a402d] p-8 rounded-[32px] shadow-xl text-white col-span-1">
            <h3 class="text-sm font-black uppercase tracking-widest text-green-400 mb-6">Soil Health Index</h3>
            <div class="space-y-6">
              <div v-for="soil in avgSoilHealth" :key="soil.label" class="space-y-2">
                <div class="flex justify-between items-center">
                  <span class="text-xs font-bold">{{ soil.label }}</span>
                  <span class="text-xs font-black">{{ soil.value }}{{ soil.unit }}</span>
                </div>
                <div class="h-1.5 w-full bg-white/10 rounded-full overflow-hidden">
                  <div class="bg-green-400 h-full rounded-full" :style="`width: ${soil.percent}%`"></div>
                </div>
              </div>
            </div>
          </div>
          <div class="md:col-span-2 bg-white p-8 rounded-[32px] border border-gray-100 shadow-sm">
            <h3 class="text-lg font-black text-gray-800 mb-4">Kumulatif Input Nutrisi Lahan</h3>
            <div class="h-56 w-full"><Bar :data="nutrientCumulativeData" :options="barOptions" /></div>
          </div>
        </section>

        <section class="flex flex-col md:flex-row gap-4">
          <div class="relative flex-1 w-full">
            <SearchIcon class="absolute left-5 top-1/2 -translate-y-1/2 w-5 h-5 text-gray-400" />
            <input 
              v-model="searchQuery" 
              type="text" 
              placeholder="Cari ID Batch, Tanaman, atau Lahan..." 
              class="w-full pl-14 pr-6 py-4 bg-white border border-gray-100 rounded-2xl outline-none focus:ring-4 focus:ring-green-500/10 transition-all font-bold text-sm" 
            />
          </div>
        </section>

        <section class="bg-white rounded-[40px] border border-gray-100 shadow-sm overflow-hidden mb-10">
          <div class="overflow-x-auto no-scrollbar">
            <table class="w-full text-left border-collapse min-w-[1000px]">
              <thead>
                <tr class="bg-gray-50/50 border-b border-gray-100 text-[11px] font-black text-gray-400 uppercase tracking-[0.15em]">
                  <th class="px-8 py-6">Batch Tracking</th>
                  <th class="px-8 py-6">Commodity & Area</th>
                  <th class="px-8 py-6">Intelligence Meta</th>
                  <th class="px-8 py-6 text-right">Yield Record</th>
                  <th class="px-8 py-6 text-center">Audit</th>
                </tr>
              </thead>
              <tbody class="divide-y divide-gray-50">
                <tr v-for="log in filteredHistory" :key="log.id" class="hover:bg-green-50/30 transition-all group">
                  <td class="px-8 py-6">
                    <div class="flex flex-col">
                      <span class="font-black text-gray-800 text-lg group-hover:text-green-800 transition">{{ log.id }}</span>
                      <div class="flex items-center gap-2 mt-1">
                        <span class="text-[10px] bg-gray-100 text-gray-500 px-2 py-0.5 rounded font-bold uppercase tracking-tighter">Verified</span>
                        <span class="text-[10px] text-gray-400 font-bold uppercase tracking-widest">{{ formatDate(log.harvestDate) }}</span>
                      </div>
                    </div>
                  </td>
                  <td class="px-8 py-6">
                    <div class="flex items-center gap-4">
                      <div class="w-12 h-12 rounded-2xl bg-green-100 text-green-700 flex items-center justify-center font-black text-xl shadow-inner">{{ log.cropName.charAt(0) }}</div>
                      <div class="flex flex-col">
                        <span class="font-black text-gray-700">{{ log.cropName }}</span>
                        <span class="text-xs text-gray-400 font-medium italic">{{ log.landName }}</span>
                      </div>
                    </div>
                  </td>
                  <td class="px-8 py-6">
                    <div class="grid grid-cols-2 gap-x-6 gap-y-2">
                      <div class="flex items-center gap-2">
                        <div class="w-1.5 h-1.5 rounded-full bg-orange-500"></div>
                        <span class="text-[11px] font-bold text-gray-500 uppercase">pH: {{ log.soilIntelligence.ph }}</span>
                      </div>
                      <div class="flex items-center gap-2">
                        <div class="w-1.5 h-1.5 rounded-full bg-blue-500"></div>
                        <span class="text-[11px] font-bold text-gray-500 uppercase">Moist: {{ log.soilIntelligence.moisture }}%</span>
                      </div>
                      <div class="flex items-center gap-2">
                        <div class="w-1.5 h-1.5 rounded-full bg-green-500"></div>
                        <span class="text-[11px] font-bold text-gray-500 uppercase">Nutrients: {{ log.soilIntelligence.nutrientStatus }}</span>
                      </div>
                    </div>
                  </td>
                  <td class="px-8 py-6 text-right">
                    <div class="flex flex-col">
                      <span class="text-2xl font-black text-[#1a402d]">{{ log.totalAmount }} <small class="text-xs italic font-bold">Ton</small></span>
                      <span class="text-[10px] font-black text-green-600 uppercase">Target: {{ log.predictionYield }}t</span>
                    </div>
                  </td>
                  <td class="px-8 py-6 text-center">
                    <button @click="viewDetail(log)" class="p-4 bg-gray-50 text-gray-400 rounded-2xl hover:bg-[#1a402d] hover:text-white hover:rotate-12 transition-all shadow-sm">
                      <EyeIcon class="w-6 h-6" />
                    </button>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </section>
      </div>
    </main>

    <div v-if="isDetailModalOpen" class="fixed inset-0 z-50 flex items-center justify-center p-4">
      <div class="absolute inset-0 bg-[#0c1a13]/80 backdrop-blur-lg animate-in fade-in" @click="isDetailModalOpen = false"></div>
      
      <div class="bg-white rounded-[50px] w-full max-w-4xl max-h-[90vh] overflow-hidden shadow-[0_40px_100px_rgba(0,0,0,0.5)] relative z-10 flex flex-col animate-in slide-in-from-bottom duration-500">
        
        <div class="bg-gradient-to-br from-[#1a402d] via-[#1a402d] to-[#2d5c41] p-12 text-white relative">
          <button @click="isDetailModalOpen = false" class="absolute top-10 right-10 p-3 bg-white/10 hover:bg-white/20 rounded-full transition-all">
            <XIcon class="w-6 h-6" />
          </button>
          <div class="flex items-center gap-3 mb-4">
            <span class="px-3 py-1 bg-green-400 text-[#1a402d] text-[10px] font-black rounded-full uppercase tracking-widest">Intelligence Verified</span>
            <span class="text-white/50 font-mono text-sm tracking-widest">LOG: {{ selectedLog?.id }}</span>
          </div>
          <h2 class="text-5xl font-black tracking-tighter mb-2">Detail Riwayat Panen</h2>
          <p class="text-lg text-white/60 font-medium italic">{{ selectedLog?.landName }} • Batch Siklus {{ selectedLog?.cropName }}</p>
        </div>

        <div class="p-12 overflow-y-auto no-scrollbar grid grid-cols-1 md:grid-cols-2 gap-12 bg-white">
          
          <div class="space-y-6">
            <h3 class="text-xs font-black text-green-700 uppercase tracking-[0.2em] flex items-center gap-2">
              <ThermometerIcon class="w-4 h-4" /> Soil Intelligence Meta
            </h3>
            <div class="bg-gray-50 rounded-[32px] p-8 border border-gray-100 grid grid-cols-2 gap-8">
              <div v-for="(val, key) in selectedLog?.soilIntelligence.detailed" :key="key">
                <p class="text-[10px] font-black text-gray-400 uppercase mb-1">{{ key }} Level</p>
                <div class="flex items-baseline gap-1">
                  <span class="text-2xl font-black text-gray-800">{{ val }}</span>
                  <span class="text-[10px] font-bold text-gray-400">mg/kg</span>
                </div>
              </div>
              <div class="col-span-2 pt-4 border-t border-gray-200">
                <p class="text-[10px] font-black text-gray-400 uppercase mb-3 text-center tracking-widest">Overall Analysis</p>
                <div class="flex items-center gap-3">
                  <span class="flex-1 py-3 px-4 bg-white rounded-2xl text-center text-sm font-black border border-gray-100 shadow-sm">pH {{ selectedLog?.soilIntelligence.ph }}</span>
                  <span class="flex-1 py-3 px-4 bg-white rounded-2xl text-center text-sm font-black border border-gray-100 shadow-sm text-blue-600">{{ selectedLog?.soilIntelligence.moisture }}% Moist</span>
                </div>
              </div>
            </div>
          </div>

          <div class="space-y-6">
            <h3 class="text-xs font-black text-blue-700 uppercase tracking-[0.2em] flex items-center gap-2">
              <DropletsIcon class="w-4 h-4" /> Nutrient & Treatment Log
            </h3>
            <div class="bg-blue-50/30 rounded-[32px] p-8 border border-blue-100 space-y-6">
              <div v-for="input in selectedLog?.nutrientInputs" :key="input.name" class="flex justify-between items-center">
                <div class="flex items-center gap-3">
                  <div class="w-10 h-10 bg-white rounded-xl flex items-center justify-center shadow-sm">
                    <ArchiveIcon class="w-5 h-5 text-blue-600" />
                  </div>
                  <div class="flex flex-col">
                    <span class="text-sm font-black text-gray-800">{{ input.name }}</span>
                    <span class="text-[10px] text-gray-400 font-bold uppercase tracking-tighter">Applied on cycle {{ input.cycle }}</span>
                  </div>
                </div>
                <span class="text-lg font-black text-blue-900">{{ input.amount }}kg</span>
              </div>
            </div>
          </div>

          <div class="md:col-span-2 pt-8 border-t border-gray-100">
             <div class="flex justify-between items-end mb-8">
               <div>
                <h3 class="text-2xl font-black text-gray-800">Final Harvest Audit</h3>
                <p class="text-sm text-gray-400 font-medium italic">Perbandingan output prediksi vs realita lapangan</p>
               </div>
               <div class="text-right">
                 <p class="text-[10px] font-black text-green-600 uppercase mb-1">Akurasi Siklus</p>
                 <p class="text-3xl font-black text-green-800">{{ selectedLog?.accuracy }}%</p>
               </div>
             </div>
             
             <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
               <div class="p-8 rounded-[32px] bg-gray-50 border border-gray-100 shadow-inner">
                 <p class="text-[10px] font-black text-gray-400 uppercase mb-2 text-center">Output Prediksi</p>
                 <p class="text-4xl font-black text-center text-gray-400">{{ selectedLog?.predictionYield }} <small class="text-xs italic">t</small></p>
               </div>
               <div class="p-8 rounded-[32px] bg-[#1a402d] text-white shadow-2xl relative overflow-hidden group">
                 <div class="absolute inset-0 bg-white/5 opacity-0 group-hover:opacity-100 transition-opacity"></div>
                 <p class="text-[10px] font-black text-green-400 uppercase mb-2 text-center relative z-10">Output Realita</p>
                 <p class="text-5xl font-black text-center relative z-10">{{ selectedLog?.totalAmount }} <small class="text-sm italic">t</small></p>
               </div>
               <div class="flex flex-col justify-between gap-4">
                 <div v-for="g in ['gradeA', 'gradeB', 'reject']" :key="g" class="flex justify-between items-center p-4 bg-white border border-gray-100 rounded-2xl">
                   <span class="text-xs font-black uppercase text-gray-400 tracking-widest">{{ g.replace('grade', 'Grade ') }}</span>
                   <span class="font-black text-gray-800">{{ selectedLog?.[g] }}t</span>
                 </div>
               </div>
             </div>
          </div>
        </div>

        <div class="p-10 bg-gray-50 border-t border-gray-100 text-center">
          <p class="text-xs text-gray-400 font-medium mb-6 italic">Laporan ini bersifat Read-Only dan merupakan arsip intelijen HELP-IN.</p>
          <button @click="isDetailModalOpen = false" class="px-12 py-5 bg-[#1a402d] text-white rounded-3xl font-black shadow-xl shadow-green-900/40 hover:scale-[1.05] active:scale-95 transition-all">Selesai Meninjau Intelijen</button>
        </div>
      </div>
    </div>

  </div>
</template>

<script setup>
import { ref, computed } from 'vue'
import { 
  LayoutDashboardIcon, ArchiveIcon, MapIcon, WheatIcon, HistoryIcon, 
  LogOutIcon, MenuIcon, XIcon, FileDownIcon, SearchIcon, EyeIcon, 
  TrendingUpIcon, ThermometerIcon, DropletsIcon
} from 'lucide-vue-next'

// CHART INTEGRATION
import { Line, Doughnut, Bar } from 'vue-chartjs'
import { 
  Chart as ChartJS, Title, Tooltip, Legend, BarElement, 
  CategoryScale, LinearScale, PointElement, LineElement, ArcElement, Filler 
} from 'chart.js'

ChartJS.register(Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale, PointElement, LineElement, ArcElement, Filler)

const isSidebarOpen = ref(false)
const isDetailModalOpen = ref(false)
const selectedLog = ref(null)
const searchQuery = ref('')

// DATASET FULL INTELLIGENCE (Dummy Berdasarkan Flowchart Pakar)
const historyData = ref([
  { 
    id: 'BATCH-2026-X1', 
    landName: 'Lahan Utama - Blok A', 
    cropName: 'Jagung Hibrida Bisi-2', 
    harvestDate: '2026-04-12',
    predictionYield: 17.5,
    totalAmount: 18.2,
    accuracy: 96.0,
    gradeA: 13.5, gradeB: 3.5, reject: 1.2,
    soilIntelligence: {
      ph: 6.8, moisture: 45, nutrientStatus: 'Optimal',
      detailed: { Nitrogen: 120, Phosphorus: 45, Potassium: 85, Magnesium: 12 }
    },
    nutrientInputs: [
      { name: 'Pupuk Urea (N)', amount: 250, cycle: 'Vegetatif' },
      { name: 'NPK Phonska', amount: 400, cycle: 'Generatif' },
      { name: 'Kapur Dolomit', amount: 150, cycle: 'Pre-Planting' }
    ]
  },
  { 
    id: 'BATCH-2026-X2', 
    landName: 'Sawah Timur - Blok C', 
    cropName: 'Padi Inpari 32', 
    harvestDate: '2026-05-02',
    predictionYield: 32.0,
    totalAmount: 29.5,
    accuracy: 92.2,
    gradeA: 20.0, gradeB: 7.5, reject: 2.0,
    soilIntelligence: {
      ph: 6.2, moisture: 78, nutrientStatus: 'Slightly Deficit',
      detailed: { Nitrogen: 95, Phosphorus: 30, Potassium: 70, Magnesium: 8 }
    },
    nutrientInputs: [
      { name: 'SP-36 (P)', amount: 200, cycle: 'Vegetatif' },
      { name: 'NPK Mutiara', amount: 500, cycle: 'Generatif' },
      { name: 'POC (Organik)', amount: 50, cycle: 'Maintenance' }
    ]
  },
  { 
    id: 'BATCH-2026-X3', 
    landName: 'Lahan Kering - Blok B', 
    cropName: 'Kedelai Anjasmoro', 
    harvestDate: '2026-05-28',
    predictionYield: 10.5,
    totalAmount: 11.2,
    accuracy: 93.8,
    gradeA: 8.5, gradeB: 2.0, reject: 0.7,
    soilIntelligence: {
      ph: 6.5, moisture: 35, nutrientStatus: 'Optimal',
      detailed: { Nitrogen: 110, Phosphorus: 38, Potassium: 90, Magnesium: 15 }
    },
    nutrientInputs: [
      { name: 'ZA (Sulfat)', amount: 100, cycle: 'Early Growth' },
      { name: 'NPK Khusus', amount: 250, cycle: 'Generatif' }
    ]
  }
])

// CHART COMPUTATIONS
const totalTonnage = computed(() => historyData.value.reduce((acc, curr) => acc + curr.totalAmount, 0).toFixed(1))

const trendChartData = computed(() => ({
  labels: historyData.value.map(i => i.id.split('-').pop()),
  datasets: [
    { label: 'Prediksi (Ton)', data: historyData.value.map(i => i.predictionYield), borderColor: '#cbd5e1', backgroundColor: 'transparent', borderDash: [5, 5], tension: 0.4 },
    { label: 'Realita (Ton)', data: historyData.value.map(i => i.totalAmount), borderColor: '#10b981', backgroundColor: 'rgba(16, 185, 129, 0.1)', fill: true, tension: 0.4 }
  ]
}))

const accuracyChartData = computed(() => ({
  labels: ['Akurasi', 'Deviasi'],
  datasets: [{ data: [92.4, 7.6], backgroundColor: ['#10b981', '#f1f5f9'], borderWidth: 0 }]
}))

const nutrientCumulativeData = computed(() => {
  const inputs = {};
  historyData.value.forEach(d => {
    d.nutrientInputs.forEach(n => {
      inputs[n.name] = (inputs[n.name] || 0) + n.amount
    })
  })
  return {
    labels: Object.keys(inputs),
    datasets: [{ label: 'Total Penggunaan (kg)', data: Object.values(inputs), backgroundColor: '#1a402d', borderRadius: 8 }]
  }
})

const avgSoilHealth = [
  { label: 'Nitrogen Content', value: 108, unit: 'mg', percent: 85 },
  { label: 'Phosporus Content', percent: 65, value: 38, unit: 'mg' },
  { label: 'Potassium Content', percent: 78, value: 82, unit: 'mg' }
]

// OPTIONS
const chartOptions = { responsive: true, maintainAspectRatio: false, plugins: { legend: { position: 'bottom', labels: { font: { weight: 'bold', size: 10 } } } } }
const doughnutOptions = { responsive: true, maintainAspectRatio: false, cutout: '85%', plugins: { legend: { display: false } } }
const barOptions = { responsive: true, maintainAspectRatio: false, plugins: { legend: { display: false } }, scales: { y: { beginAtZero: true } } }

// HELPERS
const formatDate = (d) => new Date(d).toLocaleDateString('id-ID', { day: '2-digit', month: 'short', year: 'numeric' })
const viewDetail = (log) => { selectedLog.value = log; isDetailModalOpen.value = true }
const exportData = () => { alert('Generating Full Intelligence Audit Report (PDF)...') }

const filteredHistory = computed(() => {
  return historyData.value.filter(item => {
    const s = searchQuery.value.toLowerCase()
    return item.cropName.toLowerCase().includes(s) || item.id.toLowerCase().includes(s) || item.landName.toLowerCase().includes(s)
  })
})
</script>

<style scoped>
.no-scrollbar::-webkit-scrollbar { display: none; }
.no-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }

.active-menu::before {
  content: ""; position: absolute; right: 0; top: -24px; width: 24px; height: 24px;
  background-color: transparent; border-bottom-right-radius: 24px;
  box-shadow: 12px 12px 0 12px #f4f7f5; pointer-events: none;
}

.active-menu::after {
  content: ""; position: absolute; right: 0; bottom: -24px; width: 24px; height: 24px;
  background-color: transparent; border-top-right-radius: 24px;
  box-shadow: 12px -12px 0 12px #f4f7f5; pointer-events: none;
}

.animate-in { animation: fadeIn 0.5s ease-out forwards; }
@keyframes fadeIn {
  from { opacity: 0; transform: translateY(20px); }
  to { opacity: 1; transform: translateY(0); }
}
</style>