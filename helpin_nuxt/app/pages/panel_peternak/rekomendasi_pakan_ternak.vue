<template>
  <div class="flex h-screen w-full bg-[#f4f7f5] font-sans overflow-hidden relative text-slate-900">
    
    <SidebarPeternak :isOpen="isSidebarOpen" @close="isSidebarOpen = false" />

    <main class="flex-1 flex flex-col overflow-y-auto relative w-full no-scrollbar">
      <header class="flex justify-between items-center px-6 md:px-10 py-6 border-b border-gray-200 bg-white/80 backdrop-blur-md z-10 sticky top-0">
        <div class="flex items-center gap-4">
          <button @click="isSidebarOpen = true" class="md:hidden p-2 text-gray-600 border rounded-lg transition"><MenuIcon class="w-6 h-6" /></button>
          <div class="flex items-center gap-3">
            <div class="bg-gradient-to-tr from-yellow-400 to-orange-500 p-2.5 rounded-2xl shadow-xl shadow-orange-100">
              <SparklesIcon class="w-6 h-6 text-white animate-pulse" />
            </div>
            <div>
              <h1 class="text-xl md:text-2xl font-black text-gray-800 tracking-tight uppercase">Rekomendasi Pakan Ternak</h1>
              <p class="text-xs text-gray-500 font-medium italic hidden sm:block">Deep Nutritional Audit & Growth Optimization</p>
            </div>
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

      <div class="p-4 md:p-10 space-y-8 flex flex-col">
        


        <section class="grid grid-cols-1 md:grid-cols-3 gap-6">
          <div v-for="stat in topStats" :key="stat.label" class="bg-white p-8 rounded-[35px] border border-gray-100 shadow-sm relative overflow-hidden group">
            <div class="absolute -right-4 -bottom-4 bg-green-50 w-24 h-24 rounded-full opacity-50 group-hover:scale-150 transition-transform duration-500"></div>
            <p class="text-[10px] font-black text-gray-400 uppercase tracking-widest mb-2">{{ stat.label }}</p>
            <h3 class="text-4xl font-black text-[#1a402d]">{{ stat.value }} <small class="text-xs text-green-500">{{ stat.unit }}</small></h3>
          </div>
        </section>
        <button 
          @click="openInputModal" 
          class="self-end bg-[#1a402d] text-white px-8 py-4 rounded-[24px] font-black shadow-2xl shadow-green-900/30 flex items-center gap-3 hover:scale-105 active:scale-95 transition-all shrink-0"
        >
          <ZapIcon class="w-5 h-5 text-yellow-400 fill-current" />
          <span class="uppercase tracking-widest text-sm">Analisa Pakan</span>
        </button>
        <section class="bg-white rounded-[45px] shadow-sm border border-gray-100 overflow-hidden mb-10">
          
          <div class="p-8 border-b border-gray-100 flex justify-between items-center bg-gray-50/50">
            <h2 class="text-xl font-black text-gray-800 tracking-tight uppercase">Intelligence Audit History</h2>
            <div class="relative">
              <SearchIcon class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-gray-400" />
              <input v-model="searchQuery" type="text" placeholder="Search Tag ID..." class="pl-10 pr-4 py-3 bg-white border-none rounded-2xl text-sm font-bold w-64 shadow-inner focus:ring-2 focus:ring-green-500/20" />
            </div>
          </div>
          
          <div class="overflow-x-auto no-scrollbar">
            <table class="w-full text-left border-collapse min-w-[1000px]">
              <thead>
                <tr class="bg-gray-50/30 text-[11px] font-black text-gray-400 uppercase tracking-[0.2em] border-b border-gray-100">
                  <th class="px-8 py-6">Audit ID & Target</th>
                  <th class="px-8 py-6">Formulation Matrix</th>
                  <th class="px-8 py-6 text-center">Core Energy (TDN)</th>
                  <th class="px-8 py-6 text-center">Efficiency</th>
                  <th class="px-8 py-6 text-center">Audit</th>
                </tr>
              </thead>
              <tbody class="divide-y divide-gray-50">
                <tr v-for="rec in filteredRecs" :key="rec.id" class="hover:bg-green-50/30 transition-all group">
                  <td class="px-8 py-6">
                    <div class="flex flex-col">
                      <span class="font-black text-gray-800 text-lg tracking-tighter uppercase">{{ rec.tagId }}</span>
                      <span class="text-[10px] text-gray-400 font-bold uppercase tracking-widest">{{ rec.breed }} • {{ formatDate(rec.createdAt) }}</span>
                    </div>
                  </td>
                  <td class="px-8 py-6">
                    <div class="flex flex-wrap gap-2">
                       <span v-for="ing in rec.ingredients" :key="ing" class="px-3 py-1 bg-white border border-gray-100 text-[#1a402d] text-[10px] font-black rounded-lg shadow-sm">
                         {{ ing }}
                       </span>
                    </div>
                  </td>
                  <td class="px-8 py-6 text-center">
                    <div class="flex flex-col items-center">
                       <span class="text-xl font-black text-orange-600">{{ rec.nutrition.tdn }}%</span>
                       <span class="text-[9px] font-bold text-gray-400 uppercase">Total Digestible</span>
                    </div>
                  </td>
                  <td class="px-8 py-6 text-center">
                    <div class="flex flex-col items-center">
                       <span class="text-xl font-black text-blue-600">{{ rec.efficiency }}%</span>
                       <div class="h-1 w-16 bg-gray-100 rounded-full mt-1 overflow-hidden">
                          <div class="h-full bg-blue-500" :style="`width: ${rec.efficiency}%`" ></div>
                       </div>
                    </div>
                  </td>
                  <td class="px-8 py-6 text-center">
                    <button @click="showFullAudit(rec)" class="p-4 bg-white border border-gray-100 text-gray-400 rounded-2xl hover:bg-[#1a402d] hover:text-white hover:shadow-xl transition-all">
                      <BarChart3Icon class="w-6 h-6" />
                    </button>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </section>
      </div>
    </main>

    <div v-if="isInputModalOpen" class="fixed inset-0 z-50 flex items-center justify-center p-4">
      <div class="absolute inset-0 bg-[#0c1a13]/90 backdrop-blur-xl" @click="closeInputModal"></div>
      <div class="bg-white rounded-[50px] w-full max-w-2xl overflow-hidden shadow-2xl relative z-10 flex flex-col animate-in slide-in-from-bottom-10 duration-500">
        
        <div v-if="isProcessing" class="absolute inset-0 bg-white/95 z-50 flex flex-col items-center justify-center text-center p-10">
           <div class="relative mb-8">
              <div class="w-32 h-32 border-4 border-gray-100 border-t-[#1a402d] rounded-full animate-spin"></div>
              <SparklesIcon class="absolute inset-0 m-auto w-10 h-10 text-yellow-500 fill-current animate-bounce" />
           </div>
           <h3 class="text-3xl font-black text-gray-800 mb-2 italic">NEURAL OPTIMIZATION</h3>
           <p class="text-sm text-gray-400 font-bold max-w-xs leading-relaxed uppercase tracking-widest">
             Menganalisis korelasi massa ({{ formData.weight }}KG) terhadap standar nutrisi NRC ras {{ formData.breed }}...
           </p>
           <div class="mt-10 w-72 h-2 bg-gray-100 rounded-full overflow-hidden shadow-inner">
             <div class="h-full bg-gradient-to-r from-green-600 to-yellow-400 animate-progress"></div>
           </div>
        </div>

        <div class="bg-gradient-to-br from-[#1a402d] via-[#1a402d] to-[#2d5c41] p-12 text-white relative">
          <button @click="closeInputModal" class="absolute top-8 right-8 p-2 bg-white/10 hover:bg-white/20 rounded-full transition-all"><XIcon class="w-5 h-5" /></button>
          <div class="flex items-center gap-3 mb-4">
            <SparklesIcon class="w-5 h-5 text-yellow-400" />
            <span class="text-[10px] font-black uppercase tracking-widest text-green-400">Smart Formulation Engine</span>
          </div>
          <h2 class="text-4xl font-black italic tracking-tighter">Generate Feed Logic</h2>
        </div>

        <div class="p-12 space-y-8">
          <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
            <div class="space-y-2">
              <label class="text-[10px] font-black text-gray-400 uppercase tracking-widest ml-2">Livestock Identification</label>
              <input v-model="formData.tagId" type="text" placeholder="TAG-ID" class="w-full px-6 py-5 bg-gray-50 border border-gray-100 rounded-3xl font-black text-xl outline-none focus:ring-4 focus:ring-green-500/10 transition-all" />
            </div>
            <div class="space-y-2">
              <label class="text-[10px] font-black text-gray-400 uppercase tracking-widest ml-2">Genetic Breed</label>
              <select v-model="formData.breed" class="w-full px-6 py-5 bg-gray-50 border border-gray-100 rounded-3xl font-black text-xl outline-none focus:ring-4 focus:ring-green-500/10 transition-all">
                <option value="Brahman">Brahman</option>
                <option value="Limousin">Limousin</option>
                <option value="Etawa">Etawa</option>
              </select>
            </div>
          </div>
          <div class="space-y-2">
              <label class="text-[10px] font-black text-gray-400 uppercase tracking-widest ml-2">Animal Mass Analysis (kg)</label>
              <input v-model.number="formData.weight" type="number" class="w-full px-8 py-6 bg-gray-50 border border-gray-100 rounded-[35px] font-black text-4xl text-[#1a402d] outline-none" />
          </div>
          <div class="flex gap-4 pt-6">
            <button @click="closeInputModal" class="flex-1 py-5 bg-gray-100 text-gray-500 rounded-3xl font-black uppercase text-xs tracking-widest hover:bg-gray-200 transition-all">Batal</button>
            <button @click="runAIEngine" class="flex-[2] py-5 bg-[#1a402d] text-white rounded-[30px] font-black text-lg shadow-2xl hover:scale-105 transition-all uppercase tracking-widest flex items-center justify-center gap-3">
              <ZapIcon class="w-5 h-5 text-yellow-400 fill-current" /> Start Deep Analysis
            </button>
          </div>
        </div>
      </div>
    </div>

    <div v-if="isResultModalOpen" class="fixed inset-0 z-[60] flex items-center justify-center p-4">
      <div class="absolute inset-0 bg-[#0c1a13]/95 backdrop-blur-2xl animate-in fade-in" @click="isResultModalOpen = false"></div>
      
      <div class="bg-white rounded-[60px] w-full max-w-5xl max-h-[95vh] overflow-hidden shadow-2xl relative z-10 flex flex-col animate-in zoom-in duration-500">
        
        <div class="bg-gradient-to-br from-[#1a402d] to-[#2d5c41] p-12 text-white relative shadow-2xl z-20">
          <button @click="isResultModalOpen = false" class="absolute top-10 right-10 p-3 bg-white/10 hover:bg-red-500 rounded-full transition-all"><XIcon class="w-6 h-6" /></button>
          <div class="flex items-center gap-4 mb-6">
            <div class="bg-yellow-400 text-[#1a402d] px-4 py-1.5 rounded-full font-black text-[10px] uppercase tracking-widest shadow-lg shadow-yellow-400/20">AI Recommended Logic</div>
            <span class="text-white/40 font-mono text-sm tracking-widest italic">VERIFIED_LOG: #{{ selectedRec?.id }}</span>
          </div>
          <h2 class="text-7xl font-black tracking-tighter italic uppercase mb-2">{{ selectedRec?.tagId }}</h2>
          <p class="text-2xl text-green-400 font-bold italic tracking-tight">{{ selectedRec?.feedName }}</p>
        </div>

        <div class="p-12 overflow-y-auto no-scrollbar grid grid-cols-1 md:grid-cols-3 gap-12 bg-white flex-1">
          <div class="space-y-8 border-r border-gray-100 pr-8">
            <h3 class="text-[11px] font-black text-gray-400 uppercase tracking-[0.3em] pb-4 border-b border-gray-100">Ingredient Formulation Matrix</h3>
            <div class="space-y-3">
              <div v-for="ing in selectedRec?.ingredients" :key="ing" class="flex items-center gap-4 bg-gray-50 p-4 rounded-2xl border border-gray-100 hover:border-green-300 transition-all">
                <div class="w-2 h-2 rounded-full bg-green-500 shadow-[0_0_10px_rgba(34,197,94,0.5)]"></div>
                <span class="font-black text-gray-700 text-xs uppercase">{{ ing }}</span>
              </div>
            </div>
            <div class="bg-blue-50 p-8 rounded-[40px] border border-blue-100 flex flex-col items-center text-center shadow-inner">
               <p class="text-[10px] font-black text-blue-400 uppercase mb-2 tracking-widest">Digestibility Index</p>
               <h4 class="text-5xl font-black text-blue-900">{{ selectedRec?.digestibility }}<small class="text-xl">%</small></h4>
               <p class="text-[9px] font-black text-blue-700 mt-2 bg-blue-200 px-3 py-1 rounded-full uppercase">High Bioavailability</p>
            </div>
          </div>

          <div class="md:col-span-2 space-y-12">
            <div>
              <h3 class="text-[11px] font-black text-gray-400 uppercase tracking-[0.3em] mb-8">Deep Nutritional Intelligence (%)</h3>
              <div class="grid grid-cols-1 md:grid-cols-2 gap-x-12 gap-y-8">
                 <div v-for="(val, key) in selectedRec?.nutrition" :key="key" class="space-y-3">
                    <div class="flex justify-between items-end">
                       <span class="text-xs font-black text-slate-800 uppercase tracking-tighter">{{ key }} Content</span>
                       <span class="text-xl font-black text-[#1a402d]">{{ val }}%</span>
                    </div>
                    <div class="h-3 w-full bg-gray-100 rounded-full overflow-hidden shadow-inner border border-gray-50">
                       <div :class="['h-full rounded-full transition-all duration-1000', getNutrientColor(key)]" :style="`width: ${getBarWidth(key, val)}%`" ></div>
                    </div>
                 </div>
              </div>
            </div>

            <div class="bg-gradient-to-r from-[#1a402d] to-[#2d5c41] p-10 rounded-[45px] text-white flex flex-col md:flex-row justify-between items-center gap-8 shadow-2xl relative overflow-hidden group">
               <div class="absolute -left-10 -bottom-10 w-40 h-40 bg-white/5 rounded-full group-hover:scale-110 transition-transform duration-700"></div>
               <div class="relative z-10 text-center md:text-left">
                  <p class="text-[10px] font-black text-green-400 uppercase tracking-widest mb-1">Gain Prediction Score</p>
                  <p class="text-5xl font-black text-white">+{{ selectedRec?.gainPrediction }} <span class="text-sm font-medium italic text-green-400">kg/day</span></p>
               </div>
               <div class="h-px w-20 bg-white/20 hidden md:block"></div>
               <div class="relative z-10 text-center md:text-right">
                  <p class="text-[10px] font-black text-green-400 uppercase tracking-widest mb-1">Formulation Logic</p>
                  <p class="text-2xl font-black text-white uppercase italic tracking-tighter">Aggressive Growth</p>
               </div>
            </div>

            <div class="bg-orange-50 p-8 rounded-[40px] border border-orange-100 flex items-start gap-4">
              <InfoIcon class="w-8 h-8 text-orange-400 shrink-0" />
              <div class="space-y-1">
                 <p class="text-xs font-black text-orange-900 uppercase tracking-widest">Feeding Protocol Advisory</p>
                 <p class="text-sm font-bold text-orange-700 leading-relaxed italic">"Berdasarkan analisis massa {{ selectedRec?.weight }}KG, rasio pakan adalah 10% dari berat badan (BK). Berikan pakan 3x sehari untuk optimalisasi rumen dan efisiensi metabolik."</p>
              </div>
            </div>
          </div>
        </div>

        <div class="p-10 bg-gray-50 border-t border-gray-100 text-center shrink-0">
          <button @click="isResultModalOpen = false" class="px-24 py-5 bg-[#1a402d] text-white rounded-[35px] font-black shadow-xl hover:scale-105 active:scale-95 transition-all uppercase tracking-[0.2em] text-xs">Acknowledge Formulation</button>
        </div>
      </div>
    </div>

  </div>
