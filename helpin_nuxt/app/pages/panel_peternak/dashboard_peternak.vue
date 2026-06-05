<template>
  <div class="flex h-screen w-full bg-[#f4f7f5] font-sans overflow-hidden relative text-slate-900">
    
  <SidebarPeternak :isOpen="isSidebarOpen" @close="isSidebarOpen = false" />

    <main class="flex-1 flex flex-col overflow-y-auto relative w-full no-scrollbar">
      <header class="flex justify-between items-center px-6 md:px-10 py-6 border-b border-gray-200 bg-white shadow-sm z-10 sticky top-0">
        <div class="flex items-center gap-4">
          <button @click="isSidebarOpen = true" class="md:hidden p-2 text-gray-600 hover:bg-gray-100 rounded-lg transition"><MenuIcon class="w-6 h-6" /></button>
          <div>
            <h1 class="text-xl md:text-2xl font-black text-gray-800 leading-tight">Dashboard</h1>
            <p class="text-xs md:text-sm text-gray-500 font-medium mt-0.5">Analisis pakar dan pemantauan sensor real-time</p>
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
          <div class="lg:col-span-2 bg-white p-8 rounded-[32px] border border-gray-100 shadow-sm">
            <h3 class="text-lg font-black text-gray-800 mb-6 uppercase tracking-widest">Yield Evolution</h3>
            <div class="h-64 w-full"><Line :data="intelligenceTrendData" :options="chartOptions" /></div>
          </div>
          <div class="bg-white p-8 rounded-[32px] border border-gray-100 shadow-sm flex flex-col items-center justify-center">
            <h3 class="text-lg font-black text-gray-800 mb-6">Status Kesehatan</h3>
            <div class="relative h-48 w-48 flex items-center justify-center">
              <Doughnut :data="healthDistributionData" :options="doughnutOptions" />
              <div class="absolute inset-0 flex flex-col items-center justify-center">
                <span class="text-4xl font-black text-gray-800">94%</span>
                <span class="text-[10px] text-green-600 font-black uppercase">Healthy</span>
              </div>
            </div>
          </div>
        </section>

        <section class="grid grid-cols-1 md:grid-cols-4 gap-4">
           <div v-for="sensor in sensorMetrics" :key="sensor.label" class="bg-white p-6 rounded-3xl border border-gray-100 shadow-sm">
             <div class="flex items-center gap-4">
                <div :class="['p-3 rounded-2xl', sensor.alert ? 'bg-red-50 text-red-500' : 'bg-green-50 text-green-600']">
                  <component :is="sensor.icon" class="w-6 h-6" />
                </div>
                <div>
                  <p class="text-[10px] font-black text-gray-400 uppercase tracking-widest">{{ sensor.label }}</p>
                  <p class="text-xl font-black text-gray-800">{{ sensor.value }}<span class="text-xs font-bold text-gray-400 ml-1">{{ sensor.unit }}</span></p>
                </div>
             </div>
           </div>
        </section>

        <section class="bg-white rounded-[40px] shadow-sm border border-gray-100 overflow-hidden mb-10">
          <div class="p-8 border-b border-gray-100 flex flex-col md:flex-row md:items-center justify-between gap-4">
            <h2 class="text-xl font-black text-gray-800 tracking-tight">Intelligence Recommendation Log</h2>
            <input v-model="searchQuery" type="text" placeholder="Search..." class="px-4 py-2 bg-gray-50 border-none rounded-xl text-sm font-bold w-full md:w-64" />
          </div>
          <div class="overflow-x-auto no-scrollbar">
            <table class="w-full text-left border-collapse">
              <thead>
                <tr class="bg-gray-50/50 text-[11px] font-black text-gray-400 uppercase tracking-[0.15em] border-b border-gray-100">
                  <th class="px-8 py-5">Audit ID & Jenis</th>
                  <th class="px-8 py-5 text-center">Vitality</th>
                  <th class="px-8 py-5">Sistem Diagnosis</th>
                  <th class="px-8 py-5 text-center">Audit</th>
                </tr>
              </thead>
              <tbody class="divide-y divide-gray-50">
                <tr v-for="item in filteredTableData" :key="item.id" class="hover:bg-green-50/20 transition-all">
                  <td class="px-8 py-6">
                    <div class="flex flex-col font-black text-gray-800 uppercase tracking-tighter">
                      {{ item.jenis }} <small class="text-[10px] text-gray-400 font-bold">LOG-{{ item.id }}</small>
                    </div>
                  </td>
                  <td class="px-8 py-6 text-center">
                    <span :class="['px-3 py-1 rounded-full text-[10px] font-black uppercase', item.kesehatan === 'Kurang Baik' ? 'bg-red-50 text-red-600' : 'bg-green-50 text-green-600']">
                      {{ item.kesehatan }}
                    </span>
                  </td>
                  <td class="px-8 py-6 font-bold text-red-500 text-sm italic">{{ item.informasi }}</td>
                  <td class="px-8 py-6 text-center">
                    <button @click="viewFullAudit(item)" class="p-3 bg-gray-50 text-gray-400 rounded-xl hover:bg-[#1a402d] hover:text-white transition-all">
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
      <div class="bg-white rounded-[50px] w-full max-w-4xl max-h-[92vh] overflow-hidden shadow-2xl relative z-10 flex flex-col animate-in slide-in-from-bottom duration-500">
        <div class="bg-gradient-to-br from-[#1a402d] to-[#2d5c41] p-12 text-white relative">
          <button @click="isAuditModalOpen = false" class="absolute top-10 right-10 p-3 bg-white/10 hover:bg-white/20 rounded-full transition-all"><XIcon class="w-6 h-6" /></button>
          <div class="flex items-center gap-3 mb-4">
            <span class="px-3 py-1 bg-green-400 text-[#1a402d] text-[10px] font-black rounded-full uppercase tracking-widest tracking-widest">Expert Audit Verified</span>
          </div>
          <h2 class="text-5xl font-black tracking-tighter mb-2 italic">Breeding Intel Passport</h2>
          <p class="text-lg text-white/60 font-medium italic">REF_ID: #LOG-{{ selectedAudit?.id }} • {{ selectedAudit?.jenis }}</p>
        </div>
        <div class="p-12 grid grid-cols-1 md:grid-cols-2 gap-12 bg-white overflow-y-auto no-scrollbar">
          <div class="space-y-6">
            <h3 class="text-xs font-black text-green-700 uppercase tracking-[0.2em] flex items-center gap-2"><SoupIcon class="w-4 h-4" /> Nutrient Input</h3>
            <div class="bg-gray-50 rounded-[32px] p-8 border border-gray-100 space-y-4">
               <div v-for="feed in feedAnalysis" :key="feed.name" class="flex justify-between items-center border-b border-gray-200 pb-3 last:border-0">
                 <span class="text-sm font-black text-gray-600">{{ feed.name }}</span>
                 <span class="text-lg font-black text-green-800">{{ feed.actual }}%</span>
               </div>
            </div>
          </div>
          <div class="space-y-6">
            <h3 class="text-xs font-black text-blue-700 uppercase tracking-[0.2em] flex items-center gap-2"><ActivityIcon class="w-4 h-4" /> Vitals</h3>
            <div class="grid grid-cols-2 gap-4">
              <div v-for="bio in biometricData" :key="bio.label" class="bg-blue-50/30 p-5 rounded-3xl border border-blue-100 flex flex-col items-center">
                <p class="text-[10px] font-black text-blue-400 uppercase mb-1">{{ bio.label }}</p>
                <p class="text-xl font-black text-blue-900">{{ bio.value }}<small class="text-[10px] ml-0.5">{{ bio.unit }}</small></p>
              </div>
            </div>
            <div class="bg-[#1a402d] rounded-[32px] p-8 text-white">
               <p class="text-[10px] font-black text-green-400 uppercase mb-2 tracking-widest">Prediction Weight</p>
               <p class="text-5xl font-black">12.5 <span class="text-lg font-medium italic text-green-400">Ton</span></p>
            </div>
          </div>
        </div>
        <div class="p-8 bg-gray-50 border-t border-gray-100 text-center">
          <button @click="isAuditModalOpen = false" class="px-12 py-4 bg-[#1a402d] text-white rounded-3xl font-black shadow-xl hover:scale-105 transition-all">Selesai Meninjau</button>
        </div>
      </div>
    </div>

  </div>
