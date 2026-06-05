<template>
  <div class="flex h-screen w-full bg-[#f4f7f5] font-sans overflow-hidden relative">
    
    <SidebarPetani :isOpen="isSidebarOpen" @close="isSidebarOpen = false" />

    <main class="flex-1 flex flex-col overflow-y-auto relative w-full no-scrollbar">
      <header class="flex justify-between items-center px-6 md:px-10 py-5 border-b border-gray-200 bg-white/90 backdrop-blur-md shadow-sm z-10 sticky top-0">
        <div class="flex items-center gap-4">
          <button @click="isSidebarOpen = true" class="md:hidden p-2 -ml-2 text-gray-600 hover:bg-gray-100 rounded-lg transition">
            <MenuIcon class="w-6 h-6" />
          </button>
          <div>
            <h1 class="text-xl md:text-2xl font-black text-gray-800 tracking-tight">Dashboard</h1>
            <p class="text-xs md:text-sm text-gray-500 font-medium mt-0.5">Overview Agrikultur, Lahan, dan Inventori terintegrasi</p>
          </div>
        </div>
        <div class="flex items-center gap-3">
          <div class="flex items-center gap-2 bg-green-100 px-3 py-1.5 md:px-4 md:py-2 rounded-lg border border-green-200">
            <div class="w-2 h-2 rounded-full bg-green-600 shrink-0 animate-pulse"></div>
            <span class="text-xs md:text-sm font-bold text-green-600 hidden sm:block uppercase">Status Online</span>
          </div>
        </div>
      </header>

      <div class="p-4 md:p-8 flex flex-col gap-6 md:gap-8 w-full max-w-[100vw]">
        
        <section class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 md:gap-6">
          <div class="bg-white p-6 rounded-2xl border border-gray-100 shadow-sm flex items-center gap-4 group hover:shadow-md transition">
            <div class="w-14 h-14 rounded-full bg-green-50 flex items-center justify-center text-green-600 group-hover:bg-green-600 group-hover:text-white transition">
              <TrendingUpIcon class="w-7 h-7" />
            </div>
            <div>
              <p class="text-xs font-bold text-gray-400 uppercase tracking-wider mb-1">Total Produksi YTD</p>
              <h3 class="text-2xl font-black text-gray-800">142.5 <span class="text-sm text-gray-500 font-bold">Ton</span></h3>
            </div>
          </div>
          
          <div class="bg-white p-6 rounded-2xl border border-gray-100 shadow-sm flex items-center gap-4 group hover:shadow-md transition">
            <div class="w-14 h-14 rounded-full bg-blue-50 flex items-center justify-center text-blue-600 group-hover:bg-blue-600 group-hover:text-white transition">
              <TargetIcon class="w-7 h-7" />
            </div>
            <div>
              <p class="text-xs font-bold text-gray-400 uppercase tracking-wider mb-1">Akurasi Prediksi</p>
              <h3 class="text-2xl font-black text-gray-800">94.2 <span class="text-sm text-gray-500 font-bold">%</span></h3>
            </div>
          </div>

          <div class="bg-white p-6 rounded-2xl border border-gray-100 shadow-sm flex items-center gap-4 group hover:shadow-md transition">
            <div class="w-14 h-14 rounded-full bg-orange-50 flex items-center justify-center text-orange-500 group-hover:bg-orange-500 group-hover:text-white transition">
              <MapIcon class="w-7 h-7" />
            </div>
            <div>
              <p class="text-xs font-bold text-gray-400 uppercase tracking-wider mb-1">Lahan Aktif</p>
              <h3 class="text-2xl font-black text-gray-800">3 <span class="text-sm text-gray-500 font-bold">Blok / 9.7 Ha</span></h3>
            </div>
          </div>

          <div class="bg-white p-6 rounded-2xl border border-gray-100 shadow-sm flex items-center gap-4 group hover:shadow-md transition">
            <div class="w-14 h-14 rounded-full bg-red-50 flex items-center justify-center text-red-500 group-hover:bg-red-500 group-hover:text-white transition">
              <AlertTriangleIcon class="w-7 h-7" />
            </div>
            <div>
              <p class="text-xs font-bold text-gray-400 uppercase tracking-wider mb-1">Peringatan Stok</p>
              <h3 class="text-2xl font-black text-gray-800">2 <span class="text-sm text-gray-500 font-bold">Item Menipis</span></h3>
            </div>
          </div>
        </section>

        <section class="grid grid-cols-1 lg:grid-cols-3 gap-6">
          
          <div class="bg-white rounded-3xl shadow-sm border border-gray-100 p-6 lg:col-span-2">
            <div class="flex justify-between items-center mb-6">
              <div>
                <h2 class="text-lg font-black text-gray-800">Tren Produksi vs Prediksi</h2>
                <p class="text-xs font-medium text-gray-400 mt-1">Evaluasi tonase panen aktual melawan algoritma prediksi HELP-IN</p>
              </div>
              <select class="bg-gray-50 border border-gray-200 text-gray-700 text-xs font-bold rounded-lg px-3 py-2 outline-none">
                <option>Tahun 2026</option>
                <option>Tahun 2025</option>
              </select>
            </div>
            <div class="relative w-full h-[300px]">
              <Bar :data="comboChartData" :options="comboChartOptions" />
            </div>
          </div>

          <div class="bg-white rounded-3xl shadow-sm border border-gray-100 p-6 flex flex-col items-center justify-center">
            <h2 class="text-lg font-black text-gray-800 w-full text-left mb-2">Kumulatif Kualitas</h2>
            <p class="text-xs font-medium text-gray-400 w-full text-left mb-6">Distribusi grade panen secara keseluruhan</p>
            
            <div class="relative h-52 w-52 flex items-center justify-center">
              <Doughnut :data="qualityChartData" :options="doughnutOptions" />
              <div class="absolute inset-0 flex flex-col items-center justify-center">
                <span class="text-2xl font-black text-gray-800">142t</span>
                <p class="text-[10px] text-gray-400 font-black uppercase tracking-widest">Total</p>
              </div>
            </div>
            
            <div class="w-full grid grid-cols-3 gap-2 mt-6">
              <div class="text-center bg-green-50 rounded-lg p-2">
                <p class="text-[10px] font-black text-green-700 uppercase">Grade A</p>
                <p class="font-bold text-sm text-gray-800">75%</p>
              </div>
              <div class="text-center bg-yellow-50 rounded-lg p-2">
                <p class="text-[10px] font-black text-yellow-700 uppercase">Grade B</p>
                <p class="font-bold text-sm text-gray-800">20%</p>
              </div>
              <div class="text-center bg-red-50 rounded-lg p-2">
                <p class="text-[10px] font-black text-red-700 uppercase">Afkir</p>
                <p class="font-bold text-sm text-gray-800">5%</p>
              </div>
            </div>
          </div>
        </section>

        <section class="grid grid-cols-1 lg:grid-cols-3 gap-6">
          
          <div class="bg-[#1a402d] rounded-3xl shadow-xl p-6 lg:col-span-1 text-white flex flex-col">
            <h2 class="text-sm font-black uppercase tracking-widest text-green-400 mb-2 flex items-center gap-2">
              <LeafIcon class="w-4 h-4" /> Soil Health Index
            </h2>
            <p class="text-xs text-gray-300 font-medium mb-8">Rata-rata kualitas tanah dari 3 blok lahan aktif berdasarkan sensor terakhir.</p>
            
            <div class="space-y-6 mt-auto">
              <div v-for="soil in avgSoilHealth" :key="soil.label" class="space-y-2">
                <div class="flex justify-between items-center">
                  <span class="text-xs font-bold">{{ soil.label }}</span>
                  <span class="text-xs font-black">{{ soil.value }}{{ soil.unit }}</span>
                </div>
                <div class="h-1.5 w-full bg-white/10 rounded-full overflow-hidden">
                  <div class="bg-green-400 h-full rounded-full transition-all duration-1000" :style="`width: ${soil.percent}%`"></div>
                </div>
              </div>
            </div>
          </div>

          <div class="bg-white rounded-3xl shadow-sm border border-gray-100 p-6 lg:col-span-2 flex flex-col">
            <div class="flex items-center gap-2 mb-6">
              <div class="w-2 h-2 rounded-full bg-[#1a402d]"></div>
              <h2 class="text-base md:text-lg font-black text-gray-800">Action Plan: Prediksi Panen Terdekat</h2>
            </div>
            
            <div class="overflow-x-auto no-scrollbar">
              <table class="w-full text-sm text-left min-w-[600px]">
                <thead class="text-[11px] text-gray-400 uppercase tracking-widest border-b border-gray-100">
                  <tr>
                    <th class="pb-4 font-black">Timeline</th>
                    <th class="pb-4 font-black">Komoditas & Lahan</th>
                    <th class="pb-4 font-black text-right">Est. Tonase</th>
                    <th class="pb-4 font-black text-center">Status</th>
                    <th class="pb-4 font-black text-center">Detail</th>
                  </tr>
                </thead>
                <tbody class="divide-y divide-gray-50">
                  <tr v-for="item in upcomingHarvests" :key="item.id" class="hover:bg-green-50/30 transition-colors group">
                    <td class="py-4">
                      <span class="font-black text-gray-800 block">{{ item.date }}</span>
                      <span class="text-xs font-bold text-red-500">{{ item.daysLeft }} Hari lagi</span>
                    </td>
                    <td class="py-4">
                      <span class="font-bold text-gray-800 block">{{ item.crop }}</span>
                      <span class="text-[11px] font-medium text-gray-400">{{ item.details.lahan }}</span>
                    </td>
                    <td class="py-4 text-right">
                      <span class="text-lg font-black text-[#1a402d]">{{ item.amount }} <small>t</small></span>
                    </td>
                    <td class="py-4 text-center">
                      <span class="px-2 py-1 bg-yellow-100 text-yellow-700 text-[10px] font-black rounded uppercase tracking-wider">Persiapan</span>
                    </td>
                    <td class="py-4 text-center">
                      <button @click="openModal(item)" class="p-2 bg-gray-50 text-gray-400 rounded-lg hover:bg-[#1a402d] hover:text-white transition shadow-sm inline-flex">
                        <EyeIcon class="w-4 h-4" />
                      </button>
                    </td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </section>

      </div>
    </main>

    <div v-if="isModalOpen" class="fixed inset-0 z-50 flex items-center justify-center p-4">
      <div class="absolute inset-0 bg-[#0c1a13]/80 backdrop-blur-sm animate-in fade-in" @click="closeModal"></div>
      
      <div class="bg-white rounded-3xl w-full max-w-4xl shadow-2xl relative z-10 flex flex-col max-h-[90vh] animate-in slide-in-from-bottom duration-300 overflow-hidden">
        <div class="flex justify-between items-center p-6 border-b border-gray-100 shrink-0 bg-gray-50/50">
          <div class="flex items-center gap-3">
            <div class="p-2 bg-green-100 text-green-700 rounded-xl hidden sm:block">
              <SproutIcon class="w-6 h-6" />
            </div>
            <div>
              <h2 class="text-xl font-black text-gray-800">Intelijen Prediksi Panen</h2>
              <p class="text-xs text-gray-500 font-medium font-mono mt-1">BATCH ID: {{ selectedData?.id }}</p>
            </div>
          </div>
          <button @click="closeModal" class="p-2 text-gray-400 hover:bg-gray-200 rounded-xl transition">
            <XIcon class="w-6 h-6" />
          </button>
        </div>

        <div class="p-6 md:p-8 overflow-y-auto grid grid-cols-1 lg:grid-cols-2 gap-8">
          <div class="space-y-6">
            <h3 class="text-xs font-black text-gray-400 uppercase tracking-widest border-b border-gray-100 pb-2">Data Agronomi</h3>
            <div class="grid grid-cols-2 gap-y-6 gap-x-4">
              <div>
                <p class="text-[10px] font-bold text-gray-400 uppercase mb-1">Komoditas</p>
                <p class="font-black text-gray-800">{{ selectedData?.crop }}</p>
              </div>
              <div>
                <p class="text-[10px] font-bold text-gray-400 uppercase mb-1">Varietas</p>
                <p class="font-black text-gray-800">{{ selectedData?.details.varietas }}</p>
              </div>
              <div>
                <p class="text-[10px] font-bold text-gray-400 uppercase mb-1">Lahan & Luasan</p>
                <p class="font-black text-gray-800">{{ selectedData?.details.lahan }}</p>
                <p class="text-xs text-gray-500">{{ selectedData?.details.luas }} Hektar</p>
              </div>
              <div>
                <p class="text-[10px] font-bold text-gray-400 uppercase mb-1">Tanggal Tanam</p>
                <p class="font-black text-gray-800">{{ selectedData?.details.tgl_tanam }}</p>
              </div>
            </div>
            
            <div class="bg-[#1a402d] text-white rounded-2xl p-6 mt-4 shadow-lg">
              <p class="text-[10px] font-black text-green-400 uppercase mb-1">Estimasi Total Output</p>
              <h2 class="text-4xl font-black">{{ selectedData?.amount }} <span class="text-sm italic font-medium">Tonase</span></h2>
            </div>
          </div>

          <div class="flex flex-col border border-gray-100 rounded-3xl p-6 shadow-sm">
            <h3 class="text-xs font-black text-gray-400 uppercase tracking-widest mb-2">Simulasi Kualitas AI</h3>
            <p class="text-[11px] text-gray-500 mb-6 line-clamp-2">Proyeksi distribusi *grade* berdasarkan analitik kesehatan tanah, input nutrisi historis, dan histori varietas.</p>
            
            <div class="flex-1 min-h-[220px] relative flex justify-center items-center">
              <Pie v-if="modalChartData" :data="modalChartData" :options="pieChartOptions" />
            </div>
          </div>
        </div>

        <div class="p-6 border-t border-gray-100 bg-gray-50 flex justify-end gap-3 shrink-0">
          <button @click="closeModal" class="px-6 py-3 font-bold text-gray-600 bg-white border border-gray-200 hover:bg-gray-100 rounded-xl transition">Tutup Panel</button>
        </div>
      </div>
    </div>

  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { 
  MenuIcon, TrendingUpIcon, TargetIcon, MapIcon, AlertTriangleIcon, 
  LeafIcon, EyeIcon, XIcon, SproutIcon 
} from 'lucide-vue-next'
import { Bar, Doughnut, Pie } from 'vue-chartjs'
import { 
  Chart as ChartJS, Title, Tooltip, Legend, BarElement, LineElement, PointElement,
  CategoryScale, LinearScale, ArcElement, LineController 
} from 'chart.js'

