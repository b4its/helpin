<template>
  <div class="flex h-screen w-full bg-[#f4f7f5] font-sans overflow-hidden relative text-slate-900">
    
      <SidebarPeternak :isOpen="isSidebarOpen" @close="isSidebarOpen = false" />

    <main class="flex-1 flex flex-col overflow-y-auto relative w-full no-scrollbar">
      <header class="flex justify-between items-center px-6 md:px-10 py-6 border-b border-gray-200 bg-white shadow-sm z-10 sticky top-0">
        <div class="flex items-center gap-4">
          <button @click="isSidebarOpen = true" class="md:hidden p-2 text-gray-600 hover:bg-gray-100 rounded-lg transition"><MenuIcon class="w-6 h-6" /></button>
          <div>
            <h1 class="text-xl md:text-2xl font-extrabold text-gray-800 leading-tight">Manajemen Kandang</h1>
            <p class="text-xs md:text-sm text-gray-500 font-medium mt-0.5">Infrastruktur dan Okupansi Batch</p>
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

      <div class="p-4 md:p-10 flex flex-col w-full max-w-[100vw]">
        
        <div class="flex flex-col md:flex-row justify-between items-start md:items-end mb-8 gap-4">
           
           <div class="flex gap-4 w-full md:w-auto">
              <div class="bg-white p-4 rounded-2xl border border-gray-100 shadow-sm text-center min-w-[120px] flex-1 md:flex-none">
                 <p class="text-[10px] font-black text-gray-400 uppercase">Kandang</p>
                 <p class="text-2xl font-black text-[#1a402d]">{{ kandangList.length }}</p>
              </div>
              <div class="bg-white p-4 rounded-2xl border border-gray-100 shadow-sm text-center min-w-[120px] flex-1 md:flex-none">
                 <p class="text-[10px] font-black text-gray-400 uppercase">Populasi</p>
                 <p class="text-2xl font-black text-orange-600">{{ totalOccupancy }}</p>
              </div>
           </div>

           <button @click="openModal('create')" class="bg-[#1a402d] text-white px-6 py-3 rounded-xl font-bold shadow-lg flex items-center justify-center gap-2 hover:bg-[#143222] transition-all active:scale-95 w-full md:w-auto shrink-0">
             <PlusIcon class="w-5 h-5" /> Tambahkan Kandang
           </button>
           
        </div>

        <section class="bg-white rounded-[40px] shadow-sm border border-gray-100 overflow-hidden mb-10">
          <div class="overflow-x-auto no-scrollbar">
            <table class="w-full text-sm text-left border-collapse min-w-[900px]">
              <thead>
                <tr class="bg-gray-50/50 text-[11px] font-black text-gray-400 uppercase tracking-[0.15em] border-b border-gray-100">
                  <th class="px-8 py-5">Identitas Kandang</th>
                  <th class="px-8 py-5">Tipe Ternak</th>
                  <th class="px-8 py-5 text-center">Kapasitas</th>
                  <th class="px-8 py-5">Hygiene</th>
                  <th class="px-8 py-5 text-center">Aksi</th>
                </tr>
              </thead>
              <tbody class="divide-y divide-gray-50">
                <tr v-for="item in kandangList" :key="item.id" class="hover:bg-gray-50 transition-colors group">
                  <td class="px-8 py-6">
                    <div class="flex flex-col font-black text-gray-800 uppercase text-base">
                      {{ item.name }} <small class="text-[10px] text-gray-400 font-bold italic tracking-widest">{{ item.id }}</small>
                    </div>
                  </td>
                  <td class="px-8 py-6">
                    <div class="flex items-center gap-3">
                      <div class="w-10 h-10 rounded-xl bg-green-50 text-green-700 flex items-center justify-center font-black">{{ item.livestockType.charAt(0) }}</div>
                      <span class="font-bold text-gray-700 uppercase">{{ item.livestockType }}</span>
                    </div>
                  </td>
                  <td class="px-8 py-6 text-center">
                    <div class="flex flex-col items-center">
                      <span class="text-lg font-black text-slate-700">{{ item.occupancy }} / {{ item.capacity }}</span>
                      <div class="h-1.5 w-24 bg-gray-100 rounded-full mt-1 overflow-hidden">
                        <div :class="['h-full rounded-full', item.occupancy >= item.capacity ? 'bg-red-500' : 'bg-[#1a402d]']" :style="`width: ${(item.occupancy / item.capacity) * 100}%`" ></div>
                      </div>
                    </div>
                  </td>
                  <td class="px-8 py-6">
                    <span :class="['px-3 py-1 rounded-full text-[10px] font-black uppercase tracking-widest', item.status === 'Kotor' ? 'bg-red-50 text-red-600' : 'bg-green-50 text-green-600']">
                      {{ item.status }}
                    </span>
                  </td>
                  <td class="px-8 py-6 text-center">
                    <div class="flex justify-center gap-2">
                      <button @click="openModal('edit', item)" class="p-2.5 bg-yellow-50 text-yellow-600 rounded-xl hover:bg-yellow-600 hover:text-white transition-all"><EditIcon class="w-4 h-4" /></button>
                      <button @click="confirmDelete(item)" class="p-2.5 bg-red-50 text-red-600 rounded-xl hover:bg-red-600 hover:text-white transition-all"><Trash2Icon class="w-4 h-4" /></button>
                    </div>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </section>
      </div>
    </main>

    <div v-if="isModalOpen" class="fixed inset-0 z-50 flex items-center justify-center p-4">
      <div class="absolute inset-0 bg-[#0c1a13]/80 backdrop-blur-md animate-in fade-in" @click="closeModal"></div>
      <div class="bg-white rounded-[40px] w-full max-w-2xl shadow-2xl relative z-10 flex flex-col animate-in slide-in-from-bottom duration-300">
        <div class="bg-gradient-to-br from-[#1a402d] to-[#2d5c41] p-10 text-white relative rounded-t-[40px]">
          <h2 class="text-4xl font-black italic tracking-tighter">{{ isEditMode ? 'Update Metadata' : 'New Infrastructure' }}</h2>
        </div>
        <div class="p-10 space-y-6">
          <div class="grid grid-cols-2 gap-6">
            <div>
              <label class="text-[10px] font-black text-gray-400 uppercase mb-2 block">Nama Kandang</label>
              <input v-model="formData.name" type="text" class="w-full px-5 py-3.5 bg-gray-50 border border-gray-100 rounded-2xl font-bold" />
            </div>
            <div>
              <label class="text-[10px] font-black text-gray-400 uppercase mb-2 block">Tipe Ternak</label>
              <select v-model="formData.livestockType" class="w-full px-5 py-3.5 bg-gray-50 border border-gray-100 rounded-2xl font-bold">
                <option value="Sapi">Sapi</option>
                <option value="Kambing">Kambing</option>
                <option value="Domba">Domba</option>
              </select>
            </div>
          </div>
          <div class="grid grid-cols-2 gap-6">
            <input v-model.number="formData.capacity" type="number" placeholder="Capacity" class="px-5 py-3.5 bg-gray-50 border border-gray-100 rounded-2xl font-bold" />
            <input v-model.number="formData.occupancy" type="number" placeholder="Occupancy" class="px-5 py-3.5 bg-gray-50 border border-gray-100 rounded-2xl font-bold" />
          </div>
          <div class="flex gap-4">
            <button @click="closeModal" class="flex-1 py-4 bg-gray-100 text-gray-500 rounded-2xl font-black uppercase text-xs">Batal</button>
            <button @click="saveData" class="flex-1 py-4 bg-[#1a402d] text-white rounded-2xl font-black shadow-xl uppercase text-xs">Simpan</button>
          </div>
        </div>
      </div>
    </div>

  </div>
