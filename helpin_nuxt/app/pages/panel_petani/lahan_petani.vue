<template>
  <div class="flex h-screen w-full bg-[#f4f7f5] font-sans overflow-hidden relative">
    
<SidebarPetani :isOpen="isSidebarOpen" @close="isSidebarOpen = false" />

    <main class="flex-1 flex flex-col overflow-y-auto relative w-full">
      <header class="flex justify-between items-center px-6 md:px-10 py-4 md:py-6 border-b border-gray-200 bg-white shadow-sm z-10 sticky top-0">
        <div class="flex items-center gap-4">
          <button @click="isSidebarOpen = true" class="md:hidden p-2 -ml-2 text-gray-600 hover:bg-gray-100 rounded-lg transition">
            <MenuIcon class="w-6 h-6" />
          </button>
          <div>
            <h1 class="text-xl md:text-2xl font-extrabold text-gray-800 leading-tight">Manajemen Lahan</h1>
            <p class="text-xs md:text-sm text-gray-500 font-medium mt-0.5">Pemetaan area dan status operasional tanah</p>
          </div>
        </div>
        <div>
          <div class="flex items-center gap-2 bg-green-100 px-3 py-1.5 md:px-4 md:py-2 rounded-lg border border-green-200">
            <div class="w-2 h-2 rounded-full bg-green-600 shrink-0 animate-pulse"></div>
            <span class="text-xs md:text-sm font-bold text-green-600 hidden sm:block">STATUS ONLINE</span>
          </div>
        </div>
      </header>

      <div class="p-4 md:p-10 flex flex-col w-full max-w-[100vw]">
        
        <section class="bg-white rounded-2xl shadow-sm border border-gray-100 p-4 md:p-6 w-full flex flex-col flex-1">
          <div class="flex flex-col sm:flex-row justify-between items-start sm:items-center mb-6 gap-4">
            <div class="flex items-center gap-2">
              <div class="w-2 h-2 rounded-full bg-[#1a402d]"></div>
              <h2 class="text-base md:text-lg font-bold text-gray-800">Daftar Lahan</h2>
            </div>
            
            <button @click="openModal('create')" class="w-full sm:w-auto px-4 py-2.5 font-bold text-white bg-[#1a402d] hover:bg-[#143222] rounded-lg transition shadow-md flex items-center justify-center gap-2">
              <PlusIcon class="w-5 h-5" /> Tambahkan Lahan
            </button>
          </div>
          
          <div class="overflow-x-auto -mx-4 md:mx-0 px-4 md:px-0">
            <table class="w-full text-sm text-left min-w-[800px]">
              <thead class="text-xs text-green-800 uppercase bg-[#e1f0e5] font-bold">
                <tr>
                  <th scope="col" class="px-4 md:px-6 py-4 rounded-l-lg">ID Lahan</th>
                  <th scope="col" class="px-4 md:px-6 py-4">Nama / Blok</th>
                  <th scope="col" class="px-4 md:px-6 py-4">Luas Area</th>
                  <th scope="col" class="px-4 md:px-6 py-4">Jenis Tanah</th>
                  <th scope="col" class="px-4 md:px-6 py-4">Status</th>
                  <th scope="col" class="px-4 md:px-6 py-4 rounded-r-lg text-center">Aksi</th>
                </tr>
              </thead>
              <tbody>
                <tr v-if="lahanList.length === 0">
                  <td colspan="6" class="text-center py-10 text-gray-500 font-medium">Belum ada data lahan.</td>
                </tr>
                <tr v-for="item in lahanList" :key="item.id" class="border-b border-gray-50 last:border-0 hover:bg-gray-50 transition-colors">
                  <td class="px-4 md:px-6 py-4 font-bold text-gray-500">{{ item.id }}</td>
                  <td class="px-4 md:px-6 py-4 font-bold text-gray-800">
                    {{ item.name }}
                    <span class="block text-xs font-medium text-gray-400 mt-0.5"><MapPinIcon class="w-3 h-3 inline mr-1" />{{ item.location }}</span>
                  </td>
                  <td class="px-4 md:px-6 py-4 font-bold text-gray-800">
                    {{ item.area }} <span class="text-xs text-gray-500 font-medium">Hektar</span>
                  </td>
                  <td class="px-4 md:px-6 py-4 font-semibold text-gray-600">{{ item.soilType }}</td>
                  <td class="px-4 md:px-6 py-4">
                    <span class="px-2.5 py-1 rounded-full text-xs font-bold" 
                      :class="{
                        'bg-green-100 text-green-700': item.status === 'Aktif Ditanami',
                        'bg-yellow-100 text-yellow-700': item.status === 'Persiapan',
                        'bg-gray-100 text-gray-600': item.status === 'Masa Bera / Istirahat'
                      }">
                      {{ item.status }}
                    </span>
                  </td>
                  <td class="px-4 md:px-6 py-4 flex justify-center gap-2">
                    <button @click="openModal('edit', item)" class="bg-[#eab308] hover:bg-yellow-600 text-white p-2 rounded-md transition shadow-sm" title="Edit">
                      <EditIcon class="w-4 h-4" />
                    </button>
                    <button @click="confirmDelete(item)" class="bg-[#ef4444] hover:bg-red-600 text-white p-2 rounded-md transition shadow-sm" title="Hapus">
                      <Trash2Icon class="w-4 h-4" />
                    </button>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </section>
      </div>
    </main>

    <div v-if="isFormModalOpen" class="fixed inset-0 z-50 flex items-center justify-center p-2 md:p-4">
      <div class="absolute inset-0 bg-black/60 backdrop-blur-sm" @click="closeModal"></div>
      
      <div class="bg-white rounded-2xl w-full max-w-2xl shadow-2xl relative z-10 flex flex-col max-h-[95vh] animate-in fade-in zoom-in duration-200">
        <div class="flex justify-between items-center p-4 md:p-6 border-b border-gray-100 shrink-0">
          <div class="flex items-center gap-3">
            <div class="p-2 bg-green-100 text-green-700 rounded-lg">
              <MapIcon class="w-6 h-6" />
            </div>
            <h2 class="text-lg md:text-xl font-extrabold text-gray-800">
              {{ isEditMode ? 'Edit Data Lahan' : 'Tambah Lahan Baru' }}
            </h2>
          </div>
          <button @click="closeModal" class="p-2 text-gray-400 hover:bg-gray-100 hover:text-gray-700 rounded-lg transition shrink-0">
            <XIcon class="w-5 h-5 md:w-6 md:h-6" />
          </button>
        </div>

        <div class="p-4 md:p-6 overflow-y-auto">
          <form @submit.prevent="saveData" class="space-y-5">
            <div class="grid grid-cols-1 md:grid-cols-2 gap-5">
              <div>
                <label class="block mb-1.5 text-sm font-bold text-gray-700">Nama / Blok Lahan</label>
                <input v-model="formData.name" type="text" required class="bg-gray-50 border border-gray-200 text-gray-900 text-sm rounded-lg focus:ring-[#1a402d] focus:border-[#1a402d] block w-full p-2.5 outline-none transition" placeholder="Cth: Blok A - Utara" />
              </div>
              
              <div>
                <label class="block mb-1.5 text-sm font-bold text-gray-700">Luas Area (Hektar)</label>
                <input v-model.number="formData.area" type="number" step="0.01" min="0" required class="bg-gray-50 border border-gray-200 text-gray-900 text-sm rounded-lg focus:ring-[#1a402d] focus:border-[#1a402d] block w-full p-2.5 outline-none transition" placeholder="Cth: 2.5" />
              </div>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-5">
              <div>
                <label class="block mb-1.5 text-sm font-bold text-gray-700">Jenis Tanah</label>
                <select v-model="formData.soilType" required class="bg-gray-50 border border-gray-200 text-gray-900 text-sm rounded-lg focus:ring-[#1a402d] focus:border-[#1a402d] block w-full p-2.5 outline-none transition">
                  <option value="" disabled>Pilih Jenis Tanah</option>
                  <option value="Tanah Lempung">Tanah Lempung</option>
                  <option value="Tanah Pasir">Tanah Pasir</option>
                  <option value="Tanah Gambut">Tanah Gambut</option>
                  <option value="Tanah Humus">Tanah Humus</option>
                </select>
              </div>
              
              <div>
                <label class="block mb-1.5 text-sm font-bold text-gray-700">Status Operasional</label>
                <select v-model="formData.status" required class="bg-gray-50 border border-gray-200 text-gray-900 text-sm rounded-lg focus:ring-[#1a402d] focus:border-[#1a402d] block w-full p-2.5 outline-none transition">
                  <option value="" disabled>Pilih Status</option>
                  <option value="Aktif Ditanami">Aktif Ditanami</option>
                  <option value="Persiapan">Persiapan (Pembajakan/Pemupukan Dasar)</option>
                  <option value="Masa Bera / Istirahat">Masa Bera / Istirahat</option>
                </select>
              </div>
            </div>

            <div>
              <label class="block mb-1.5 text-sm font-bold text-gray-700">Detail Lokasi (Opsional)</label>
              <textarea v-model="formData.location" rows="2" class="bg-gray-50 border border-gray-200 text-gray-900 text-sm rounded-lg focus:ring-[#1a402d] focus:border-[#1a402d] block w-full p-2.5 outline-none transition resize-none" placeholder="Cth: Kordinat lintang bujur atau patokan lokasi spesifik"></textarea>
            </div>
          </form>
        </div>

        <div class="p-4 md:p-6 border-t border-gray-100 flex justify-end gap-3 bg-gray-50 rounded-b-2xl shrink-0">
          <button @click="closeModal" type="button" class="px-5 py-2.5 font-bold text-gray-600 hover:bg-gray-200 rounded-lg transition text-center">
            Batal
          </button>
          <button @click="saveData" type="button" class="px-5 py-2.5 font-bold text-white bg-[#1a402d] hover:bg-[#143222] rounded-lg transition shadow-md flex items-center justify-center gap-2">
            Simpan Data
          </button>
        </div>
      </div>
    </div>

    <div v-if="isDeleteModalOpen" class="fixed inset-0 z-50 flex items-center justify-center p-4">
      <div class="absolute inset-0 bg-black/60 backdrop-blur-sm" @click="closeDeleteModal"></div>
      
      <div class="bg-white rounded-2xl w-full max-w-sm shadow-2xl relative z-10 flex flex-col animate-in fade-in zoom-in duration-200 p-6 text-center">
        <div class="w-16 h-16 bg-red-100 rounded-full flex items-center justify-center mx-auto mb-4 text-red-500">
          <AlertTriangleIcon class="w-8 h-8" />
        </div>
        <h2 class="text-xl font-extrabold text-gray-800 mb-2">Hapus Lahan?</h2>
        <p class="text-sm text-gray-500 mb-6">Anda yakin ingin menghapus <strong>{{ itemToDelete?.name }}</strong>? Data lahan yang dihapus tidak dapat dikembalikan.</p>
        
        <div class="flex gap-3 justify-center">
          <button @click="closeDeleteModal" class="px-5 py-2.5 font-bold text-gray-600 bg-gray-100 hover:bg-gray-200 rounded-lg transition flex-1">
            Batal
          </button>
          <button @click="executeDelete" class="px-5 py-2.5 font-bold text-white bg-red-500 hover:bg-red-600 rounded-lg transition flex-1">
            Ya, Hapus
          </button>
        </div>
      </div>
    </div>

  </div>
