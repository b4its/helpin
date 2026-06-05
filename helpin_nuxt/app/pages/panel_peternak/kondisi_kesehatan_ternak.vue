<template>
  <div class="flex h-screen w-full bg-[#f4f7f5] font-sans overflow-hidden relative text-slate-900">
    
  <SidebarPeternak :isOpen="isSidebarOpen" @close="isSidebarOpen = false" />

    <main class="flex-1 flex flex-col overflow-y-auto relative w-full no-scrollbar">
      <header class="flex justify-between items-center px-6 md:px-10 py-6 border-b border-gray-200 bg-white/80 backdrop-blur-md z-10 sticky top-0">
        <div class="flex items-center gap-4">
          <button @click="isSidebarOpen = true" class="md:hidden p-2 text-gray-600 border rounded-lg"><MenuIcon class="w-6 h-6" /></button>
          <div>
            <h1 class="text-xl md:text-2xl font-black text-gray-800 tracking-tight">Kondisi Kesehatan Ternak</h1>
            <p class="text-xs md:text-sm text-gray-500 font-medium italic">Monitoring bio-metrik dan audit status medis ternak</p>
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
            <div class="flex justify-between items-start mb-6">
              <div>
                <h3 class="text-lg font-black text-gray-800">Population Vitality Index</h3>
                <p class="text-xs text-gray-400 font-bold uppercase tracking-widest">Trend Kesehatan 7 Hari Terakhir</p>
              </div>
              <ActivityIcon class="w-5 h-5 text-blue-500" />
            </div>
            <div class="h-64 w-full">
              <Line :data="healthTrendData" :options="chartOptions" />
            </div>
          </div>

          <div class="bg-white p-8 rounded-[32px] border border-gray-100 shadow-sm flex flex-col items-center">
            <h3 class="text-lg font-black text-gray-800 mb-6">Status Populasi</h3>
            <div class="relative h-48 w-48">
              <Doughnut :data="statusDistributionData" :options="doughnutOptions" />
              <div class="absolute inset-0 flex flex-col items-center justify-center pointer-events-none">
                <span class="text-3xl font-black text-gray-800">92%</span>
                <span class="text-[10px] text-green-600 font-black uppercase">Safe Zone</span>
              </div>
            </div>
            <div class="mt-8 w-full space-y-2">
               <div v-for="(val, label) in statusLabels" :key="label" class="flex justify-between items-center text-[11px] font-bold uppercase">
                  <div class="flex items-center gap-2">
                    <div class="w-2 h-2 rounded-full" :style="`background-color: ${val.color}`"></div>
                    <span class="text-gray-500">{{ label }}</span>
                  </div>
                  <span>{{ val.count }} Ekor</span>
               </div>
            </div>
          </div>
        </section>

        <section class="grid grid-cols-1 md:grid-cols-3 gap-6">
          <div class="md:col-span-2 bg-white p-8 rounded-[32px] border border-gray-100 shadow-sm">
             <h3 class="text-sm font-black text-gray-400 uppercase tracking-widest mb-6">Diagnosis Kasus Per Jenis Ternak</h3>
             <div class="h-56 w-full"><Bar :data="caseComparisonData" :options="barOptions" /></div>
          </div>
          <div class="space-y-4">
             <div v-for="m in medicalQuickStats" :key="m.label" class="bg-white p-6 rounded-[32px] border border-gray-100 shadow-sm flex flex-col justify-center relative overflow-hidden group hover:border-blue-200 transition-all">
                <div class="absolute -right-4 -bottom-4 opacity-5 group-hover:opacity-10 transition-opacity">
                  <component :is="m.icon" class="w-24 h-24" />
                </div>
                <p class="text-[10px] font-black text-gray-400 uppercase tracking-widest mb-1">{{ m.label }}</p>
                <h4 class="text-3xl font-black text-[#1a402d]">{{ m.value }}<small class="text-xs ml-1 font-bold text-gray-400">{{ m.unit }}</small></h4>
             </div>
          </div>
        </section>

        <section class="bg-white rounded-[40px] shadow-sm border border-gray-100 overflow-hidden mb-10">
          <div class="p-8 border-b border-gray-100 flex justify-between items-center bg-gray-50/30">
            <div class="flex items-center gap-3">
              <div class="w-2 h-8 bg-blue-600 rounded-full"></div>
              <h2 class="text-xl font-black text-gray-800 tracking-tight">Individu Health Audit Records</h2>
            </div>
            <div class="relative">
              <SearchIcon class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-gray-400" />
              <input v-model="searchQuery" type="text" placeholder="Search Tag ID..." class="pl-10 pr-4 py-2 bg-white border border-gray-100 rounded-xl text-sm font-bold w-64 focus:ring-4 focus:ring-blue-500/10 outline-none transition" />
            </div>
          </div>
          
          <div class="overflow-x-auto no-scrollbar">
            <table class="w-full text-left border-collapse min-w-[1000px]">
              <thead>
                <tr class="bg-gray-50 text-[11px] font-black text-gray-400 uppercase tracking-[0.15em] border-b border-gray-100">
                  <th class="px-8 py-5">Livestock Source</th>
                  <th class="px-8 py-5">Vitals Status</th>
                  <th class="px-8 py-5 text-center">Vitality Score</th>
                  <th class="px-8 py-5">Diagnosis Intelligence</th>
                  <th class="px-8 py-5 text-center">View</th>
                </tr>
              </thead>
              <tbody class="divide-y divide-gray-50">
                <tr v-for="log in filteredHealthData" :key="log.id" class="hover:bg-blue-50/30 transition-all group">
                  <td class="px-8 py-6">
                    <div class="flex flex-col">
                      <span class="font-black text-gray-800 text-base uppercase">{{ log.tagId }}</span>
                      <span class="text-[10px] text-gray-400 font-bold uppercase tracking-tighter">Last Checked: {{ formatDate(log.lastCheck) }}</span>
                    </div>
                  </td>
                  <td class="px-8 py-6">
                    <div class="flex items-center gap-4">
                      <div class="flex flex-col">
                        <span class="text-[9px] font-black text-gray-400 uppercase">Detak Jantung</span>
                        <span class="font-black text-slate-700">{{ log.heartRate }} <small class="text-[9px]">BPM</small></span>
                      </div>
                      <div class="flex flex-col">
                        <span class="text-[9px] font-black text-gray-400 uppercase">Suhu Tubuh</span>
                        <span :class="['font-black', log.temp > 39 ? 'text-red-500' : 'text-blue-600']">{{ log.temp }}°C</span>
                      </div>
                    </div>
                  </td>
                  <td class="px-8 py-6 text-center">
                    <div class="flex flex-col items-center gap-1">
                       <span class="text-xl font-black text-[#1a402d]">{{ log.vitalityScore }}%</span>
                       <div class="h-1 w-20 bg-gray-100 rounded-full overflow-hidden">
                          <div :class="['h-full', log.vitalityScore < 60 ? 'bg-red-500' : 'bg-green-500']" :style="`width: ${log.vitalityScore}%`"></div>
                       </div>
                    </div>
                  </td>
                  <td class="px-8 py-6">
                    <span :class="['px-3 py-1.5 rounded-full text-[10px] font-black uppercase tracking-widest', 
                      log.status === 'Sehat' ? 'bg-green-50 text-green-600' : 
                      log.status === 'Waspada' ? 'bg-yellow-50 text-yellow-600' : 'bg-red-50 text-red-600']">
                      {{ log.status }}
                    </span>
                  </td>
                  <td class="px-8 py-6 text-center">
                    <button @click="openMedicalModal(log)" class="p-3 bg-gray-50 text-gray-400 rounded-xl hover:bg-blue-600 hover:text-white transition-all shadow-sm">
                      <ActivityIcon class="w-5 h-5" />
                    </button>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </section>
      </div>
    </main>

    <div v-if="isModalOpen" class="fixed inset-0 z-50 flex items-center justify-center p-4">
      <div class="absolute inset-0 bg-[#0c1a13]/80 backdrop-blur-xl animate-in fade-in" @click="isModalOpen = false"></div>
      
      <div class="bg-white rounded-[50px] w-full max-w-4xl max-h-[92vh] overflow-hidden shadow-2xl relative z-10 flex flex-col animate-in slide-in-from-bottom-10 duration-500 no-scrollbar">
        
        <div class="bg-gradient-to-br from-[#1a402d] to-[#1e3a8a] p-12 text-white relative">
          <button @click="isModalOpen = false" class="absolute top-10 right-10 p-3 bg-white/10 hover:bg-white/20 rounded-full transition-all"><XIcon class="w-6 h-6" /></button>
          <div class="flex items-center gap-3 mb-4">
            <span class="px-3 py-1 bg-blue-400 text-white text-[10px] font-black rounded-full uppercase tracking-widest">Medical Passport</span>
            <span class="text-white/40 font-mono text-sm uppercase italic">Audit_Log: {{ selectedLog?.id }}</span>
          </div>
          <h2 class="text-5xl font-black tracking-tighter mb-2 uppercase">{{ selectedLog?.tagId }}</h2>
          <p class="text-lg text-white/60 font-medium italic">Rekam Medis Digital Berbasis AI Advisor HELP-IN</p>
        </div>

        <div class="p-12 overflow-y-auto no-scrollbar grid grid-cols-1 md:grid-cols-3 gap-12 bg-white">
          <div class="space-y-6">
            <h3 class="text-xs font-black text-blue-700 uppercase tracking-[0.2em] flex items-center gap-2">
              <ThermometerIcon class="w-4 h-4" /> Live Biometrics
            </h3>
            <div class="space-y-4">
               <div v-for="bio in modalBiometrics" :key="bio.label" class="bg-gray-50 p-6 rounded-[32px] border border-gray-100 shadow-inner">
                  <p class="text-[10px] font-black text-gray-400 uppercase mb-2">{{ bio.label }}</p>
                  <div class="flex justify-between items-baseline">
                    <span class="text-3xl font-black text-slate-800">{{ bio.value }}</span>
                    <span class="text-xs font-bold text-blue-600">{{ bio.unit }}</span>
                  </div>
               </div>
            </div>
          </div>

          <div class="md:col-span-2 space-y-8">
            <div class="bg-blue-50/50 p-8 rounded-[40px] border border-blue-100 relative overflow-hidden">
               <div class="flex justify-between items-start mb-6">
                 <div>
                    <p class="text-[10px] font-black text-blue-500 uppercase mb-1 tracking-widest">Current Intelligence Score</p>
                    <h4 class="text-4xl font-black text-[#1a402d]">{{ selectedLog?.vitalityScore }}% Vitality</h4>
                 </div>
                 <div class="h-12 w-12 bg-white rounded-2xl flex items-center justify-center shadow-sm text-blue-600">
                    <ActivityIcon class="w-6 h-6" />
                 </div>
               </div>
               
               <p class="text-[10px] font-black text-gray-400 uppercase mb-4 tracking-widest">Diagnosis Pakar</p>
               <div class="bg-white p-6 rounded-3xl text-sm font-bold text-slate-600 leading-relaxed italic border border-gray-100 shadow-sm mb-8">
                 "{{ selectedLog?.diagnosis }}"
               </div>

               <p class="text-[10px] font-black text-gray-400 uppercase mb-4 tracking-widest">Medical Treatment History</p>
               <div class="grid grid-cols-2 gap-4">
                  <div v-for="med in selectedLog?.treatments" :key="med.name" class="bg-white p-4 rounded-2xl border border-gray-50 flex justify-between items-center">
                    <div class="flex flex-col">
                      <span class="text-xs font-black text-slate-800">{{ med.name }}</span>
                      <span class="text-[9px] text-gray-400 font-bold uppercase">{{ med.date }}</span>
                    </div>
                    <CheckCircleIcon class="w-4 h-4 text-green-500" />
                  </div>
               </div>
            </div>
          </div>
        </div>

        <div class="p-10 bg-gray-50 border-t border-gray-100 flex justify-between items-center">
          <p class="text-[10px] text-gray-400 font-black uppercase tracking-widest">Generated by HELP-IN AI Medical Advisor © 2026</p>
          <button @click="isModalOpen = false" class="px-12 py-5 bg-[#1a402d] text-white rounded-3xl font-black shadow-xl hover:scale-105 active:scale-95 transition-all uppercase text-xs tracking-widest">Tutup Catatan Medis</button>
        </div>
      </div>
    </div>

  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { 
  LayoutDashboardIcon, HomeIcon, DogIcon, ActivityIcon, ClipboardListIcon, BarChart3Icon,SoupIcon,
  LogOutIcon, MenuIcon, XIcon, ShieldCheckIcon, SearchIcon, EyeIcon, 
  TrendingUpIcon, ThermometerIcon, CheckCircleIcon, ZapIcon, InfoIcon
} from 'lucide-vue-next'