</template>

<script setup>
import { ref, reactive, computed, onMounted } from 'vue'
import { LayoutDashboardIcon, HomeIcon, DogIcon, ActivityIcon, LogOutIcon, MenuIcon, XIcon, PlusIcon, EditIcon, Trash2Icon, ClipboardListIcon, SoupIcon, BarChart3Icon } from 'lucide-vue-next'

const isSidebarOpen = ref(false)

// ==========================================
// SHARED NAVIGATION LOGIC (SINKRON)
// ==========================================
const activeMenu = ref('Kandang')
const menus = [
  { name: 'Dashboard', icon: LayoutDashboardIcon, path: '/panel_peternak/dashboard_peternak' },
  { name: 'Kandang', icon: HomeIcon, path: '/panel_peternak/kandang_peternak' },
  { name: 'Ternak', icon: DogIcon, path: '/panel_peternak/ternak' },
  { name: 'Kualitas Ternak', icon: BarChart3Icon, path: '/panel_peternak/kualitas_ternak' },
  { name: 'Kondisi Kesehatan Ternak', icon: ActivityIcon, path: '/panel_peternak/kondisi_kesehatan_ternak'  },
  { name: 'Rekomendasi Pakan', icon: SoupIcon, path: '/panel_peternak/rekomendasi_pakan_ternak' },
]

