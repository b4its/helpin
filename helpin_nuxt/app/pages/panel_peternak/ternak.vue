<template>
  <div class="flex h-screen w-full bg-[#f4f7f5] font-sans overflow-hidden relative text-slate-900">
    
  <SidebarPeternak :isOpen="isSidebarOpen" @close="isSidebarOpen = false" />

    <main class="flex-1 flex flex-col overflow-y-auto relative w-full no-scrollbar">
      <header class="flex justify-between items-center px-6 md:px-10 py-6 border-b border-gray-200 bg-white/80 backdrop-blur-md z-10 sticky top-0">
        <div class="flex items-center gap-4">
          <button @click="isSidebarOpen = true" class="md:hidden p-2 text-gray-600 border border-gray-100 rounded-lg transition"><MenuIcon class="w-6 h-6" /></button>
          <div>
            <h1 class="text-xl md:text-2xl font-black text-gray-800 leading-tight">Manajemen Individu Ternak</h1>
            <p class="text-xs md:text-sm text-gray-500 font-medium mt-0.5 italic">Audit biometrik dan histori nutrisi per individu</p>
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
  


  <section class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
    <div v-for="stat in quickStats" :key="stat.label" class="bg-white p-6 rounded-3xl border border-gray-100 shadow-sm">
      <p class="text-[10px] font-black text-gray-400 uppercase tracking-[0.2em] mb-2">{{ stat.label }}</p>
      <div class="flex items-end justify-between">
        <h3 class="text-3xl font-black text-[#1a402d]">{{ stat.value }}</h3>
        <span :class="['text-[10px] font-black px-2 py-1 rounded-lg', stat.alert ? 'bg-red-50 text-red-600' : 'bg-green-50 text-green-600']">
          {{ stat.status }}
        </span>
      </div>
    </div>
  </section>
  <div class="flex justify-end w-full">
    <button @click="openModal('create')" class="bg-[#1a402d] text-white px-6 py-3 rounded-2xl font-black shadow-xl shadow-green-900/20 flex items-center gap-2 hover:scale-105 transition-all active:scale-95">
      <PlusIcon class="w-5 h-5" /> Tambahkan Ternak
    </button>
  </div>
  <section class="flex flex-col md:flex-row gap-4">
    <div class="relative flex-1 w-full">
      <SearchIcon class="absolute left-5 top-1/2 -translate-y-1/2 w-5 h-5 text-gray-400" />
      <input v-model="searchQuery" type="text" placeholder="Cari berdasarkan Tag ID, Ras, atau Nama Ternak..." class="w-full pl-14 pr-6 py-4 bg-white border border-gray-100 rounded-2xl outline-none focus:ring-4 focus:ring-green-500/10 transition-all font-bold text-sm" />
    </div>
    <select v-model="filterCategory" class="px-6 py-4 bg-white border border-gray-100 rounded-2xl font-bold text-sm outline-none focus:ring-4 focus:ring-green-500/10">
      <option value="">Semua Kategori</option>
      <option value="Sapi">Sapi</option>
      <option value="Kambing">Kambing</option>
      <option value="Domba">Domba</option>
    </select>
  </section>

  <section class="bg-white rounded-[40px] shadow-sm border border-gray-100 overflow-hidden mb-10">
    <div class="overflow-x-auto no-scrollbar">
      <table class="w-full text-left border-collapse min-w-[1000px]">
        <thead>
          <tr class="bg-gray-50/50 text-[11px] font-black text-gray-400 uppercase tracking-[0.2em] border-b border-gray-100">
            <th class="px-8 py-6">ID & DNA Meta</th>
            <th class="px-8 py-6">Kategori & Ras</th>
            <th class="px-8 py-6 text-center">Body Weight</th>
            <th class="px-8 py-6">Health Score</th>
            <th class="px-8 py-6 text-center">Audit</th>
          </tr>
        </thead>
        <tbody class="divide-y divide-gray-50">
          <tr v-if="filteredLivestock.length === 0">
            <td colspan="5" class="py-20 text-center font-bold text-gray-300">Data ternak tidak ditemukan...</td>
          </tr>
          <tr v-for="item in filteredLivestock" :key="item.id" class="hover:bg-green-50/30 transition-all group">
            <td class="px-8 py-6">
              <div class="flex flex-col">
                <span class="font-black text-gray-800 text-lg group-hover:text-green-800 transition uppercase tracking-tighter">{{ item.tagId }}</span>
                <span class="text-[10px] text-gray-400 font-bold uppercase tracking-widest">{{ formatDate(item.entryDate) }} • Reg ID: {{ item.id }}</span>
              </div>
            </td>
            <td class="px-8 py-6">
              <div class="flex items-center gap-4">
                <div class="w-12 h-12 rounded-2xl bg-white border border-gray-100 flex items-center justify-center text-[#1a402d] font-black shadow-sm uppercase">{{ item.category.charAt(0) }}</div>
                <div class="flex flex-col">
                  <span class="font-black text-gray-700 text-base uppercase">{{ item.category }}</span>
                  <span class="text-xs text-gray-400 font-medium italic">{{ item.breed }}</span>
                </div>
              </div>
            </td>
            <td class="px-8 py-6 text-center">
              <div class="flex flex-col">
                <span class="text-2xl font-black text-slate-800">{{ item.weight }} <small class="text-xs italic font-bold">KG</small></span>
                <span class="text-[10px] font-black text-blue-500 uppercase">Growth +2.4%</span>
              </div>
            </td>
            <td class="px-8 py-6">
              <div class="flex flex-col gap-2">
                 <span :class="['w-fit px-3 py-1 rounded-full text-[10px] font-black uppercase tracking-widest', item.healthStatus === 'Sakit' ? 'bg-red-50 text-red-600' : 'bg-green-50 text-green-600']">
                  {{ item.healthStatus }}
                </span>
                <div class="h-1.5 w-24 bg-gray-100 rounded-full overflow-hidden">
                  <div :class="['h-full rounded-full', item.healthStatus === 'Sakit' ? 'bg-red-500' : 'bg-green-500']" :style="`width: ${item.healthScore}%`" ></div>
                </div>
              </div>
            </td>
            <td class="px-8 py-6 text-center">
              <div class="flex justify-center gap-2">
                <button @click="viewFullPassport(item)" class="p-3 bg-gray-50 text-gray-400 rounded-xl hover:bg-[#1a402d] hover:text-white hover:rotate-12 transition-all shadow-sm">
                  <EyeIcon class="w-5 h-5" />
                </button>
                <button @click="confirmDelete(item)" class="p-3 bg-red-50 text-red-400 rounded-xl hover:bg-red-600 hover:text-white transition-all shadow-sm">
                  <Trash2Icon class="w-5 h-5" />
                </button>
              </div>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </section>