</template>

<script setup>
import { ref, reactive, computed, onMounted } from 'vue'
import { 
  LayoutDashboardIcon, HomeIcon, DogIcon, ActivityIcon, ClipboardListIcon, BarChart3Icon,
  LogOutIcon, MenuIcon, XIcon, ZapIcon, SparklesIcon, SearchIcon, EyeIcon, 
  TrendingUpIcon, SoupIcon, InfoIcon, FileTextIcon
} from 'lucide-vue-next'

// ==========================================
// 1. DATA MODEL (DEEP INTELLIGENCE)
// ==========================================
class FeedRecModel {
  constructor(data) {
    this.id = data.id || Math.floor(Math.random() * 90000) + 10000;
    this.tagId = data.tagId;
    this.breed = data.breed;
    this.weight = data.weight;
    this.feedName = data.feedName || "HELP-IN SMART GROWTH MIX";
    this.efficiency = data.efficiency || 94;
    this.digestibility = data.digestibility || 88.5;
    this.gainPrediction = data.gainPrediction || 1.15;
    
    // Rincian Nutrisi Sesuai Flowchart Pakar
    this.nutrition = {
      protein: data.protein || 18.2,
      energy_tdn: data.tdn || 68.0,
      dry_matter: data.dm || 86.0,
      fiber: data.fiber || 12.5,
      fat: data.fat || 4.8,
      calcium: data.calcium || 1.2,
      phosphorus: data.phosphorus || 0.6
    };
    
    this.ingredients = data.ingredients || ['Silase Jagung', 'Bungkil Kedelai', 'Mineral Premix', 'Tetes Tebu'];
    this.createdAt = new Date().toISOString();
  }
}

