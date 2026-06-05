<template>
  <div class="flex h-screen w-full bg-[#f4f7f5] font-sans overflow-hidden relative text-slate-900">
    
    <div v-if="isSidebarOpen" @click="isSidebarOpen = false" class="fixed inset-0 bg-black/60 z-30 md:hidden backdrop-blur-sm transition-opacity"></div>

    <aside :class="['w-[280px] bg-[#1a402d] text-white flex flex-col justify-between shrink-0 absolute inset-y-0 left-0 z-40 transform transition-transform duration-300 ease-in-out md:relative md:translate-x-0', isSidebarOpen ? 'translate-x-0' : '-translate-x-full']">
      <div class="overflow-y-auto overflow-x-hidden h-full no-scrollbar flex flex-col justify-between">
        <div>
          <div class="px-6 md:px-8 py-8 flex justify-between items-center">
            <div class="flex items-baseline gap-1">
              <h1 class="text-3xl font-extrabold tracking-wider">HELPIN</h1>
              <span class="text-[10px] uppercase tracking-widest font-semibold text-gray-300">Breeder</span>
            </div>
            <button @click="isSidebarOpen = false" class="md:hidden text-white hover:text-red-400"><XIcon class="w-6 h-6" /></button>
          </div>

          <div class="px-6 md:px-8 mb-8 border-b border-[#23533b] pb-6 bg-[#173a28]">
            <div class="flex items-center gap-2 mb-2">
              <div class="w-2 h-2 rounded-full bg-green-500"></div>
              <span class="text-xs font-semibold text-green-400">PENGGUNA AKTIF</span>
            </div>
            <h2 class="text-xl font-bold mb-2">Petani Suki</h2>
            <span class="inline-block px-3 py-1 text-[10px] font-bold border border-green-600 text-green-400 rounded uppercase tracking-widest">PETERNAK PRO</span>
          </div>

          <nav class="flex flex-col gap-1 pl-4">
            <NuxtLink to="#" class="active-menu relative flex items-center gap-4 px-4 py-4 bg-[#f4f7f5] text-[#1a402d] rounded-l-full font-bold shadow-[-5px_0_10px_rgba(0,0,0,0.05)]">
              <LayoutDashboardIcon class="w-5 h-5" /> Dashboard
            </NuxtLink>
            <NuxtLink v-for="menu in menus" :key="menu.name" to="#" class="flex items-center gap-4 px-4 py-4 text-gray-200 hover:bg-[#23533b] rounded-l-full transition-colors font-medium">
              <component :is="menu.icon" class="w-5 h-5" /> {{ menu.name }}
            </NuxtLink>
          </nav>
        </div>

        <div class="p-6">
          <div class="bg-[#143222] rounded-xl p-4 flex items-center justify-between border border-[#1d462f]">
            <div class="min-w-0">
              <span class="text-[10px] text-gray-400 font-semibold uppercase block">Authorized Admin</span>
              <p class="text-sm font-bold truncate pr-2">Admin Suki SUPER</p>
            </div>
            <LogOutIcon class="w-5 h-5 text-red-400 cursor-pointer" />
          </div>
        </div>
      </div>
    </aside>

    <main class="flex-1 flex flex-col overflow-y-auto relative w-full no-scrollbar">
      <header class="flex justify-between items-center px-6 md:px-10 py-6 border-b border-gray-200 bg-white/80 backdrop-blur-md z-10 sticky top-0">
        <div class="flex items-center gap-4">
          <button @click="isSidebarOpen = true" class="md:hidden p-2 text-gray-600 hover:bg-gray-100 rounded-lg transition"><MenuIcon class="w-6 h-6" /></button>
          <div>
            <h1 class="text-xl md:text-2xl font-black text-gray-800 leading-tight">Expert Breeding Intelligence</h1>
            <p class="text-xs md:text-sm text-gray-500 font-medium mt-0.5 italic">Berdasarkan Audit Sensor & Alur Logika Sistem Pakar</p>
          </div>
        </div>
        <div class="flex items-center gap-4">
           <div class="hidden lg:flex flex-col text-right">
              <span class="text-[10px] font-black text-gray-400 uppercase">Akurasi Sistem</span>
              <span class="text-sm font-bold text-green-600">98.2% Precision</span>
           </div>
           <button @click="exportReport" class="bg-[#1a402d] text-white p-2.5 rounded-xl shadow-lg shadow-green-900/20 hover:scale-105 transition-all">
             <DownloadIcon class="w-5 h-5" />
           </button>
        </div>
      </header>

      <div class="p-4 md:p-10 space-y-8">
        
        <section class="grid grid-cols-1 lg:grid-cols-3 gap-6">
          <div class="lg:col-span-2 bg-white p-8 rounded-[32px] border border-gray-100 shadow-sm">
            <div class="flex justify-between items-start mb-6">
              <div>
                <h3 class="text-lg font-black text-gray-800">Evaluasi Pertumbuhan & Produksi</h3>
                <p class="text-xs text-gray-400 font-bold uppercase tracking-widest">Realitas vs Prediksi Sistem Pakar</p>
              </div>
              <div class="bg-green-50 text-green-600 px-3 py-1 rounded-full text-xs font-black">+12.4% Yield</div>
            </div>
            <div class="h-72 w-full">
              <Line :data="intelligenceTrendData" :options="chartOptions" />
            </div>
          </div>

          <div class="bg-white p-8 rounded-[32px] border border-gray-100 shadow-sm flex flex-col items-center justify-center">
            <h3 class="text-lg font-black text-gray-800 mb-6 text-center">Status Kesehatan Global</h3>
            <div class="relative h-56 w-56">
              <Doughnut :data="healthDistributionData" :options="doughnutOptions" />
              <div class="absolute inset-0 flex flex-col items-center justify-center pointer-events-none">
                <span class="text-4xl font-black text-gray-800">94%</span>
                <span class="text-[10px] text-green-600 font-black uppercase">Good Health</span>
              </div>
            </div>
          </div>
        </section>

        <section class="grid grid-cols-1 md:grid-cols-4 gap-4">
           <div v-for="sensor in sensorMetrics" :key="sensor.label" class="bg-white p-6 rounded-3xl border border-gray-100 shadow-sm group hover:border-green-200 transition-all">
             <div class="flex items-center gap-4">
                <div :class="['p-3 rounded-2xl transition-colors', sensor.alert ? 'bg-red-50 text-red-500' : 'bg-green-50 text-green-600']">
                  <component :is="sensor.icon" class="w-6 h-6" />
                </div>
                <div>
                  <p class="text-[10px] font-black text-gray-400 uppercase tracking-widest">{{ sensor.label }}</p>
                  <p class="text-xl font-black text-gray-800">{{ sensor.value }}<span class="text-xs font-bold text-gray-400 ml-1">{{ sensor.unit }}</span></p>
                </div>
             </div>
             <div class="mt-4 flex items-center gap-2">
                <div class="h-1 flex-1 bg-gray-100 rounded-full overflow-hidden">
                  <div :class="['h-full rounded-full', sensor.alert ? 'bg-red-500' : 'bg-green-500']" :style="`width: ${sensor.progress}%`"></div>
                </div>
                <span class="text-[10px] font-bold text-gray-400">{{ sensor.status }}</span>
             </div>
           </div>
        </section>

        <section class="bg-white rounded-[40px] shadow-sm border border-gray-100 overflow-hidden mb-10">
          <div class="p-8 border-b border-gray-100 flex flex-col md:flex-row md:items-center justify-between gap-4">
            <div class="flex items-center gap-3">
              <div class="w-2 h-8 bg-[#1a402d] rounded-full"></div>
              <h2 class="text-xl font-black text-gray-800 tracking-tight">Intelligence Recommendation Log</h2>
            </div>
            <div class="relative">
              <SearchIcon class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-gray-400" />
              <input v-model="searchQuery" type="text" placeholder="Cari ID atau Jenis Ternak..." class="pl-10 pr-4 py-2.5 bg-gray-50 border-none rounded-xl text-sm outline-none focus:ring-2 focus:ring-green-500/20 transition-all w-full md:w-64 font-bold" />
            </div>
          </div>
          
          <div class="overflow-x-auto no-scrollbar">
            <table class="w-full text-left border-collapse">
              <thead>
                <tr class="bg-gray-50/50 text-[11px] font-black text-gray-400 uppercase tracking-[0.15em] border-b border-gray-100">
                  <th class="px-8 py-5">Audit ID & Ternak</th>
                  <th class="px-8 py-5">Populasi</th>
                  <th class="px-8 py-5 text-center">Vitality Score</th>
                  <th class="px-8 py-5">Diagnosis Intelligence</th>
                  <th class="px-8 py-5 text-center">Detail Audit</th>
                </tr>
              </thead>
              <tbody class="divide-y divide-gray-50">
                <tr v-for="item in filteredTableData" :key="item.id" class="hover:bg-green-50/20 transition-all group">
                  <td class="px-8 py-6">
                    <div class="flex items-center gap-4">
                      <div class="w-12 h-12 rounded-2xl bg-white border border-gray-100 flex items-center justify-center text-[#1a402d] font-black text-lg shadow-sm">{{ item.jenis.charAt(0) }}</div>
                      <div class="flex flex-col">
                        <span class="font-black text-gray-800 text-base">{{ item.jenis }}</span>
                        <span class="text-[10px] text-gray-400 font-bold uppercase tracking-widest tracking-tighter italic">LOG-{{ item.id }}</span>
                      </div>
                    </div>
                  </td>
                  <td class="px-8 py-6">
                    <div class="flex flex-col">
                      <span class="text-xl font-black text-[#1a402d]">{{ item.jumlah }} <small class="text-[10px] font-bold">EKOR</small></span>
                      <span class="text-[10px] font-bold text-gray-400 uppercase">Growth Cycle Day 42</span>
                    </div>
                  </td>
                  <td class="px-8 py-6">
                    <div class="flex flex-col items-center gap-1">
                       <span :class="['px-3 py-1 rounded-full text-[10px] font-black uppercase tracking-widest', item.kesehatan === 'Kurang Baik' ? 'bg-red-50 text-red-600' : 'bg-green-50 text-green-600']">
                        {{ item.kesehatan }}
                      </span>
                      <div class="h-1 w-20 bg-gray-100 rounded-full overflow-hidden">
                        <div :class="['h-full', item.kesehatan === 'Kurang Baik' ? 'bg-red-500' : 'bg-green-500']" :style="`width: ${item.kesehatan === 'Kurang Baik' ? 40 : 95}%`"></div>
                      </div>
                    </div>
                  </td>
                  <td class="px-8 py-6">
                    <div class="flex flex-col">
                      <p class="font-bold text-red-500 text-sm leading-relaxed">{{ item.informasi }}</p>
                      <p class="text-[10px] text-gray-400 font-medium italic mt-1">Saran Pakar: Berikan konsentrat tipe-B + Vitamin C</p>
                    </div>
                  </td>
                  <td class="px-8 py-6 text-center">
                    <button @click="openAuditDetail(item)" class="p-3 bg-gray-50 text-gray-400 rounded-2xl hover:bg-[#1a402d] hover:text-white hover:rotate-12 transition-all shadow-sm">
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

    <div v-if="isAuditModalOpen" class="fixed inset-0 z-50 flex items-center justify-center p-4">
      <div class="absolute inset-0 bg-[#0c1a13]/80 backdrop-blur-md animate-in fade-in" @click="isAuditModalOpen = false"></div>
      
      <div class="bg-white rounded-[50px] w-full max-w-4xl max-h-[92vh] overflow-hidden shadow-[0_40px_100px_rgba(0,0,0,0.5)] relative z-10 flex flex-col animate-in slide-in-from-bottom-10 duration-500 no-scrollbar">
        
        <div class="bg-gradient-to-br from-[#1a402d] via-[#1a402d] to-[#2d5c41] p-12 text-white relative">
          <button @click="isAuditModalOpen = false" class="absolute top-10 right-10 p-3 bg-white/10 hover:bg-white/20 rounded-full transition-all"><XIcon class="w-6 h-6" /></button>
          <div class="flex items-center gap-3 mb-4">
            <span class="px-3 py-1 bg-green-400 text-[#1a402d] text-[10px] font-black rounded-full uppercase tracking-widest">Digital Audit Verified</span>
            <span class="text-white/50 font-mono text-sm tracking-widest tracking-widest">#AUD-2026-{{ selectedAudit?.id }}</span>
          </div>
          <h2 class="text-5xl font-black tracking-tighter mb-2 italic">Breeding Intelligence</h2>
          <p class="text-lg text-white/60 font-medium italic">Data Rinci Nutrisi & Jejak Perkembangan Ternak</p>
        </div>

        <div class="p-12 overflow-y-auto no-scrollbar grid grid-cols-1 md:grid-cols-2 gap-12 bg-white">
          
          <div class="space-y-6">
            <h3 class="text-xs font-black text-green-700 uppercase tracking-[0.2em] flex items-center gap-2">
              <SoupIcon class="w-4 h-4" /> Nutrient & Feed Input Log
            </h3>
            <div class="bg-gray-50 rounded-[32px] p-8 border border-gray-100 space-y-6 shadow-inner">
               <div v-for="feed in feedAnalysis" :key="feed.name" class="flex justify-between items-center">
                 <div class="flex flex-col">
                   <span class="text-sm font-black text-gray-800">{{ feed.name }}</span>
                   <span class="text-[10px] text-gray-400 font-bold uppercase tracking-tighter">Requirement: {{ feed.target }}%</span>
                 </div>
                 <div class="flex items-baseline gap-1">
                   <span class="text-2xl font-black text-green-800">{{ feed.actual }}</span>
                   <span class="text-[10px] font-bold text-gray-400">%</span>
                 </div>
               </div>
               <div class="pt-6 border-t border-gray-200">
                  <p class="text-[10px] font-black text-gray-400 uppercase mb-4 text-center tracking-widest">Expert Recommendation</p>
                  <div class="bg-white p-5 rounded-2xl text-xs font-bold leading-relaxed text-slate-600 border border-gray-100 shadow-sm italic">
                    "Sistem mendeteksi defisiensi kalsium. Tambahkan premix mineral 2% dari total pakan harian selama 7 hari ke depan untuk pemulihan optimal."
                  </div>
               </div>
            </div>
          </div>

          <div class="space-y-6">
            <h3 class="text-xs font-black text-blue-700 uppercase tracking-[0.2em] flex items-center gap-2">
              <ActivityIcon class="w-4 h-4" /> Real-time Biometrics
            </h3>
            <div class="grid grid-cols-2 gap-4">
              <div v-for="bio in biometricData" :key="soil.label" class="bg-blue-50/30 p-5 rounded-3xl border border-blue-100 flex flex-col items-center justify-center">
                <p class="text-[10px] font-black text-blue-400 uppercase mb-1">{{ bio.label }}</p>
                <p class="text-2xl font-black text-blue-900">{{ bio.value }}<small class="text-xs ml-0.5">{{ bio.unit }}</small></p>
              </div>
            </div>
            <div class="bg-[#1a402d] rounded-[32px] p-8 text-white relative overflow-hidden shadow-2xl">
               <div class="absolute -right-10 -top-10 w-40 h-40 bg-white/5 rounded-full"></div>
               <p class="text-[10px] font-black text-green-400 uppercase mb-2 tracking-widest">Weight Prediction Yield</p>
               <p class="text-5xl font-black mb-4">12.5 <small class="text-lg font-medium italic">Ton</small></p>
               <p class="text-xs text-white/60 font-medium leading-relaxed italic">Estimasi total output daging saat panen batch ini berdasarkan kurva pertumbuhan saat ini.</p>
            </div>
          </div>
        </div>

        <div class="p-10 bg-gray-50 border-t border-gray-100 flex justify-between items-center">
          <p class="text-xs text-gray-400 font-medium italic max-w-sm">Data riwayat ini di-generate otomatis oleh HELP-IN AI Advisor berdasarkan flowchart sistem pakar.</p>
          <button @click="isAuditModalOpen = false" class="px-10 py-5 bg-[#1a402d] text-white rounded-3xl font-black shadow-xl shadow-green-900/40 hover:scale-[1.05] active:scale-95 transition-all">Selesai Meninjau Audit</button>
        </div>
      </div>
    </div>

  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { 
  LayoutDashboardIcon, HomeIcon, DogIcon, BarChart3Icon, 
  ActivityIcon, ClipboardListIcon, SoupIcon, LogOutIcon, 
  EyeIcon, XIcon, MenuIcon, DownloadIcon, SearchIcon,
  TrendingUpIcon, ThermometerIcon, DropletsIcon, ArchiveIcon
} from 'lucide-vue-next'
import { Line, Doughnut, Bar } from 'vue-chartjs'
import { 
  Chart as ChartJS, Title, Tooltip, Legend, BarElement, 
  CategoryScale, LinearScale, PointElement, LineElement, ArcElement, Filler 
} from 'chart.js'