// CHART INTEGRATION
import { Line, Doughnut, Bar } from 'vue-chartjs'
import { 
  Chart as ChartJS, Title, Tooltip, Legend, BarElement, 
  CategoryScale, LinearScale, PointElement, LineElement, ArcElement, Filler 
} from 'chart.js'

ChartJS.register(Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale, PointElement, LineElement, ArcElement, Filler)

const isSidebarOpen = ref(false)
const isModalOpen = ref(false)
const selectedLog = ref(null)
const searchQuery = ref('')

// ==========================================
// 1. DYNAMIC SHARED NAVIGATION
// ==========================================
const activeMenu = ref('Kondisi Kesehatan Ternak')
const menus = [
  { name: 'Dashboard', icon: LayoutDashboardIcon, path: '/panel_peternak/dashboard_peternak' },
  { name: 'Kandang', icon: HomeIcon, path: '/panel_peternak/kandang_peternak' },
  { name: 'Ternak', icon: DogIcon, path: '/panel_peternak/ternak' },
  { name: 'Kualitas Ternak', icon: BarChart3Icon, path: '/panel_peternak/kualitas_ternak' },
  { name: 'Kondisi Kesehatan Ternak', icon: ActivityIcon, path: '/panel_peternak/kondisi_kesehatan_ternak'  },
  { name: 'Rekomendasi Pakan', icon: SoupIcon, path: '/panel_peternak/rekomendasi_pakan_ternak' },
]

