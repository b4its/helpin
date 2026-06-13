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
      <option value="">Semua Jenis</option>
      <option v-for="j in jenisOptions" :key="j" :value="j">{{ j }}</option>
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
          <div class="bg-green-50 border border-green-100 rounded-2xl px-4 py-2.5 text-[11px] font-bold text-green-700">
            ID ternak akan dibuat otomatis oleh sistem berdasarkan jenis ternak.
          </div>
          <div class="grid grid-cols-2 gap-6">
            <div>
              <label class="text-[10px] font-black text-gray-400 uppercase mb-2 block">Jenis Ternak</label>
              <select v-model="formData.category" class="w-full px-5 py-3.5 bg-gray-50 border border-gray-100 rounded-2xl font-bold outline-none focus:ring-4 focus:ring-green-500/10">
                <option v-for="j in jenisOptions" :key="j" :value="j">{{ j }}</option>
              </select>
            </div>
            <div>
              <label class="text-[10px] font-black text-gray-400 uppercase mb-2 block">Ras Ternak</label>
              <input v-model="formData.breed" type="text" placeholder="Cth: Brahman / Etawa / Broiler" class="w-full px-5 py-3.5 bg-gray-50 border border-gray-100 rounded-2xl font-bold" />
            </div>
          </div>
          <div class="grid grid-cols-2 gap-6">
            <div>
              <label class="text-[10px] font-black text-gray-400 uppercase mb-2 block">Usia (bulan)</label>
              <input v-model.number="formData.ageMonths" type="number" min="0" placeholder="Cth: 12" class="w-full px-5 py-3.5 bg-gray-50 border border-gray-100 rounded-2xl font-bold" />
            </div>
            <div>
              <label class="text-[10px] font-black text-gray-400 uppercase mb-2 block">Berat (kg) — opsional</label>
              <input v-model.number="formData.weight" type="number" min="0" placeholder="Cth: 45" class="w-full px-5 py-3.5 bg-gray-50 border border-gray-100 rounded-2xl font-bold" />
            </div>
          </div>
          <div>
            <label class="text-[10px] font-black text-gray-400 uppercase mb-2 block">Kandang (opsional)</label>
            <select v-model="formData.penId" class="w-full px-5 py-3.5 bg-gray-50 border border-gray-100 rounded-2xl font-bold">
              <option value="">— Belum ditempatkan —</option>
              <option v-for="p in penOptions" :key="p.id" :value="p.id">{{ p.name }} ({{ p.pen_type }})</option>
            </select>
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
// 2. API COMPOSABLE
// ==========================================
const { list: fetchLivestock, create: createLivestock, update: updateLivestock, remove: removeLivestock } = useLivestock()
const { list: fetchPens } = usePen()
const penOptions = ref([])

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
const loading = ref(false)

const formData = reactive({ category: 'Mamalia', breed: '', weight: null, ageMonths: null, penId: '' })
const jenisOptions = ['Unggas', 'Mamalia', 'Ruminansia', 'Serangga', 'Aves', 'Ikan', 'Reptil', 'Lainnya']

// Helper to normalize API response to UI model
const mapLivestockItem = (item) => ({
  id: item.id,
  tagId: item.tag_id || item.tagId,
  category: item.category,
  breed: item.breed,
  weight: item.weight != null ? parseFloat(item.weight) : 0,
  ageMonths: item.age_months ?? item.ageMonths ?? null,
  penId: item.pen_id ?? item.penId ?? null,
  healthStatus: item.health_status || item.healthStatus || 'Sehat',
  healthScore: item.health_score || item.healthScore || 95,
  entryDate: item.entry_date || item.entryDate || new Date().toISOString(),
  gender: item.gender || 'Jantan',
  biometrics: item.biometrics || [
    { label: 'Avg Heart Rate', value: '72', unit: 'bpm' },
    { label: 'Body Temp', value: '38.4', unit: '°C' },
    { label: 'Respiratory', value: '18', unit: 'bpm' },
    { label: 'Movement', value: '3.2', unit: 'km/d' }
  ]
})