</template>

<script setup>
import { ref, computed } from 'vue'
import { 
  LayoutDashboardIcon, HomeIcon, DogIcon, BarChart3Icon, 
  ActivityIcon, ClipboardListIcon, SoupIcon, LogOutIcon, 
  EyeIcon, XIcon, MenuIcon, DownloadIcon, SearchIcon,
  TrendingUpIcon, ThermometerIcon, DropletsIcon, ArchiveIcon
} from 'lucide-vue-next'
import { Line, Doughnut } from 'vue-chartjs'
import { Chart as ChartJS, Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale, PointElement, LineElement, ArcElement, Filler } from 'chart.js'

ChartJS.register(Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale, PointElement, LineElement, ArcElement, Filler)

const isSidebarOpen = ref(false)
const isAuditModalOpen = ref(false)
const selectedAudit = ref(null)
const searchQuery = ref('')

// ==========================================
// SHARED NAVIGATION LOGIC
// ==========================================
const activeMenu = ref('Dashboard')
const menus = [
  { name: 'Dashboard', icon: LayoutDashboardIcon, path: '/panel_peternak/dashboard_peternak' },
  { name: 'Kandang', icon: HomeIcon, path: '/panel_peternak/kandang_peternak' },
  { name: 'Ternak', icon: DogIcon, path: '/panel_peternak/ternak' },
  { name: 'Kualitas Ternak', icon: BarChart3Icon, path: '/panel_peternak/kualitas_ternak' },
  { name: 'Kondisi Kesehatan Ternak', icon: ActivityIcon, path: '/panel_peternak/kondisi_kesehatan_ternak'  },
  { name: 'Rekomendasi Pakan', icon: SoupIcon, path: '/panel_peternak/rekomendasi_pakan_ternak' },
]