ChartJS.register(Title, Tooltip, Legend, BarElement, LineElement, PointElement, CategoryScale, LinearScale, ArcElement, LineController)

const isSidebarOpen = ref(false)

// ==========================================
// DATA MOCKUP (Gabungan dari 4 Referensi)
// ==========================================

// Data Upcoming Harvest (Tabel Dashboard Bawah)
const upcomingHarvests = ref([
  { 
    id: 'BATCH-NX1', date: '12 Mar 2026', daysLeft: 3, crop: 'Jagung Manis', amount: 10, 
    details: { varietas: 'Bonanza F1', lahan: 'Blok A - Lahan Utara', luas: 2.5, tgl_tanam: '12 Des 2025' },
    gradePrediction: [6.5, 2.5, 1.0] 
  },
  { 
    id: 'BATCH-NX2', date: '15 Mar 2026', daysLeft: 6, crop: 'Padi Hibrida', amount: 25, 
    details: { varietas: 'Mapan P-05', lahan: 'Blok C - Sawah Timur', luas: 4.0, tgl_tanam: '05 Des 2025' },
    gradePrediction: [18.0, 5.0, 2.0] 
  },
  { 
    id: 'BATCH-NX3', date: '20 Mar 2026', daysLeft: 11, crop: 'Kedelai', amount: 8, 
    details: { varietas: 'Grobogan', lahan: 'Blok B - Lahan Kering', luas: 3.2, tgl_tanam: '01 Jan 2026' },
    gradePrediction: [5.0, 2.0, 1.0] 
  }
])