// ==========================================
// CRUD DATA & STATE
// ==========================================
class KandangModel {
  constructor(data) {
    this.id = data.id || `KDG-${Math.floor(Math.random() * 900) + 100}`;
    this.name = data.name;
    this.capacity = parseInt(data.capacity);
    this.occupancy = parseInt(data.occupancy);
    this.livestockType = data.livestockType;
    this.status = data.status || 'Bersih';
  }
}

const kandangList = ref([])
const isModalOpen = ref(false)
const isEditMode = ref(false)
const editingId = ref(null)

const formData = reactive({ name: '', capacity: 0, occupancy: 0, livestockType: 'Sapi', status: 'Bersih' })

onMounted(() => {
  kandangList.value = [
    new KandangModel({ id: 'KDG-001', name: 'Kandang Brahman A1', capacity: 25, occupancy: 20, livestockType: 'Sapi' }),
    new KandangModel({ id: 'KDG-002', name: 'Kandang Domba C1', capacity: 50, occupancy: 45, livestockType: 'Domba' })
  ]
})

const totalOccupancy = computed(() => kandangList.value.reduce((acc, curr) => acc + curr.occupancy, 0))

// CRUD METHODS
const openModal = (mode, item = null) => {
  if (mode === 'edit' && item) {
    isEditMode.value = true; editingId.value = item.id;
    Object.assign(formData, { name: item.name, capacity: item.capacity, occupancy: item.occupancy, livestockType: item.livestockType, status: item.status });
  } else {
    isEditMode.value = false; editingId.value = null;
    Object.assign(formData, { name: '', capacity: 10, occupancy: 0, livestockType: 'Sapi', status: 'Bersih' });
  }
  isModalOpen.value = true;
}
const closeModal = () => isModalOpen.value = false
const saveData = () => {
  if (isEditMode.value) {
    const index = kandangList.value.findIndex(i => i.id === editingId.value);
    if (index !== -1) kandangList.value[index] = new KandangModel({ id: editingId.value, ...formData });
  } else {
    kandangList.value.unshift(new KandangModel({ ...formData }));
  }
  closeModal();
}
const confirmDelete = (item) => {
  if (confirm(`Hapus ${item.name}?`)) kandangList.value = kandangList.value.filter(i => i.id !== item.id);
}
</script>

<style scoped>
.no-scrollbar::-webkit-scrollbar { display: none; }
.no-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }
.active-menu::before { content: ""; position: absolute; right: 0; top: -24px; width: 24px; height: 24px; background-color: transparent; border-bottom-right-radius: 24px; box-shadow: 12px 12px 0 12px #f4f7f5; pointer-events: none; }
.active-menu::after { content: ""; position: absolute; right: 0; bottom: -24px; width: 24px; height: 24px; background-color: transparent; border-top-right-radius: 24px; box-shadow: 12px -12px 0 12px #f4f7f5; pointer-events: none; }
.animate-in { animation: fadeIn 0.5s ease-out forwards; }
@keyframes fadeIn { from { opacity: 0; transform: translateY(20px); } to { opacity: 1; transform: translateY(0); } }
</style>