// DUMMY DATA FOR DASHBOARD
const dummyRekomendasi = [
  { id: '101', jenis: 'SAPI BRAHMAN', jumlah: 20, kesehatan: 'Kurang Baik', informasi: 'Beri Vitamin Pada ternak anda' },
  { id: '102', jenis: 'SAPI LIMOUSIN', jumlah: 15, kesehatan: 'Sangat Baik', informasi: 'Komposisi pakan stabil' },
  { id: '103', jenis: 'KAMBING ETAWA', jumlah: 45, kesehatan: 'Kurang Baik', informasi: 'Cek suhu kandang area B' },
]

const sensorMetrics = [
  { label: 'Suhu Kandang', value: '28.5', unit: '°C', icon: ThermometerIcon, alert: false },
  { label: 'Kelembapan', value: '85', unit: '%', icon: DropletsIcon, alert: true },
  { label: 'Amonia', value: '12', unit: 'ppm', icon: ActivityIcon, alert: false },
  { label: 'Sisa Pakan', value: '150', unit: 'kg', icon: ArchiveIcon, alert: false },
]

const feedAnalysis = [{ name: 'Protein', actual: 16.5 }, { name: 'Serat', actual: 12.2 }, { name: 'Lemak', actual: 4.8 }]
const biometricData = [{ label: 'Heart Rate', value: '72', unit: 'bpm' }, { label: 'Temp', value: '38.2', unit: '°C' }]

// CHART LOGIC
const intelligenceTrendData = computed(() => ({
  labels: ['M1', 'M2', 'M3', 'M4', 'M5', 'M6'],
  datasets: [
    { label: 'Prediksi', data: [10, 15, 12, 18, 20, 25], borderColor: '#cbd5e1', borderDash: [5, 5], tension: 0.4 },
    { label: 'Realita', data: [11, 14, 13, 21, 22, 28], borderColor: '#10b981', backgroundColor: 'rgba(16, 185, 129, 0.1)', fill: true, tension: 0.4 }
  ]
}))

const healthDistributionData = { labels: ['Sehat', 'Sakit'], datasets: [{ data: [94, 6], backgroundColor: ['#10b981', '#ef4444'], borderWidth: 0 }] }
const chartOptions = { responsive: true, maintainAspectRatio: false, plugins: { legend: { display: false } }, scales: { y: { display: false }, x: { grid: { display: false } } } }
const doughnutOptions = { responsive: true, maintainAspectRatio: false, cutout: '85%', plugins: { legend: { display: false } } }

// METHODS
const filteredTableData = computed(() => dummyRekomendasi.filter(i => i.jenis.toLowerCase().includes(searchQuery.value.toLowerCase())))
const viewFullAudit = (item) => { selectedAudit.value = item; isAuditModalOpen.value = true; }
const exportReport = () => alert('Exporting Report...');
</script>

<style scoped>
.no-scrollbar::-webkit-scrollbar { display: none; }
.no-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }
.active-menu::before { content: ""; position: absolute; right: 0; top: -24px; width: 24px; height: 24px; background-color: transparent; border-bottom-right-radius: 24px; box-shadow: 12px 12px 0 12px #f4f7f5; pointer-events: none; }
.active-menu::after { content: ""; position: absolute; right: 0; bottom: -24px; width: 24px; height: 24px; background-color: transparent; border-top-right-radius: 24px; box-shadow: 12px -12px 0 12px #f4f7f5; pointer-events: none; }
.animate-in { animation: fadeIn 0.5s ease-out forwards; }
@keyframes fadeIn { from { opacity: 0; transform: translateY(20px); } to { opacity: 1; transform: translateY(0); } }
</style>