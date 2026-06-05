<template>
  <div class="flex h-screen w-full bg-[#f4f7f5] font-sans overflow-hidden relative text-slate-900">
    
    <div v-if="isSidebarOpen" @click="isSidebarOpen = false" class="fixed inset-0 bg-black/60 z-30 md:hidden backdrop-blur-sm transition-opacity"></div>

    <aside :class="['w-[280px] bg-[#1a402d] text-white flex flex-col justify-between shrink-0 absolute inset-y-0 left-0 z-40 transform transition-transform duration-300 md:relative md:translate-x-0', isSidebarOpen ? 'translate-x-0' : '-translate-x-full']">
      <div class="overflow-y-auto h-full no-scrollbar flex flex-col justify-between">
        <div>
          <div class="px-6 md:px-8 py-8 flex justify-between items-center">
            <div class="flex items-baseline gap-1">
              <h1 class="text-3xl font-black tracking-tighter italic text-white">HELPIN</h1>
              <span class="text-[10px] uppercase font-bold text-green-400 tracking-widest">AI Pro</span>
            </div>
            <button @click="isSidebarOpen = false" class="md:hidden text-white"><XIcon class="w-6 h-6" /></button>
          </div>

          <div class="px-6 md:px-8 mb-8 border-b border-[#23533b] pb-6 bg-[#173a28]">
            <div class="flex items-center gap-2 mb-2">
              <div class="w-2 h-2 rounded-full bg-green-500 animate-pulse"></div>
              <span class="text-xs font-semibold text-green-400 uppercase tracking-widest">Intelligence Active</span>
            </div>
            <h2 class="text-xl font-bold mb-2 text-white">Petani Suki</h2>
            <span class="inline-block px-3 py-1 text-[10px] font-bold border border-green-600 text-green-400 rounded uppercase">Breeder Authority</span>
          </div>

          <nav class="flex flex-col gap-1 pl-4">
            <NuxtLink v-for="menu in menus" :key="menu.name" :to="menu.path" :class="['relative flex items-center gap-4 px-4 py-4 transition-all', activeMenu === menu.name ? 'active-menu bg-[#f4f7f5] text-[#1a402d] rounded-l-full font-black shadow-lg' : 'text-gray-400 hover:text-white font-bold']">
              <component :is="menu.icon" class="w-5 h-5" /> {{ menu.name }}
            </NuxtLink>
          </nav>
        </div>
        <div class="p-6">
          <div class="bg-[#143222] rounded-xl p-4 flex items-center justify-between border border-white/5">
            <p class="text-sm font-bold truncate text-white">Admin Suki SUPER</p>
            <SparklesIcon class="w-5 h-5 text-yellow-400 animate-spin-slow" />
          </div>
        </div>
      </div>
    </aside>

    <main class="flex-1 flex flex-col overflow-y-auto relative w-full no-scrollbar">
      <header class="flex justify-between items-center px-6 md:px-10 py-6 border-b border-gray-200 bg-white/80 backdrop-blur-md z-10 sticky top-0">
        <div class="flex items-center gap-4">
          <button @click="isSidebarOpen = true" class="md:hidden p-2 text-gray-600 border rounded-lg"><MenuIcon class="w-6 h-6" /></button>
          <div class="flex items-center gap-3">
            <div class="bg-gradient-to-tr from-yellow-400 to-orange-500 p-2.5 rounded-2xl shadow-xl shadow-orange-100">
              <SparklesIcon class="w-6 h-6 text-white animate-pulse" />
            </div>
            <div>
              <h1 class="text-2xl font-black text-gray-800 tracking-tight">AI FEED ADVISOR</h1>
              <p class="text-xs text-gray-500 font-medium italic">Neural Network Optimization for Livestock Growth</p>
            </div>
          </div>
        </div>
        <button @click="openInputModal" class="bg-[#1a402d] text-white px-8 py-3 rounded-2xl font-black shadow-2xl shadow-green-900/30 flex items-center gap-3 hover:scale-105 transition-all">
          <ZapIcon class="w-4 h-4 text-yellow-400 fill-current" /> GENERATE AI
        </button>
      </header>

      <div class="p-4 md:p-10 space-y-8">
        
        <section class="grid grid-cols-1 md:grid-cols-3 gap-6">
          <div v-for="stat in topStats" :key="stat.label" class="bg-white p-8 rounded-[35px] border border-gray-100 shadow-sm relative overflow-hidden group">
            <div class="absolute -right-4 -bottom-4 bg-gray-50 w-24 h-24 rounded-full opacity-50 group-hover:scale-125 transition-transform duration-500"></div>
            <p class="text-[10px] font-black text-gray-400 uppercase tracking-widest mb-2">{{ stat.label }}</p>
            <h3 class="text-4xl font-black text-[#1a402d]">{{ stat.value }} <small class="text-xs text-green-500">{{ stat.unit }}</small></h3>
          </div>
        </section>

        <section class="bg-white rounded-[45px] shadow-sm border border-gray-100 overflow-hidden mb-10">
          <div class="p-8 border-b border-gray-100 flex justify-between items-center bg-gray-50/50">
            <h2 class="text-xl font-black text-gray-800 tracking-tight uppercase">Recent Intelligence Audits</h2>
            <div class="relative">
              <SearchIcon class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-gray-400" />
              <input v-model="searchQuery" type="text" placeholder="Search Tag ID..." class="pl-10 pr-4 py-3 bg-white border-none rounded-2xl text-sm font-bold w-64 shadow-inner focus:ring-2 focus:ring-green-500/20" />
            </div>
          </div>
          
          <div class="overflow-x-auto no-scrollbar">
            <table class="w-full text-left border-collapse min-w-[1000px]">
              <thead>
                <tr class="bg-gray-50/30 text-[11px] font-black text-gray-400 uppercase tracking-[0.2em] border-b border-gray-100">
                  <th class="px-8 py-6">Reference ID</th>
                  <th class="px-8 py-6">Feed Formulation</th>
                  <th class="px-8 py-6 text-center">Core Nutrition</th>
                  <th class="px-8 py-6 text-center">Action</th>
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
                       <span v-for="ing in rec.ingredients" :key="ing" class="px-3 py-1 bg-white border border-gray-200 text-[#1a402d] text-[10px] font-black rounded-lg shadow-sm">
                         {{ ing }}
                       </span>
                    </div>
                  </td>
                  <td class="px-8 py-6">
                    <div class="flex justify-center gap-6">
                       <div class="flex flex-col items-center">
                          <span class="text-[9px] font-black text-gray-400 uppercase">Protein</span>
                          <span class="font-black text-blue-600 text-base">{{ rec.nutrition.protein }}%</span>
                       </div>
                       <div class="flex flex-col items-center">
                          <span class="text-[9px] font-black text-gray-400 uppercase">Energy</span>
                          <span class="font-black text-orange-600 text-base">{{ rec.nutrition.energy }} <small class="text-[10px]">Mcal</small></span>
                       </div>
                    </div>
                  </td>
                  <td class="px-8 py-6 text-center">
                    <button @click="showFullAudit(rec)" class="p-4 bg-[#1a402d] text-white rounded-2xl hover:rotate-6 hover:shadow-xl hover:shadow-green-900/20 transition-all">
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

    <div v-if="isInputModalOpen" class="fixed inset-0 z-50 flex items-center justify-center p-4">
      <div class="absolute inset-0 bg-[#0c1a13]/90 backdrop-blur-xl" @click="closeInputModal"></div>
      <div class="bg-white rounded-[50px] w-full max-w-2xl overflow-hidden shadow-2xl relative z-10 flex flex-col animate-in slide-in-from-bottom-10">
        
        <div v-if="isProcessing" class="absolute inset-0 bg-white/95 z-50 flex flex-col items-center justify-center text-center p-10">
           <div class="relative mb-8">
              <div class="w-32 h-32 border-4 border-gray-100 border-t-[#1a402d] rounded-full animate-spin"></div>
              <ZapIcon class="absolute inset-0 m-auto w-10 h-10 text-yellow-500 fill-current animate-pulse" />
           </div>
           <h3 class="text-3xl font-black text-gray-800 mb-2 italic">CALCULATING BIO-METRICS</h3>
           <p class="text-sm text-gray-400 font-medium max-w-xs leading-relaxed uppercase tracking-widest">
             Optimizing {{ formData.breed }} growth formula for {{ formData.weight }}KG mass...
           </p>
           <div class="mt-10 w-72 h-2 bg-gray-100 rounded-full overflow-hidden">
             <div class="h-full bg-gradient-to-r from-green-600 to-yellow-400 animate-progress"></div>
           </div>
        </div>

        <div class="bg-gradient-to-br from-[#1a402d] to-[#2d5c41] p-12 text-white">
          <h2 class="text-5xl font-black tracking-tighter italic">Feed Intelligence</h2>
          <p class="text-white/60 font-medium mt-2">Generate Smart Nutritional Program</p>
        </div>

        <div class="p-12 space-y-8">
          <div class="grid grid-cols-2 gap-8">
            <div class="space-y-2">
              <label class="text-[10px] font-black text-gray-400 uppercase tracking-widest ml-2">Livestock Tag</label>
              <input v-model="formData.tagId" type="text" placeholder="E.g: BRH-098" class="w-full px-6 py-5 bg-gray-50 border border-gray-100 rounded-3xl font-black text-xl outline-none focus:ring-4 focus:ring-green-500/10 transition-all" />
            </div>
            <div class="space-y-2">
              <label class="text-[10px] font-black text-gray-400 uppercase tracking-widest ml-2">Breed Type</label>
              <select v-model="formData.breed" class="w-full px-6 py-5 bg-gray-50 border border-gray-100 rounded-3xl font-black text-xl outline-none focus:ring-4 focus:ring-green-500/10">
                <option value="Brahman">Brahman</option>
                <option value="Limousin">Limousin</option>
                <option value="Etawa">Etawa</option>
              </select>
            </div>
          </div>
          <div class="space-y-2">
              <label class="text-[10px] font-black text-gray-400 uppercase tracking-widest ml-2">Mass Analysis (kg)</label>
              <input v-model.number="formData.weight" type="number" class="w-full px-8 py-6 bg-gray-50 border border-gray-100 rounded-[35px] font-black text-4xl text-[#1a402d] outline-none" />
          </div>
          <button @click="runAIEngine" class="w-full py-6 bg-[#1a402d] text-white rounded-[30px] font-black text-lg shadow-2xl hover:scale-[1.02] transition-all uppercase tracking-widest">
            START NEURAL ANALYSIS
          </button>
        </div>
      </div>
    </div>

    <div v-if="isResultModalOpen" class="fixed inset-0 z-[60] flex items-center justify-center p-4">
      <div class="absolute inset-0 bg-[#0c1a13]/95 backdrop-blur-2xl animate-in fade-in" @click="isResultModalOpen = false"></div>
      
      <div class="bg-white rounded-[60px] w-full max-w-5xl max-h-[95vh] overflow-hidden shadow-2xl relative z-10 flex flex-col animate-in zoom-in duration-500">
        
        <div class="bg-[#1a402d] p-12 text-white relative">
          <button @click="isResultModalOpen = false" class="absolute top-10 right-10 p-3 bg-white/10 hover:bg-red-500 rounded-full transition-all"><XIcon class="w-6 h-6" /></button>
          <div class="flex items-center gap-4 mb-6">
            <div class="bg-yellow-400 text-[#1a402d] px-4 py-1.5 rounded-full font-black text-[10px] uppercase tracking-widest shadow-lg shadow-yellow-400/20">AI Recommended</div>
            <span class="text-white/40 font-mono text-sm tracking-widest">BATCH_LOG: #{{ selectedRec?.id }}</span>
          </div>
          <h2 class="text-6xl font-black tracking-tighter italic uppercase mb-2">{{ selectedRec?.tagId }}</h2>
          <p class="text-xl text-green-400 font-bold italic tracking-tight">{{ selectedRec?.feedName }}</p>
        </div>

        <div class="p-12 overflow-y-auto no-scrollbar grid grid-cols-1 md:grid-cols-3 gap-12">
          <div class="space-y-8">
            <h3 class="text-xs font-black text-gray-400 uppercase tracking-[0.2em] border-b border-gray-100 pb-4">Main Composition</h3>
            <div class="space-y-4">
              <div v-for="ing in selectedRec?.ingredients" :key="ing" class="flex items-center gap-4 bg-gray-50 p-5 rounded-3xl border border-gray-100">
                <div class="w-2 h-2 rounded-full bg-green-500 shadow-[0_0_10px_rgba(34,197,94,0.5)]"></div>
                <span class="font-black text-gray-700 text-sm uppercase">{{ ing }}</span>
              </div>
            </div>
            <div class="bg-orange-50 p-6 rounded-[35px] border border-orange-100">
              <p class="text-[10px] font-black text-orange-400 uppercase mb-2">Feeding Method</p>
              <p class="text-sm font-bold text-orange-900 leading-relaxed italic">"Pemberian 2x sehari (pagi & sore) dengan perbandingan air 1:2 untuk hidrasi optimal."</p>
            </div>
          </div>

          <div class="md:col-span-2 space-y-10">
            <div>
              <h3 class="text-xs font-black text-gray-400 uppercase tracking-[0.2em] mb-6">Deep Nutritional Audit (%)</h3>
              <div class="grid grid-cols-1 gap-6">
                 <div v-for="(val, key) in selectedRec?.nutrition" :key="key" class="space-y-3">
                    <div class="flex justify-between items-end">
                       <span class="text-sm font-black text-slate-800 uppercase tracking-tighter">{{ key }} Content</span>
                       <span class="text-lg font-black text-[#1a402d]">{{ val }}%</span>
                    </div>
                    <div class="h-3 w-full bg-gray-100 rounded-full overflow-hidden shadow-inner">
                       <div :class="['h-full rounded-full transition-all duration-1000 shadow-lg', getNutrientColor(key)]" :style="`width: ${val * 4}%`" ></div>
                    </div>
                 </div>
              </div>
            </div>

            <div class="grid grid-cols-2 gap-6">
               <div class="bg-blue-50 p-8 rounded-[40px] border border-blue-100 flex flex-col items-center text-center">
                  <p class="text-[10px] font-black text-blue-400 uppercase mb-2">Digestibility Rate</p>
                  <h4 class="text-5xl font-black text-blue-900">89<small class="text-xl">%</small></h4>
                  <p class="text-[10px] font-bold text-blue-700 mt-2 tracking-widest">HIGH EFFICIENCY</p>
               </div>
               <div class="bg-green-50 p-8 rounded-[40px] border border-green-100 flex flex-col items-center text-center">
                  <p class="text-[10px] font-black text-green-400 uppercase mb-2">Immune Booster</p>
                  <h4 class="text-5xl font-black text-green-900">Active</h4>
                  <p class="text-[10px] font-bold text-green-700 mt-2 tracking-widest">VITAMIN E+ COMPLEX</p>
               </div>
            </div>
          </div>
        </div>

        <div class="p-10 bg-gray-50 border-t border-gray-100 text-center">
          <button @click="isResultModalOpen = false" class="px-20 py-5 bg-[#1a402d] text-white rounded-[30px] font-black shadow-2xl hover:scale-105 active:scale-95 transition-all uppercase tracking-[0.2em] text-xs">Acknowledge Result</button>
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
  TrendingUpIcon, SoupIcon, FileTextIcon
} from 'lucide-vue-next'