</div>
    </main>

    <div v-if="isPassportOpen" class="fixed inset-0 z-50 flex items-center justify-center p-4">
      <div class="absolute inset-0 bg-[#0c1a13]/90 backdrop-blur-xl animate-in fade-in" @click="isPassportOpen = false"></div>
      
      <div class="bg-white rounded-[50px] w-full max-w-5xl max-h-[92vh] overflow-hidden shadow-2xl relative z-10 flex flex-col animate-in slide-in-from-bottom-10 duration-500 no-scrollbar">
        
        <div class="bg-gradient-to-br from-[#1a402d] via-[#1a402d] to-[#2d5c41] p-12 text-white relative">
          <button @click="isPassportOpen = false" class="absolute top-10 right-10 p-3 bg-white/10 hover:bg-white/20 rounded-full transition-all"><XIcon class="w-6 h-6" /></button>
          <div class="flex items-center gap-3 mb-4">
            <span class="px-3 py-1 bg-green-400 text-[#1a402d] text-[10px] font-black rounded-full uppercase tracking-widest">Biological DNA Audit</span>
            <span class="text-white/40 font-mono text-sm">TAG_ID: {{ selectedTernak?.tagId }}</span>
          </div>
          <h2 class="text-5xl font-black tracking-tighter mb-2 italic">Livestock Passport</h2>
          <p class="text-lg text-white/60 font-medium italic">Sistem Pakar HELP-IN: Digital Twin Management</p>
        </div>

        <div class="p-12 overflow-y-auto no-scrollbar grid grid-cols-1 md:grid-cols-3 gap-12 bg-white">
          
          <div class="space-y-8">
            <h3 class="text-xs font-black text-green-700 uppercase tracking-[0.2em] flex items-center gap-2">
              <InfoIcon class="w-4 h-4" /> Genetic & Origin
            </h3>
            <div class="space-y-6 bg-gray-50 p-8 rounded-[40px] border border-gray-100 shadow-inner">
               <div v-for="(val, label) in bioBasic" :key="label">
                  <p class="text-[10px] font-black text-gray-400 uppercase mb-1">{{ label }}</p>
                  <p class="text-lg font-black text-gray-800">{{ val }}</p>
               </div>
            </div>
          </div>

          <div class="space-y-8">
            <h3 class="text-xs font-black text-blue-700 uppercase tracking-[0.2em] flex items-center gap-2">
              <ActivityIcon class="w-4 h-4" /> Biometric Sensors
            </h3>
            <div class="grid grid-cols-1 gap-4">
              <div v-for="bio in selectedTernak?.biometrics" :key="bio.label" class="bg-blue-50/40 p-6 rounded-[32px] border border-blue-100 flex justify-between items-center">
                <div>
                   <p class="text-[10px] font-black text-blue-400 uppercase mb-1">{{ bio.label }}</p>
                   <p class="text-2xl font-black text-blue-900">{{ bio.value }}<small class="text-xs ml-0.5">{{ bio.unit }}</small></p>
                </div>
                <div class="h-10 w-10 bg-white rounded-full flex items-center justify-center shadow-sm">
                   <TrendingUpIcon class="w-4 h-4 text-blue-500" />
                </div>
              </div>
            </div>
          </div>

          <div class="space-y-8">
            <h3 class="text-xs font-black text-orange-700 uppercase tracking-[0.2em] flex items-center gap-2">
              <SoupIcon class="w-4 h-4" /> Nutrition & Growth
            </h3>
            <div class="bg-orange-50/30 rounded-[40px] p-8 border border-orange-100 shadow-inner relative overflow-hidden">
               <p class="text-[10px] font-black text-orange-400 uppercase mb-2 tracking-widest">Growth Prediction</p>
               <p class="text-5xl font-black text-orange-900 mb-6">+42.5 <small class="text-sm italic">kg/month</small></p>
               
               <div class="pt-6 border-t border-orange-200">
                  <p class="text-[10px] font-black text-gray-500 uppercase mb-4 text-center tracking-widest">Intelligence Advisor</p>
                  <div class="bg-white p-5 rounded-2xl text-xs font-bold leading-relaxed text-slate-600 border border-gray-100 shadow-sm italic">
                    "Deteksi peningkatan metabolisme. Berdasarkan siklus ini, naikkan porsi serat kasar 15% untuk menjaga stabilitas pH rumen ternak."
                  </div>
               </div>
            </div>
          </div>

        </div>

        <div class="p-8 bg-gray-50 border-t border-gray-100 flex justify-between items-center">
          <div class="flex gap-4">
             <button @click="isPassportOpen = false" class="px-10 py-5 bg-white border border-gray-200 text-gray-500 rounded-3xl font-black shadow-sm uppercase text-xs tracking-widest hover:bg-gray-50 transition-all">Tutup Passport</button>
             <button class="px-10 py-5 bg-[#1a402d] text-white rounded-3xl font-black shadow-xl shadow-green-900/40 hover:scale-105 active:scale-95 transition-all uppercase text-xs tracking-widest">Cetak Bio Audit</button>
          </div>
          <p class="text-xs text-gray-400 font-bold italic">Official HELP-IN Breeder Documentation © 2026</p>
        </div>
      </div>
    </div>

    <div v-if="isModalOpen" class="fixed inset-0 z-50 flex items-center justify-center p-4">
      <div class="absolute inset-0 bg-[#0c1a13]/80 backdrop-blur-md animate-in fade-in" @click="closeModal"></div>
      <div class="bg-white rounded-[40px] w-full max-w-2xl shadow-2xl relative z-10 flex flex-col animate-in slide-in-from-bottom duration-300">
        <div class="bg-[#1a402d] p-10 text-white relative rounded-t-[40px]">
          <h2 class="text-4xl font-black italic tracking-tighter">{{ isEditMode ? 'Update Biological Meta' : 'New Livestock Registration' }}</h2>
        </div>
        <div class="p-10 space-y-6">
          <div class="grid grid-cols-2 gap-6">
            <input v-model="formData.tagId" type="text" placeholder="Tag ID (E.g: BRH-001)" class="w-full px-5 py-3.5 bg-gray-50 border border-gray-100 rounded-2xl font-bold outline-none focus:ring-4 focus:ring-green-500/10" />
            <select v-model="formData.category" class="w-full px-5 py-3.5 bg-gray-50 border border-gray-100 rounded-2xl font-bold outline-none focus:ring-4 focus:ring-green-500/10">
              <option value="Sapi">Sapi</option>
              <option value="Kambing">Kambing</option>
              <option value="Domba">Domba</option>
            </select>
          </div>
          <div class="grid grid-cols-2 gap-6">
            <input v-model="formData.breed" type="text" placeholder="Breed / Ras" class="px-5 py-3.5 bg-gray-50 border border-gray-100 rounded-2xl font-bold" />
            <input v-model.number="formData.weight" type="number" placeholder="Weight (kg)" class="px-5 py-3.5 bg-gray-50 border border-gray-100 rounded-2xl font-bold" />
          </div>
          <div class="flex gap-4 pt-6 border-t border-gray-100">
            <button @click="closeModal" class="flex-1 py-4 bg-gray-100 text-gray-500 rounded-2xl font-black uppercase text-xs">Batal</button>
            <button @click="saveData" class="flex-1 py-4 bg-[#1a402d] text-white rounded-2xl font-black shadow-xl uppercase text-xs">Simpan Data</button>
          </div>
        </div>
      </div>
    </div>

  </div>