ChartJS.register(Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale, PointElement, LineElement, ArcElement, Filler)

const isSidebarOpen = ref(false)
const isAuditModalOpen = ref(false)
const selectedAudit = ref(null)
const searchQuery = ref('')

// MENU LIST
const menus = [
  { name: 'Kandang', icon: HomeIcon },
  { name: 'Ternak', icon: DogIcon },
  { name: 'Kualitas Ternak', icon: BarChart3Icon },
  { name: 'Kondisi Kesehatan', icon: ActivityIcon },
  { name: 'Riwayat Kesehatan', icon: ClipboardListIcon },
  { name: 'Rekomendasi Pakan', icon: SoupIcon },
]

// ==========================================
// DATASET INTELLIGENCE (Berdasarkan Flowchart)
// ==========================================
const dummyRekomendasi = [
  { id: '101', jenis: 'SAPI BRAHMAN', jumlah: 20, kesehatan: 'Kurang Baik', informasi: 'Beri Vitamin Pada ternak anda' },
  { id: '102', jenis: 'SAPI LIMOUSIN', jumlah: 15, kesehatan: 'Sangat Baik', informasi: 'Pertahankan komposisi pakan saat ini' },
  { id: '103', jenis: 'KAMBING ETAWA', jumlah: 45, kesehatan: 'Kurang Baik', informasi: 'Cek kelembapan kandang, terdeteksi tinggi' },
  { id: '104', jenis: 'SAPI BRAHMAN', jumlah: 20, kesehatan: 'Kurang Baik', informasi: 'Beri Vitamin Pada ternak anda' },
  { id: '105', jenis: 'SAPI BRAHMAN', jumlah: 20, kesehatan: 'Kurang Baik', informasi: 'Beri Vitamin Pada ternak anda' }
];