// ==========================================
// 2. DATA MODEL (CLASS / DTO)
// ==========================================
class HealthLogModel {
  constructor(data) {
    this.id = data.id || `MED-${Math.floor(Math.random() * 9000) + 1000}`;
    this.tagId = data.tagId;
    this.status = data.status || 'Sehat';
    this.vitalityScore = data.vitalityScore || 90;
    this.heartRate = data.heartRate || 72;
    this.temp = data.temp || 38.5;
    this.lastCheck = data.lastCheck || new Date().toISOString();
    this.diagnosis = data.diagnosis || 'Kondisi ternak terpantau optimal, metabolisme stabil.';
    this.treatments = data.treatments || [
      { name: 'Vaksin SE', date: '12 Jan 2026' },
      { name: 'Vitamin B-Complex', date: '05 Feb 2026' }
    ];
  }

  static fromJSON(json) {
    return new HealthLogModel({
      id: json.rekam_medis_id,
      tagId: json.id_ternak,
      status: json.status_kesehatan,
      vitalityScore: json.skor_vitalitas,
      heartRate: json.bpm_sensor,
      temp: json.body_temp,
      lastCheck: json.tgl_periksa,
      diagnosis: json.hasil_diagnosis,
      treatments: json.riwayat_tindakan
    });
  }
}