</template>

<script setup>
import { ref, reactive, computed, onMounted } from 'vue'
import { 
  LayoutDashboardIcon, HomeIcon, DogIcon, ActivityIcon, ClipboardListIcon, BarChart3Icon,
  LogOutIcon, MenuIcon, XIcon, PlusIcon, EditIcon, 
  Trash2Icon, SearchIcon, EyeIcon, TrendingUpIcon, SoupIcon, InfoIcon
} from 'lucide-vue-next'

// ==========================================
// 1. DYNAMIC SHARED NAVIGATION ( synced )
// ==========================================
const isSidebarOpen = ref(false)
const activeMenu = ref('Ternak')
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
class LivestockModel {
  constructor(data) {
    this.id = data.id || `REG-${Math.floor(Math.random() * 9000) + 1000}`;
    this.tagId = data.tagId;
    this.category = data.category;
    this.breed = data.breed;
    this.weight = parseFloat(data.weight);
    this.healthStatus = data.healthStatus || 'Sehat';
    this.healthScore = data.healthScore || 95;
    this.entryDate = data.entryDate || new Date().toISOString();
    this.gender = data.gender || 'Jantan';
    
    // Intelligence & Sensor Meta
    this.biometrics = data.biometrics || [
      { label: 'Avg Heart Rate', value: '72', unit: 'bpm' },
      { label: 'Body Temp', value: '38.4', unit: '°C' },
      { label: 'Respiratory', value: '18', unit: 'bpm' },
      { label: 'Movement', value: '3.2', unit: 'km/d' }
    ];
  }