// Data Tren Panen (Combo Bar & Line)
const comboChartData = ref({
  labels: ['Jan', 'Feb', 'Mar', 'Apr', 'Mei', 'Jun', 'Jul', 'Agt', 'Sep', 'Okt', 'Nov', 'Des'],
  datasets: [
    {
      type: 'line',
      label: 'Prediksi Algoritma (Ton)',
      data: [45, 60, 65, 80, 55, 65, 60, 75, 65, 55, 68, 60],
      borderColor: '#94a3b8',
      borderWidth: 2,
      borderDash: [5, 5],
      tension: 0.4,
      pointRadius: 0
    },
    {
      type: 'bar',
      label: 'Realita Panen (Ton)',
      backgroundColor: '#1a402d',
      borderRadius: 6,
      maxBarThickness: 24,
      data: [50, 62, 68, 75, 58, 68, 62, 70, 62, 50, 70, 62]
    }
  ]
})

const comboChartOptions = {
  responsive: true, maintainAspectRatio: false,
  interaction: { mode: 'index', intersect: false },
  plugins: { 
    legend: { position: 'top', align: 'end', labels: { usePointStyle: true, boxWidth: 8, font: { weight: 'bold', size: 11 } } }
  },
  scales: {
    y: { grid: { color: '#f1f5f9', drawBorder: false }, ticks: { font: { weight: 'bold' } } },
    x: { grid: { display: false, drawBorder: false }, ticks: { font: { weight: 'bold', color: '#64748b' } } }
  }
}