// ==========================================
// 3. REACTIVE STATE & DATA
// ==========================================
const healthDataList = ref([])

onMounted(() => {
  const dummyJSON = [
    { rekam_medis_id: 'MED-9901', id_ternak: 'BRH-001', status_kesehatan: 'Sehat', skor_vitalitas: 98, bpm_sensor: 70, body_temp: 38.4, tgl_periksa: '2026-05-01', hasil_diagnosis: 'Ternak dalam kondisi prima.', riwayat_tindakan: [{name:'Vitamin A', date:'01 Mei'}, {name:'Vaksin Antrax', date:'10 Apr'}] },
    { rekam_medis_id: 'MED-9902', id_ternak: 'LMS-002', status_kesehatan: 'Waspada', skor_vitalitas: 65, bpm_sensor: 85, body_temp: 39.2, tgl_periksa: '2026-05-12', hasil_diagnosis: 'Terdeteksi suhu tubuh di atas normal, indikasi stres panas.', riwayat_tindakan: [{name:'Elektrolit', date:'12 Mei'}] },
    { rekam_medis_id: 'MED-9903', id_ternak: 'ETW-045', status_kesehatan: 'Sakit', skor_vitalitas: 42, bpm_sensor: 62, body_temp: 37.8, tgl_periksa: '2026-05-14', hasil_diagnosis: 'Nafsu makan menurun, perlu karantina segera.', riwayat_tindakan: [{name:'Antibiotik', date:'14 Mei'}] },
    { rekam_medis_id: 'MED-9904', id_ternak: 'BRH-012', status_kesehatan: 'Sehat', skor_vitalitas: 92, bpm_sensor: 72, body_temp: 38.5, tgl_periksa: '2026-05-15', hasil_diagnosis: 'Pemulihan pasca melahirkan sangat baik.', riwayat_tindakan: [{name:'Kalsium Cair', date:'15 Mei'}] },
  ];
  healthDataList.value = dummyJSON.map(d => HealthLogModel.fromJSON(d));
});