// ==========================================
// 2. REACTIVE STATE
// ==========================================
const isSidebarOpen = ref(false)
const activeMenu = ref('Rekomendasi Pakan')
const feedRecs = ref([])
const searchQuery = ref('')

const isInputModalOpen = ref(false)
const isResultModalOpen = ref(false)
const isProcessing = ref(false)
const selectedRec = ref(null)

const formData = reactive({ tagId: '', breed: 'Brahman', weight: 0 })

onMounted(() => {
  feedRecs.value = [
    new FeedRecModel({ tagId: 'BRH-001', breed: 'Brahman', weight: 450, protein: 16.5, tdn: 65, dm: 84, fiber: 14.5, fat: 3.2, gainPrediction: 1.05 }),
    new FeedRecModel({ tagId: 'LMS-002', breed: 'Limousin', weight: 520, protein: 19.8, tdn: 72, dm: 88, fiber: 10.2, fat: 5.8, gainPrediction: 1.42, feedName: "MAX-MUSCLE PRO" }),
  ]
})

// ==========================================
// 3. SMART ANALYTICS LOGIC
// ==========================================
const openInputModal = () => isInputModalOpen.value = true;
const closeInputModal = () => !isProcessing.value && (isInputModalOpen.value = false);

const runAIEngine = () => {
  if (!formData.tagId || formData.weight <= 0) return alert('Input data tidak valid!');
  isProcessing.value = true;
  
  // Simulasi Neural Analysis Berbasis Berat (3.5 Detik)
  setTimeout(() => {
    const isHeavy = formData.weight > 500;
    
    const newRec = new FeedRecModel({
      tagId: formData.tagId,
      breed: formData.breed,
      weight: formData.weight,
      protein: isHeavy ? 20.5 : 17.8,
      tdn: isHeavy ? 74.0 : 66.5,
      dm: isHeavy ? 90.0 : 85.0,
      fiber: isHeavy ? 9.2 : 14.0,
      fat: isHeavy ? 6.2 : 4.0,
      calcium: isHeavy ? 2.2 : 1.1,
      phosphorus: isHeavy ? 0.9 : 0.5,
      gainPrediction: isHeavy ? 1.55 : 1.10,
      feedName: isHeavy ? "MAX-GROWTH NEURAL FORMULA" : "STANDARD MAINTENANCE MIX",
      ingredients: formData.breed === 'Etawa' ? ['Rumput Gajah', 'Vitamins', 'Dedak'] : ['Corn Silage', 'Soybean Meal', 'Mineral Premix', 'Molasses']
    });

    feedRecs.value.unshift(newRec);
    selectedRec.value = newRec; 
    
    isProcessing.value = false;
    isInputModalOpen.value = false;
    isResultModalOpen.value = true; // AUTO-SHOW RESULT PASSPORT
    
    formData.tagId = ''; formData.weight = 0;
  }, 3500);
}