onMounted(async () => {
  loading.value = true
  try {
    const data = await fetchLivestock()
    livestockList.value = (data || []).map(mapLivestockItem)
    try { penOptions.value = (await fetchPens()) || [] } catch { penOptions.value = [] }
  } catch (e) {
    console.error('Failed to load livestock:', e)
    livestockList.value = []
  } finally {
    loading.value = false
  }
})

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

const quickStats = computed(() => {
  const total = livestockList.value.length
  const avgWeight = total > 0 ? Math.round(livestockList.value.reduce((s, i) => s + i.weight, 0) / total) : 0
  const sickCount = livestockList.value.filter(i => i.healthStatus === 'Sakit').length
  const healthyPercent = total > 0 ? Math.round(((total - sickCount) / total) * 100) : 0
  return [
    { label: 'Populasi Aktif', value: total, status: 'Stabil', alert: false },
    { label: 'Rerata Berat', value: `${avgWeight}kg`, status: '+2%', alert: false },
    { label: 'Health Index', value: `${healthyPercent}%`, status: 'Optimal', alert: false },
    { label: 'Sakit/Waspada', value: `${sickCount} Ekor`, status: sickCount > 0 ? 'Alert' : 'OK', alert: sickCount > 0 },
  ]
});

const bioBasic = computed(() => {
  if (!selectedTernak.value) return {};
  return {
    'DNA / Breed': selectedTernak.value.breed,
    'Kategori': selectedTernak.value.category,
    'Jenis Kelamin': selectedTernak.value.gender,
    'Usia': selectedTernak.value.ageMonths ? selectedTernak.value.ageMonths + ' bulan' : '-',
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
    Object.assign(formData, { category: item.category, breed: item.breed, weight: item.weight, ageMonths: item.ageMonths, penId: item.penId || '' });
  } else {
    isEditMode.value = false;
    Object.assign(formData, { category: 'Mamalia', breed: '', weight: null, ageMonths: null, penId: '' });
  }
  isModalOpen.value = true;
}

const viewFullPassport = (item) => {
  selectedTernak.value = item;
  isPassportOpen.value = true;
}

const closeModal = () => isModalOpen.value = false;

const saveData = async () => {
  if (!formData.category || !formData.breed) {
    useToast().warning('Data belum lengkap', 'Lengkapi jenis ternak dan ras')
    return
  }
  loading.value = true
  try {
    const payload = {
      category: formData.category,
      breed: formData.breed,
      weight: formData.weight ? Number(formData.weight) : null,
      age_months: formData.ageMonths ? Number(formData.ageMonths) : null,
      pen_id: formData.penId || null
    }

    if (isEditMode.value) {
      const updated = await updateLivestock(editingId.value, payload)
      const idx = livestockList.value.findIndex(i => i.id === editingId.value)
      if (idx !== -1) {
        livestockList.value[idx] = mapLivestockItem(updated || { ...payload, id: editingId.value })
      }
    } else {
      const created = await createLivestock(payload)
      livestockList.value.unshift(mapLivestockItem(created || { ...payload, id: crypto.randomUUID() }))
    }
    closeModal()
  } catch (e) {
    console.error('Failed to save livestock:', e)
    useToast().error('Gagal menyimpan data ternak', e?.data?.error?.message || e?.data?.message)
  } finally {
    loading.value = false
  }
}

const confirmDelete = async (item) => {
  if (confirm(`Hapus data ternak ${item.tagId}?`)) {
    loading.value = true
    try {
      await removeLivestock(item.id)
      livestockList.value = livestockList.value.filter(i => i.id !== item.id)
    } catch (e) {
      console.error('Failed to delete livestock:', e)
      useToast().error('Gagal menghapus data ternak', e?.data?.message)
    } finally {
      loading.value = false
    }
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