const sensorMetrics = [
  { label: 'Suhu Kandang', value: '28.5', unit: '°C', status: 'Normal', progress: 75, icon: ThermometerIcon, alert: false },
  { label: 'Kelembapan (H)', value: '85', unit: '%', status: 'Waspada', progress: 85, icon: DropletsIcon, alert: true },
  { label: 'Kadar Amonia', value: '12', unit: 'ppm', status: 'Aman', progress: 30, icon: ActivityIcon, alert: false },
  { label: 'Sisa Pakan', value: '150', unit: 'kg', status: 'Cukup', progress: 60, icon: ArchiveIcon, alert: false },
]

const feedAnalysis = [
  { name: 'Protein Kasar', target: 18, actual: 16.5 },
  { name: 'Serat Kasar', target: 12, actual: 12.2 },
  { name: 'Lemak', target: 5, actual: 4.8 },
  { name: 'Kalsium (Ca)', target: 2.5, actual: 1.1 }
]

const biometricData = [
  { label: 'Avg Heart Rate', value: '72', unit: 'bpm' },
  { label: 'Body Temp', value: '38.2', unit: '°C' },
  { label: 'Daily Movement', value: '2.4', unit: 'km' },
  { label: 'Feed Intake', value: '14.5', unit: 'kg/day' }
]