// ==========================================
// 4. CHART DATA COMPUTATIONS
// ==========================================
const healthTrendData = computed(() => ({
  labels: ['Sen', 'Sel', 'Rab', 'Kam', 'Jum', 'Sab', 'Min'],
  datasets: [{
    label: 'Avg Vitality Index',
    data: [90, 92, 88, 85, 94, 95, 92],
    borderColor: '#2563eb',
    backgroundColor: 'rgba(37, 99, 235, 0.05)',
    fill: true,
    tension: 0.4,
    pointRadius: 5,
    pointBackgroundColor: '#fff',
    pointBorderWidth: 2
  }]
}))

const statusDistributionData = computed(() => {
  const s = { Sehat: 0, Sakit: 0, Waspada: 0 };
  healthDataList.value.forEach(i => s[i.status]++);
  return {
    labels: ['Sehat', 'Sakit', 'Observasi'],
    datasets: [{
      data: [s.Sehat, s.Sakit, s.Waspada],
      backgroundColor: ['#10b981', '#ef4444', '#fbbf24'],
      borderWidth: 0,
      hoverOffset: 20
    }]
  }
})

const caseComparisonData = computed(() => ({
  labels: ['Brahman', 'Limousin', 'Etawa', 'Angus'],
  datasets: [{
    label: 'Kasus Medis',
    data: [12, 19, 3, 8],
    backgroundColor: '#1a402d',
    borderRadius: 12
  }]
}))

// ==========================================
// 5. HELPERS & METHODS
// ==========================================
const statusLabels = {
  'Sehat': { color: '#10b981', count: 84 },
  'Waspada': { color: '#fbbf24', count: 12 },
  'Sakit': { color: '#ef4444', count: 4 }
}

const medicalQuickStats = [
  { label: 'Avg Body Temp', value: '38.5', unit: '°C', icon: ThermometerIcon },
  { label: 'Vitality Index', value: '92.4', unit: '%', icon: ActivityIcon },
  { label: 'Pending Vaccines', value: '18', unit: 'Doses', icon: ZapIcon },
]

const modalBiometrics = computed(() => {
  if (!selectedLog.value) return [];
  return [
    { label: 'Body Temperature', value: selectedLog.value.temp, unit: 'Celsius' },
    { label: 'Heart Rate (Pulse)', value: selectedLog.value.heartRate, unit: 'BPM' },
    { label: 'Respiratory Rate', value: '18', unit: 'BPM' }
  ]
})

const filteredHealthData = computed(() => {
  return healthDataList.value.filter(i => i.tagId.toLowerCase().includes(searchQuery.value.toLowerCase()))
})

const openMedicalModal = (log) => { selectedLog.value = log; isModalOpen.value = true; }
const exportMedicalReport = () => alert('Downloading Health Audit Report PDF...');
const formatDate = (d) => new Date(d).toLocaleDateString('id-ID', { day: '2-digit', month: 'short' });

const chartOptions = { responsive: true, maintainAspectRatio: false, plugins: { legend: { display: false } }, scales: { y: { min: 80, max: 100, display: false }, x: { grid: { display: false } } } }
const barOptions = { responsive: true, maintainAspectRatio: false, plugins: { legend: { display: false } }, scales: { y: { display: false }, x: { grid: { display: false } } } }
const doughnutOptions = { responsive: true, maintainAspectRatio: false, cutout: '82%', plugins: { legend: { display: false } } }
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

.animate-in { animation: fadeIn 0.4s ease-out forwards; }
@keyframes fadeIn {
  from { opacity: 0; transform: translateY(20px); }
  to { opacity: 1; transform: translateY(0); }
}

.overflow-y-auto::-webkit-scrollbar { width: 4px; }
.overflow-y-auto::-webkit-scrollbar-thumb { background: #e2e8f0; border-radius: 10px; }
</style>