  static fromJSON(json) {
    return new LivestockModel({
      id: json.kode_reg,
      tagId: json.id_tag,
      category: json.tipe_hewan,
      breed: json.jenis_ras,
      weight: json.berat_kg,
      healthStatus: json.status_medis,
      healthScore: json.skor_vitalitas,
      entryDate: json.tgl_masuk
    });
  }
}

// ==========================================
// 3. REACTIVE STATE
// ==========================================
const livestockList = ref([])
const searchQuery = ref('')
const filterCategory = ref('')
const isModalOpen = ref(false)
const isEditMode = ref(false)
const isPassportOpen = ref(false)
const selectedTernak = ref(null)
const editingId = ref(null)

const formData = reactive({ tagId: '', category: 'Sapi', breed: '', weight: 0 })

onMounted(() => {
  // Dummy Data JSON
  const dummyData = [
    { kode_reg: '2026001', id_tag: 'BRH-001', tipe_hewan: 'Sapi', jenis_ras: 'Brahman', berat_kg: 450.5, status_medis: 'Sehat', skor_vitalitas: 98, tgl_masuk: '2026-01-10' },
    { kode_reg: '2026002', id_tag: 'LMS-002', tipe_hewan: 'Sapi', jenis_ras: 'Limousin', berat_kg: 520.0, status_medis: 'Observasi', skor_vitalitas: 75, tgl_masuk: '2026-02-15' },
    { kode_reg: '2026003', id_tag: 'ETW-045', tipe_hewan: 'Kambing', jenis_ras: 'Etawa', berat_kg: 45.2, status_medis: 'Sakit', skor_vitalitas: 40, tgl_masuk: '2026-03-20' },
  ];
  livestockList.value = dummyData.map(d => LivestockModel.fromJSON(d));
});