// Data Doughnut Kualitas Panen
const qualityChartData = ref({
  labels: ['Grade A', 'Grade B', 'Afkir/Reject'],
  datasets: [{
    data: [75, 20, 5],
    backgroundColor: ['#16a34a', '#eab308', '#ef4444'],
    borderWidth: 0,
    hoverOffset: 4
  }]
})

const doughnutOptions = {
  responsive: true, maintainAspectRatio: false, cutout: '75%',
  plugins: { legend: { display: false }, tooltip: { padding: 12, cornerRadius: 8 } }
}

// Data Soil Health Index
const avgSoilHealth = ref([
  { label: 'Nitrogen Content', value: 108, unit: 'mg', percent: 85 },
  { label: 'Phosphorus Content', value: 38, unit: 'mg', percent: 65 },
  { label: 'Potassium Content', value: 82, unit: 'mg', percent: 78 }
])

// ==========================================
// MODAL LOGIC
// ==========================================
const isModalOpen = ref(false)
const selectedData = ref(null)
const modalChartData = ref(null)

const pieChartOptions = {
  responsive: true, maintainAspectRatio: false,
  plugins: {
    legend: { position: 'right', labels: { font: { weight: 'bold' }, color: '#374151', padding: 15 } }
  }
}

const openModal = (item) => {
  selectedData.value = item;
  modalChartData.value = {
    labels: ['Grade A (Super)', 'Grade B (Standar)', 'Afkir / Reject'],
    datasets: [{
      backgroundColor: ['#16a34a', '#eab308', '#ef4444'],
      borderWidth: 2, borderColor: '#ffffff',
      data: item.gradePrediction
    }]
  }
  isModalOpen.value = true;
}

const closeModal = () => {
  isModalOpen.value = false;
  setTimeout(() => { selectedData.value = null; modalChartData.value = null; }, 300);
}
</script>

<style scoped>
.no-scrollbar::-webkit-scrollbar { display: none; }
.no-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }

.animate-in { animation: fadeIn 0.4s cubic-bezier(0.16, 1, 0.3, 1) forwards; }
@keyframes fadeIn {
  from { opacity: 0; transform: translateY(15px); }
  to { opacity: 1; transform: translateY(0); }
}
</style>