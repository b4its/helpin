<template>
  <div class="flex h-screen w-full bg-[#f4f7f5] font-sans overflow-hidden relative text-slate-900">
    
  <SidebarPeternak :isOpen="isSidebarOpen" @close="isSidebarOpen = false" />

    <main class="flex-1 flex flex-col overflow-y-auto relative w-full no-scrollbar">
      <header class="flex justify-between items-center px-6 md:px-10 py-6 border-b border-gray-200 bg-white/80 backdrop-blur-md z-10 sticky top-0">
        <div class="flex items-center gap-4">
          <button @click="isSidebarOpen = true" class="md:hidden p-2 text-gray-600 border rounded-lg"><MenuIcon class="w-6 h-6" /></button>
          <div>
            <h1 class="text-2xl font-black text-gray-800 tracking-tight">Kualitas Ternak</h1>
            <p class="text-sm text-gray-500 font-medium italic">Evaluasi standar mutu dan performa genetik ternak</p>
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
                <h3 class="text-lg font-black text-gray-800 tracking-tight">Growth Velocity (ADG)</h3>
                <p class="text-xs text-gray-400 font-bold uppercase tracking-widest">Kecepatan Tumbuh Berat (kg/hari)</p>
              </div>
              <TrendingUpIcon class="w-5 h-5 text-green-600" />
            </div>
            <div class="h-64 w-full">
              <Line :data="adgTrendData" :options="chartOptions" />
            </div>
          </div>

          <div class="bg-white p-8 rounded-[32px] border border-gray-100 shadow-sm flex flex-col items-center">
            <h3 class="text-lg font-black text-gray-800 mb-6">Yield Grade Distribution</h3>
            <div class="relative h-48 w-48">
              <Doughnut :data="gradeDistributionData" :options="doughnutOptions" />
              <div class="absolute inset-0 flex flex-col items-center justify-center">
                <span class="text-3xl font-black text-gray-800">88%</span>
                <span class="text-[10px] text-green-600 font-black uppercase">Standard A</span>
              </div>
            </div>
            <div class="mt-6 flex gap-4">
              <div class="flex items-center gap-1.5"><div class="w-2 h-2 rounded-full bg-green-500"></div><span class="text-[10px] font-bold">Grade A</span></div>
              <div class="flex items-center gap-1.5"><div class="w-2 h-2 rounded-full bg-yellow-400"></div><span class="text-[10px] font-bold">Grade B</span></div>
              <div class="flex items-center gap-1.5"><div class="w-2 h-2 rounded-full bg-red-500"></div><span class="text-[10px] font-bold">Reject</span></div>
            </div>
          </div>
        </section>

        <section class="grid grid-cols-1 md:grid-cols-4 gap-6">
          <div class="md:col-span-2 bg-white p-8 rounded-[32px] border border-gray-100 shadow-sm">
             <h3 class="text-sm font-black text-gray-400 uppercase tracking-widest mb-4">Feed Conversion Ratio (FCR) per Breed</h3>
             <div class="h-48 w-full"><Bar :data="fcrComparisonData" :options="barOptions" /></div>
          </div>
          <div v-for="stat in qualityMetrics" :key="stat.label" class="bg-white p-6 rounded-[32px] border border-gray-100 shadow-sm flex flex-col justify-center">
             <component :is="stat.icon" class="w-6 h-6 text-[#1a402d] mb-3" />
             <p class="text-[10px] font-black text-gray-400 uppercase tracking-widest mb-1">{{ stat.label }}</p>
             <h4 class="text-2xl font-black text-gray-800">{{ stat.value }}<small class="text-xs ml-1">{{ stat.unit }}</small></h4>
          </div>
        </section>

        <section class="bg-white rounded-[40px] shadow-sm border border-gray-100 overflow-hidden mb-10">
          <div class="p-8 border-b border-gray-100 flex justify-between items-center bg-gray-50/30">
            <div class="flex items-center gap-3">
              <div class="w-2 h-8 bg-[#1a402d] rounded-full"></div>
              <h2 class="text-xl font-black text-gray-800 tracking-tight">Livestock Quality Records</h2>
            </div>
            <div class="relative">
              <SearchIcon class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-gray-400" />
              <input v-model="searchQuery" type="text" placeholder="Filter ID Ternak..." class="pl-10 pr-4 py-2 bg-white border border-gray-100 rounded-xl text-sm font-bold w-64 focus:ring-2 focus:ring-[#1a402d]/10 outline-none transition" />
            </div>
          </div>
          
          <div class="overflow-x-auto no-scrollbar">
            <table class="w-full text-left border-collapse min-w-[1000px]">
              <thead>
                <tr class="bg-gray-50 text-[11px] font-black text-gray-400 uppercase tracking-[0.15em] border-b border-gray-100">
                  <th class="px-8 py-5">Ternak Source</th>
                  <th class="px-8 py-5">DNA Meta</th>
                  <th class="px-8 py-5 text-center">FCR Score</th>
                  <th class="px-8 py-5 text-center">Current Grade</th>
                  <th class="px-8 py-5 text-center">Audit</th>
                </tr>
              </thead>
              <tbody class="divide-y divide-gray-50">
                <tr v-for="item in filteredQualityData" :key="item.id" class="hover:bg-green-50/20 transition-all group">
                  <td class="px-8 py-6">
                    <div class="flex flex-col">
                      <span class="font-black text-gray-800 text-base uppercase">{{ item.tagId }}</span>
                      <span class="text-[10px] text-gray-400 font-bold uppercase tracking-widest italic">Source: HELP-IN IOT-Node</span>
                    </div>
                  </td>
                  <td class="px-8 py-6">
                    <div class="flex items-center gap-3">
                      <div class="w-10 h-10 rounded-xl bg-green-50 text-green-700 flex items-center justify-center font-black">{{ item.breed.charAt(0) }}</div>
                      <div class="flex flex-col">
                        <span class="font-bold text-gray-700">{{ item.breed }}</span>
                        <span class="text-[10px] text-gray-400 font-bold uppercase tracking-tighter">Gen: F1 Purebred</span>
                      </div>
                    </div>
                  </td>
                  <td class="px-8 py-6 text-center">
                    <div class="flex flex-col items-center gap-1">
                       <span class="text-xl font-black text-[#1a402d]">{{ item.fcr }}</span>
                       <div class="h-1 w-20 bg-gray-100 rounded-full overflow-hidden">
                          <div class="h-full bg-blue-500" :style="`width: ${Math.min(item.fcr * 10, 100)}%`"></div>
                       </div>
                    </div>
                  </td>
                  <td class="px-8 py-6 text-center">
                    <span :class="['px-4 py-1.5 rounded-full text-[10px] font-black uppercase tracking-[0.2em] shadow-sm', 
                      item.grade === 'A' ? 'bg-green-500 text-white' : 
                      item.grade === 'B' ? 'bg-yellow-400 text-white' : 'bg-red-500 text-white']">
                      Grade {{ item.grade }}
                    </span>
                  </td>
                  <td class="px-8 py-6 text-center">
                    <button @click="openQualityAudit(item)" class="p-3 bg-gray-100 text-gray-500 rounded-xl hover:bg-[#1a402d] hover:text-white transition-all shadow-sm">
                      <BarChart3Icon class="w-5 h-5" />
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
      <div class="absolute inset-0 bg-[#0c1a13]/90 backdrop-blur-xl animate-in fade-in" @click="isAuditModalOpen = false"></div>
      
      <div class="bg-white rounded-[50px] w-full max-w-4xl max-h-[92vh] overflow-hidden shadow-2xl relative z-10 flex flex-col animate-in slide-in-from-bottom-10 duration-500 no-scrollbar">
        
        <div class="bg-gradient-to-br from-[#1a402d] via-[#1a402d] to-[#2d5c41] p-12 text-white relative">
          <button @click="isAuditModalOpen = false" class="absolute top-10 right-10 p-3 bg-white/10 hover:bg-white/20 rounded-full transition-all"><XIcon class="w-6 h-6" /></button>
          <div class="flex items-center gap-3 mb-4">
            <span class="px-3 py-1 bg-green-400 text-[#1a402d] text-[10px] font-black rounded-full uppercase tracking-widest tracking-widest">DNA & Yield Verified</span>
            <span class="text-white/40 font-mono text-sm uppercase">Audit_ID: {{ selectedAudit?.id }}</span>
          </div>
          <h2 class="text-5xl font-black tracking-tighter mb-2 italic uppercase">{{ selectedAudit?.tagId }}</h2>
          <p class="text-lg text-white/60 font-medium italic">Analisis Mendalam Performa Komoditas {{ selectedAudit?.breed }}</p>
        </div>

        <div class="p-12 overflow-y-auto no-scrollbar grid grid-cols-1 md:grid-cols-3 gap-12 bg-white">
          
          <div class="space-y-6">
            <h3 class="text-xs font-black text-green-700 uppercase tracking-[0.2em] flex items-center gap-2">
              <ZapIcon class="w-4 h-4" /> Growth Metrics
            </h3>
            <div class="space-y-4">
               <div v-for="metric in selectedAudit?.metrics" :key="metric.label" class="bg-gray-50 p-5 rounded-[32px] border border-gray-100 flex justify-between items-center shadow-inner">
                  <p class="text-[10px] font-black text-gray-400 uppercase">{{ metric.label }}</p>
                  <p class="text-xl font-black text-gray-800">{{ metric.value }}<small class="text-xs ml-0.5 text-gray-400">{{ metric.unit }}</small></p>
               </div>
            </div>
          </div>

          <div class="md:col-span-2 space-y-6">
            <h3 class="text-xs font-black text-[#1a402d] uppercase tracking-[0.2em] flex items-center gap-2">
              <ClipboardListIcon class="w-4 h-4" /> Intelligence Yield Evaluation
            </h3>
            <div class="bg-green-50/30 p-8 rounded-[40px] border border-green-100 relative overflow-hidden">
               <p class="text-[10px] font-black text-green-600 uppercase mb-2 tracking-widest">Quality Insight</p>
               <h4 class="text-3xl font-black text-[#1a402d] mb-4">High Performance Grade</h4>
               <p class="text-sm text-slate-600 font-medium leading-relaxed italic mb-8">
                 "Ternak menunjukkan konversi pakan yang luar biasa (FCR {{ selectedAudit?.fcr }}). Pertumbuhan massa otot di atas rata-rata populasi. Sangat direkomendasikan untuk program pemuliaan genetik batch berikutnya."
               </p>
               <div class="grid grid-cols-2 gap-4">
                  <div class="bg-white p-6 rounded-3xl shadow-sm">
                    <p class="text-[10px] font-black text-gray-400 uppercase">Yield Prediction</p>
                    <p class="text-2xl font-black text-gray-800">72.4% <small class="text-[10px] uppercase">Net Meat</small></p>
                  </div>
                  <div class="bg-white p-6 rounded-3xl shadow-sm">
                    <p class="text-[10px] font-black text-gray-400 uppercase">Health Consistency</p>
                    <p class="text-2xl font-black text-green-600">98.5% <small class="text-[10px] uppercase">Optimal</small></p>
                  </div>
               </div>
            </div>
          </div>

        </div>

        <div class="p-10 bg-gray-50 border-t border-gray-100 flex justify-between items-center">
          <p class="text-xs text-gray-400 font-black uppercase tracking-widest">© HELP-IN Precision Livestock Analytics 2026</p>
          <button @click="isAuditModalOpen = false" class="px-12 py-5 bg-[#1a402d] text-white rounded-3xl font-black shadow-xl hover:scale-105 active:scale-95 transition-all">Selesai Audit</button>
        </div>
      </div>
    </div>

  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { 
  LayoutDashboardIcon, HomeIcon, DogIcon, ActivityIcon, ClipboardListIcon, BarChart3Icon, SoupIcon,
  LogOutIcon, MenuIcon, XIcon, FileTextIcon, SearchIcon, EyeIcon, 
  TrendingUpIcon, ZapIcon, InfoIcon, ShieldCheckIcon
} from 'lucide-vue-next'
import { Line, Doughnut, Bar } from 'vue-chartjs'
import { Chart as ChartJS, Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale, PointElement, LineElement, ArcElement, Filler } from 'chart.js'