// ==========================================
// 4. COMPUTED & LOGIC
// ==========================================
const filteredLivestock = computed(() => {
  return livestockList.value.filter(i => {
    const matchSearch = i.tagId.toLowerCase().includes(searchQuery.value.toLowerCase()) || i.breed.toLowerCase().includes(searchQuery.value.toLowerCase());
    const matchCat = filterCategory.value ? i.category === filterCategory.value : true;
    return matchSearch && matchCat;
  });
});

const quickStats = computed(() => [
  { label: 'Populasi Aktif', value: livestockList.value.length, status: 'Stabil', alert: false },
  { label: 'Rerata Berat', value: '338kg', status: '+2%', alert: false },
  { label: 'Health Index', value: '92%', status: 'Optimal', alert: false },
  { label: 'Sakit/Waspada', value: '1 Ekor', status: 'Alert', alert: true },
]);

const bioBasic = computed(() => {
  if (!selectedTernak.value) return {};
  return {
    'DNA / Breed': selectedTernak.value.breed,
    'Kategori': selectedTernak.value.category,
    'Jenis Kelamin': selectedTernak.value.gender,
    'Body Weight': selectedTernak.value.weight + ' KG',
    'Tanggal Masuk': formatDate(selectedTernak.value.entryDate)
  }
});

// ==========================================
// 5. CRUD METHODS
// ==========================================
const openModal = (mode, item = null) => {
  if (mode === 'edit' && item) {
    isEditMode.value = true; editingId.value = item.id;
    Object.assign(formData, { tagId: item.tagId, category: item.category, breed: item.breed, weight: item.weight });
  } else {
    isEditMode.value = false;
    Object.assign(formData, { tagId: '', category: 'Sapi', breed: '', weight: 0 });
  }
  isModalOpen.value = true;
}

const viewFullPassport = (item) => {
  selectedTernak.value = item;
  isPassportOpen.value = true;
}

const closeModal = () => isModalOpen.value = false;

const saveData = () => {
  if (isEditMode.value) {
    const idx = livestockList.value.findIndex(i => i.id === editingId.value);
    livestockList.value[idx] = new LivestockModel({ id: editingId.value, ...formData });
  } else {
    livestockList.value.unshift(new LivestockModel({ ...formData }));
  }
  closeModal();
}

const confirmDelete = (item) => {
  if (confirm(`Hapus data ternak ${item.tagId}?`)) {
    livestockList.value = livestockList.value.filter(i => i.id !== item.id);
  }
}

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
</style>