const showFullAudit = (rec) => {
  selectedRec.value = rec;
  isResultModalOpen.value = true;
}

const getNutrientColor = (key) => {
  const map = { protein: 'bg-blue-500', energy_tdn: 'bg-orange-500', dry_matter: 'bg-slate-700', fiber: 'bg-green-600', fat: 'bg-yellow-500', calcium: 'bg-purple-500', phosphorus: 'bg-indigo-500' };
  return map[key] || 'bg-slate-400';
}

const getBarWidth = (key, val) => {
  if (key === 'energy_tdn' || key === 'dry_matter') return val;
  return val * 4.5; // Scale up percentage visual
}

const topStats = computed(() => [
  { label: 'Neural Accuracy', value: '98.4', unit: '%' },
  { label: 'Active Audit Logs', value: feedRecs.value.length, unit: 'LOGS' },
  { label: 'Avg Biomass Gain', value: '+1.24', unit: 'kg/day' },
]);

const filteredRecs = computed(() => feedRecs.value.filter(i => i.tagId.toLowerCase().includes(searchQuery.value.toLowerCase())))
const formatDate = (d) => new Date(d).toLocaleDateString('id-ID', { day: 'numeric', month: 'short' });

// // SHARED NAV
// const menus = [
//   { name: 'Dashboard', icon: LayoutDashboardIcon, path: '/panel_peternak/dashboard_peternak' },
//   { name: 'Kandang', icon: HomeIcon, path: '/panel_peternak/kandang_peternak' },
//   { name: 'Data Ternak', icon: DogIcon, path: '/panel_peternak/ternak' },
//   { name: 'Kualitas Ternak', icon: BarChart3Icon, path: '/panel_peternak/kualitas_ternak' },
//   { name: 'Kondisi Kesehatan', icon: ActivityIcon, path: '/panel_peternak/kondisi_kesehatan_ternak' },
//   { name: 'Rekomendasi Pakan', icon: SoupIcon, path: '/panel_peternak/rekomendasi_pakan_ternak' },
// ]
</script>