ChartJS.register(Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale, PointElement, LineElement, ArcElement, Filler)

const isSidebarOpen = ref(false)
const isAuditModalOpen = ref(false)
const selectedAudit = ref(null)
const searchQuery = ref('')

// ==========================================
// 1. DYNAMIC NAVIGATION
// ==========================================
const activeMenu = ref('Kualitas Ternak')
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
class QualityAuditModel {
  constructor(data) {
    this.id = data.id || `QA-${Math.floor(Math.random() * 9000) + 1000}`;
    this.tagId = data.tagId;
    this.breed = data.breed;
    this.grade = data.grade || 'B';
    this.fcr = parseFloat(data.fcr); // Feed Conversion Ratio
    this.adg = parseFloat(data.adg); // Average Daily Gain
    
    this.metrics = [
      { label: 'Meat Yield', value: data.meatYield || 70, unit: '%' },
      { label: 'Fat Thickness', value: data.fatThickness || 12, unit: 'mm' },
      { label: 'Growth Speed', value: data.adg, unit: 'kg/d' },
      { label: 'FCR Index', value: data.fcr, unit: 'score' }
    ];
  }

  static fromJSON(json) {
    return new QualityAuditModel({
      id: json.audit_id,
      tagId: json.id_ternak,
      breed: json.jenis_ras,
      grade: json.grade_akhir,
      fcr: json.feed_ratio,
      adg: json.daily_gain,
      meatYield: json.persentase_daging,
      fatThickness: json.ketebalan_lemak
    });
  }
}