// ==========================================
// 1. DATA MODEL (CLASS LENGKAP)
// ==========================================
class FeedRecModel {
  constructor(data) {
    this.id = data.id || Math.floor(Math.random() * 90000) + 10000;
    this.tagId = data.tagId;
    this.breed = data.breed;
    this.weight = data.weight;
    this.feedName = data.feedName || "HELP-IN PRE-GROWTH FORMULA X1";
    
    // Rincian Nutrisi Lebih Dalam
    this.nutrition = {
      protein: data.protein || 18.2,
      energy: data.energy || 2.4,
      fiber: data.fiber || 12.5,
      fat: data.fat || 4.8,
      calcium: data.calcium || 1.2
    };
    
    this.ingredients = data.ingredients || ['Konsentrat Hijau', 'Ampas Tahu', 'Mineral Premix', 'Molasses'];
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

const menus = [
  { name: 'Dashboard', icon: LayoutDashboardIcon, path: '/panel_peternak/dashboard_peternak' },
  { name: 'Kandang', icon: HomeIcon, path: '/panel_peternak/kandang_peternak' },
  { name: 'Ternak', icon: DogIcon, path: '/panel_peternak/ternak' },
  { name: 'Kualitas Ternak', icon: BarChart3Icon, path: '/panel_peternak/kualitas_ternak' },
  { name: 'Kondisi Kesehatan', icon: ActivityIcon, path: '/panel_peternak/kondisi_kesehatan_ternak'  },
  { name: 'Rekomendasi Pakan', icon: SoupIcon, path: '/panel_peternak/rekomendasi_pakan_peternak' },
]

onMounted(() => {
  feedRecs.value = [
    new FeedRecModel({ tagId: 'BRH-001', breed: 'Brahman', weight: 450, protein: 17.5, energy: 2.1, fiber: 14.2, fat: 3.5, calcium: 1.0 }),
    new FeedRecModel({ tagId: 'LMS-002', breed: 'Limousin', weight: 520, protein: 19.5, energy: 2.8, fiber: 10.2, fat: 5.2, calcium: 1.5 }),
  ]
})

// ==========================================
// 3. SMART LOGIC
// ==========================================
const openInputModal = () => isInputModalOpen.value = true;
const closeInputModal = () => !isProcessing.value && (isInputModalOpen.value = false);

const runAIEngine = () => {
  if (!formData.tagId || formData.weight <= 0) return alert('Data tidak valid!');
  
  isProcessing.value = true;
  
  // Simulasi Upload & Komputasi Berdasarkan Berat (3.5 Detik)
  setTimeout(() => {
    const isHeavy = formData.weight > 500;
    
    const newRec = new FeedRecModel({
      tagId: formData.tagId,
      breed: formData.breed,
      weight: formData.weight,
      protein: isHeavy ? 20.5 : 18.0,
      energy: isHeavy ? 3.1 : 2.2,
      fiber: isHeavy ? 9.5 : 13.0,
      fat: isHeavy ? 6.0 : 4.2,
      calcium: isHeavy ? 2.0 : 1.1,
      feedName: isHeavy ? "MAX-GROWTH PRO MUSCLE FORMULA" : "STANDARD MAINTENANCE MIX",
      ingredients: formData.breed === 'Etawa' ? ['Rumput Gajah', 'Vitamins', 'Dedak'] : ['Corn Silage', 'Soybean Meal', 'Mineral', 'Urea']
    });

    feedRecs.value.unshift(newRec);
    selectedRec.value = newRec; // Set data untuk modal hasil
    
    isProcessing.value = false;
    isInputModalOpen.value = false;
    isResultModalOpen.value = true; // LANGSUNG TAMPILKAN MODAL HASIL
    
    // Reset
    formData.tagId = ''; formData.weight = 0;
  }, 3500);
}

const showFullAudit = (rec) => {
  selectedRec.value = rec;
  isResultModalOpen.value = true;
}

const getNutrientColor = (key) => {
  const colors = { protein: 'bg-blue-500', energy: 'bg-orange-500', fiber: 'bg-green-600', fat: 'bg-yellow-500', calcium: 'bg-purple-500' };
  return colors[key] || 'bg-slate-500';
}

const topStats = computed(() => [
  { label: 'Avg Protein Efficiency', value: '18.8', unit: '%' },
  { label: 'Total Formula Active', value: feedRecs.value.length, unit: 'LOGS' },
  { label: 'Daily Energy Avg', value: '2.5', unit: 'Mcal' },
]);

const filteredRecs = computed(() => feedRecs.value.filter(i => i.tagId.toLowerCase().includes(searchQuery.value.toLowerCase())))
const formatDate = (d) => new Date(d).toLocaleDateString('id-ID', { day: 'numeric', month: 'short' });

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

.animate-spin-slow { animation: spin 8s linear infinite; }
@keyframes spin { from { transform: rotate(0deg); } to { transform: rotate(360deg); } }

.animate-progress { animation: progress 3.5s ease-in-out forwards; width: 0%; }
@keyframes progress { 0% { width: 0%; } 20% { width: 30%; } 80% { width: 85%; } 100% { width: 100%; } }

.animate-in { animation: fadeIn 0.5s cubic-bezier(0.16, 1, 0.3, 1) forwards; }
@keyframes fadeIn { from { opacity: 0; transform: translateY(20px); } to { opacity: 1; transform: translateY(0); } }

.zoom-in { animation: zoomIn 0.4s cubic-bezier(0.16, 1, 0.3, 1) forwards; }
@keyframes zoomIn { from { opacity: 0; transform: scale(0.9); } to { opacity: 1; transform: scale(1); } }
</style>