<style scoped>
.no-scrollbar::-webkit-scrollbar { display: none; }
.no-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }

.active-menu::before { content: ""; position: absolute; right: 0; top: -24px; width: 24px; height: 24px; background-color: transparent; border-bottom-right-radius: 24px; box-shadow: 12px 12px 0 12px #f4f7f5; pointer-events: none; }
.active-menu::after { content: ""; position: absolute; right: 0; bottom: -24px; width: 24px; height: 24px; background-color: transparent; border-top-right-radius: 24px; box-shadow: 12px -12px 0 12px #f4f7f5; pointer-events: none; }

.animate-spin-slow { animation: spin 8s linear infinite; }
@keyframes spin { from { transform: rotate(0deg); } to { transform: rotate(360deg); } }

.animate-progress { animation: progress 3.5s ease-in-out forwards; width: 0%; }
@keyframes progress { 0% { width: 0%; } 20% { width: 35%; } 80% { width: 85%; } 100% { width: 100%; } }

.animate-in { animation: fadeIn 0.5s cubic-bezier(0.16, 1, 0.3, 1) forwards; }
@keyframes fadeIn { from { opacity: 0; transform: translateY(20px); } to { opacity: 1; transform: translateY(0); } }

.zoom-in { animation: zoomIn 0.4s cubic-bezier(0.16, 1, 0.3, 1) forwards; }
@keyframes zoomIn { from { opacity: 0; transform: scale(0.9); } to { opacity: 1; transform: scale(1); } }
</style>