// ==========================================
// CHART CONFIGURATIONS
// ==========================================
const intelligenceTrendData = computed(() => ({
  labels: ['Batch 1', 'Batch 2', 'Batch 3', 'Batch 4', 'Batch 5', 'Batch 6'],
  datasets: [
    { label: 'Prediksi Sistem', data: [10, 15, 12, 18, 20, 25], borderColor: '#cbd5e1', borderDash: [5, 5], tension: 0.4, fill: false },
    { label: 'Realita Lapangan', data: [11, 14, 13, 21, 22, 28], borderColor: '#10b981', backgroundColor: 'rgba(16, 185, 129, 0.1)', fill: true, tension: 0.4 }
  ]
}))

const healthDistributionData = {
  labels: ['Sehat', 'Observasi', 'Sakit'],
  datasets: [{ data: [94, 4, 2], backgroundColor: ['#10b981', '#fbbf24', '#ef4444'], borderWidth: 0, hoverOffset: 10 }]
}

const chartOptions = { responsive: true, maintainAspectRatio: false, plugins: { legend: { position: 'bottom', labels: { usePointStyle: true, font: { weight: 'bold', size: 10 } } } }, scales: { y: { display: false }, x: { grid: { display: false } } } }
const doughnutOptions = { responsive: true, maintainAspectRatio: false, cutout: '85%', plugins: { legend: { display: false } } }

// ==========================================
// LOGIC METHODS
// ==========================================
const filteredTableData = computed(() => {
  return dummyRekomendasi.filter(i => i.jenis.toLowerCase().includes(searchQuery.value.toLowerCase()) || i.id.includes(searchQuery.value))
})

const openAuditDetail = (item) => {
  selectedAudit.value = item;
  isAuditModalOpen.value = true;
}

const exportReport = () => alert('Menyiapkan Laporan Audit Intelijen Peternakan (PDF)...');
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

/* Modal Scrollbar Custom */
.overflow-y-auto::-webkit-scrollbar { width: 4px; }
.overflow-y-auto::-webkit-scrollbar-track { background: transparent; }
.overflow-y-auto::-webkit-scrollbar-thumb { background: #e2e8f0; border-radius: 10px; }
</style>