// ==========================================
// 3. REACTIVE STATE & DATA
// ==========================================
const qualityDataList = ref([])

onMounted(() => {
  const dummyJSON = [
    { audit_id: 'QA-2026-001', id_ternak: 'BRH-001', jenis_ras: 'Brahman', grade_akhir: 'A', feed_ratio: 4.2, daily_gain: 1.2, persentase_daging: 74, ketebalan_lemak: 10 },
    { audit_id: 'QA-2026-002', id_ternak: 'LMS-002', jenis_ras: 'Limousin', grade_akhir: 'B', feed_ratio: 5.1, daily_gain: 0.9, persentase_daging: 68, ketebalan_lemak: 14 },
    { audit_id: 'QA-2026-003', id_ternak: 'ETW-045', jenis_ras: 'Etawa', grade_akhir: 'A', feed_ratio: 3.8, daily_gain: 0.4, persentase_daging: 70, ketebalan_lemak: 8 },
    { audit_id: 'QA-2026-004', id_ternak: 'BRH-012', jenis_ras: 'Brahman', grade_akhir: 'C', feed_ratio: 6.2, daily_gain: 0.7, persentase_daging: 60, ketebalan_lemak: 18 },
  ];
  qualityDataList.value = dummyJSON.map(d => QualityAuditModel.fromJSON(d));
});