</template>

<script setup>
import { ref, reactive, onMounted } from 'vue'
import { 
  LayoutDashboardIcon, ArchiveIcon, MapIcon, LeafIcon, WheatIcon, 
  HistoryIcon, LogOutIcon, MenuIcon, XIcon, PlusIcon, EditIcon, 
  Trash2Icon, AlertTriangleIcon, MapPinIcon
} from 'lucide-vue-next'

const isSidebarOpen = ref(false)

// ==========================================
// 1. DATA MODEL (CLASS)
// ==========================================
class LahanModel {
  constructor(data) {
    this.id = data.id || `LHN-${Math.floor(Math.random() * 900) + 100}`;
    this.name = data.name;
    this.area = parseFloat(data.area);
    this.soilType = data.soilType;
    this.status = data.status;
    this.location = data.location;
  }

  // Deserialisasi dari JSON API Response
  static fromJSON(jsonApiData) {
    return new LahanModel({
      id: jsonApiData.kode_lahan,
      name: jsonApiData.nama_lahan,
      area: jsonApiData.luas_hektar,
      soilType: jsonApiData.jenis_tanah,
      status: jsonApiData.status_operasional,
      location: jsonApiData.koordinat_lokasi
    });
  }

  // Serialisasi untuk request API (POST/PUT)
  toJSON() {
    return {
      kode_lahan: this.id,
      nama_lahan: this.name,
      luas_hektar: this.area,
      jenis_tanah: this.soilType,
      status_operasional: this.status,
      koordinat_lokasi: this.location
    }
  }
}

// ==========================================
// 2. SIMULASI DATA API
// ==========================================
const dummyApiJsonResponse = [
  { kode_lahan: 'LHN-101', nama_lahan: 'Blok A - Lahan Utara', luas_hektar: 2.5, jenis_tanah: 'Tanah Humus', status_operasional: 'Aktif Ditanami', koordinat_lokasi: '-0.5022, 117.1536' },
  { kode_lahan: 'LHN-102', nama_lahan: 'Blok B - Lahan Kering', luas_hektar: 3.2, jenis_tanah: 'Tanah Lempung', status_operasional: 'Persiapan', koordinat_lokasi: 'Timur Gudang Utama' },
  { kode_lahan: 'LHN-103', nama_lahan: 'Blok C - Sawah Timur', luas_hektar: 4.0, jenis_tanah: 'Tanah Gambut', status_operasional: 'Masa Bera / Istirahat', koordinat_lokasi: '-0.4912, 117.1601' },
]

// ==========================================
// 3. STATE MANAGEMENT
// ==========================================
const lahanList = ref([])

onMounted(() => {
  lahanList.value = dummyApiJsonResponse.map(data => LahanModel.fromJSON(data))
})

// ==========================================
// 4. CRUD LOGIC & MODALS
// ==========================================
const isFormModalOpen = ref(false)
const isEditMode = ref(false)
const editingId = ref(null)