// ==========================================
// 4. CHART DATA COMPUTATIONS
// ==========================================
const adgTrendData = computed(() => ({
  labels: ['W1', 'W2', 'W3', 'W4', 'W5', 'W6'],
  datasets: [{
    label: 'ADG Performance (kg/d)',
    data: [0.8, 1.1, 0.9, 1.2, 1.4, 1.3],
    borderColor: '#1a402d',
    backgroundColor: 'rgba(26, 64, 45, 0.1)',
    fill: true,
    tension: 0.4,
    pointRadius: 4
  }]
}))

const gradeDistributionData = computed(() => {
  const grades = { A: 0, B: 0, C: 0 };
  qualityDataList.value.forEach(i => grades[i.grade]++);
  return {
    labels: ['Grade A', 'Grade B', 'Reject (C)'],
    datasets: [{
      data: [grades.A, grades.B, grades.C],
      backgroundColor: ['#10b981', '#fbbf24', '#ef4444'],
      borderWidth: 0,
      hoverOffset: 15
    }]
  }
})

const fcrComparisonData = computed(() => {
  const breeds = {};
  qualityDataList.value.forEach(d => {
    breeds[d.breed] = (breeds[d.breed] || 0) + d.fcr
  });
  return {
    labels: Object.keys(breeds),
    datasets: [{
      label: 'FCR Index (Lower is Better)',
      data: Object.values(breeds).map(v => (v / 2).toFixed(1)),
      backgroundColor: '#1a402d',
      borderRadius: 12
    }]
  }
})

// ==========================================
// 5. HELPER STATS & METHODS
// ==========================================
const qualityMetrics = [
  { label: 'Avg Daily Gain', value: '1.24', unit: 'kg/d', icon: ZapIcon },
  { label: 'Overall Yield', value: '71.2', unit: '%', icon: ShieldCheckIcon },
]

const filteredQualityData = computed(() => {
  return qualityDataList.value.filter(i => i.tagId.toLowerCase().includes(searchQuery.value.toLowerCase()))
})

const openQualityAudit = (item) => {
  selectedAudit.value = item;
  isAuditModalOpen.value = true;
}

const printAudit = () => alert('Generating Comprehensive Quality Audit PDF...');

const chartOptions = { responsive: true, maintainAspectRatio: false, plugins: { legend: { display: false } }, scales: { y: { display: false }, x: { grid: { display: false } } } }
const barOptions = { responsive: true, maintainAspectRatio: false, plugins: { legend: { display: false } }, scales: { y: { beginAtZero: true }, x: { grid: { display: false } } } }
const doughnutOptions = { responsive: true, maintainAspectRatio: false, cutout: '82%', plugins: { legend: { display: false } } }

const formatDate = (d) => new Date(d).toLocaleDateString('id-ID', { day: '2-digit', month: 'short', year: 'numeric' });
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