const formData = reactive({
  name: '',
  area: null,
  soilType: '',
  status: '',
  location: ''
})

const openModal = (mode, item = null) => {
  if (mode === 'edit' && item) {
    isEditMode.value = true
    editingId.value = item.id
    formData.name = item.name
    formData.area = item.area
    formData.soilType = item.soilType
    formData.status = item.status
    formData.location = item.location
  } else {
    isEditMode.value = false
    editingId.value = null
    formData.name = ''
    formData.area = null
    formData.soilType = ''
    formData.status = ''
    formData.location = ''
  }
  isFormModalOpen.value = true
}

const closeModal = () => {
  isFormModalOpen.value = false
}

const saveData = () => {
  if (!formData.name || formData.area === null || !formData.soilType || !formData.status) {
    alert("Mohon lengkapi field yang wajib diisi!")
    return
  }

  if (isEditMode.value) {
    const index = lahanList.value.findIndex(item => item.id === editingId.value)
    if (index !== -1) {
      lahanList.value[index] = new LahanModel({
        id: editingId.value,
        name: formData.name,
        area: formData.area,
        soilType: formData.soilType,
        status: formData.status,
        location: formData.location
      })
    }
  } else {
    const newItem = new LahanModel({
      name: formData.name,
      area: formData.area,
      soilType: formData.soilType,
      status: formData.status,
      location: formData.location
    })
    lahanList.value.unshift(newItem)
  }
  
  closeModal()
}

// DELETE LOGIC
const isDeleteModalOpen = ref(false)
const itemToDelete = ref(null)

const confirmDelete = (item) => {
  itemToDelete.value = item
  isDeleteModalOpen.value = true
}

const closeDeleteModal = () => {
  isDeleteModalOpen.value = false
  setTimeout(() => { itemToDelete.value = null }, 200)
}

const executeDelete = () => {
  if (itemToDelete.value) {
    lahanList.value = lahanList.value.filter(item => item.id !== itemToDelete.value.id)
    closeDeleteModal()
  }
}
</script>

<style scoped>
a {
  transition: all 0.2s ease-in-out;
}

.no-scrollbar::-webkit-scrollbar { display: none; }
.no-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }

/* Inverted Border Radius pada Menu "Lahan" */
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

.animate-in { animation: animateIn 0.3s cubic-bezier(0.16, 1, 0.3, 1) forwards; }
@keyframes animateIn {
  from { opacity: 0; transform: scale(0.95) translateY(10px); }
  to { opacity: 1; transform: scale(1) translateY(0); }
}
</style>