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
            <h1 class="text-xl md:text-2xl font-extrabold text-gray-800 leading-tight">Manajemen Inventori</h1>
            <p class="text-xs md:text-sm text-gray-500 font-medium mt-0.5">Kelola stok barang, pupuk, dan peralatan</p>
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
              <h2 class="text-base md:text-lg font-bold text-gray-800">Daftar Inventori</h2>
            </div>
            
            <button @click="openModal('create')" class="w-full sm:w-auto px-4 py-2.5 font-bold text-white bg-[#1a402d] hover:bg-[#143222] rounded-lg transition shadow-md flex items-center justify-center gap-2">
              <PlusIcon class="w-5 h-5" /> Tambahkan Inventori
            </button>
          </div>
          
          <div class="overflow-x-auto -mx-4 md:mx-0 px-4 md:px-0">
            <table class="w-full text-sm text-left min-w-[800px]">
              <thead class="text-xs text-green-800 uppercase bg-[#e1f0e5] font-bold">
                <tr>
                  <th scope="col" class="px-4 md:px-6 py-4 rounded-l-lg">ID Barang</th>
                  <th scope="col" class="px-4 md:px-6 py-4">Nama Barang</th>
                  <th scope="col" class="px-4 md:px-6 py-4">Kategori</th>
                  <th scope="col" class="px-4 md:px-6 py-4">Stok</th>
                  <th scope="col" class="px-4 md:px-6 py-4">Tgl Masuk</th>
                  <th scope="col" class="px-4 md:px-6 py-4 rounded-r-lg text-center">Aksi</th>
                </tr>
              </thead>
              <tbody>
                <tr v-if="inventoryList.length === 0">
                  <td colspan="6" class="text-center py-10 text-gray-500 font-medium">Belum ada data inventori.</td>
                </tr>
                <tr v-for="item in inventoryList" :key="item.id" class="border-b border-gray-50 last:border-0 hover:bg-gray-50 transition-colors">
                  <td class="px-4 md:px-6 py-4 font-bold text-gray-500">{{ item.id }}</td>
                  <td class="px-4 md:px-6 py-4 font-bold text-gray-800">{{ item.name }}</td>
                  <td class="px-4 md:px-6 py-4">
                    <span class="px-2.5 py-1 rounded-full text-xs font-bold" 
                      :class="item.category === 'Pupuk' ? 'bg-blue-100 text-blue-700' : (item.category === 'Bibit' ? 'bg-green-100 text-green-700' : 'bg-orange-100 text-orange-700')">
                      {{ item.category }}
                    </span>
                  </td>
                  <td class="px-4 md:px-6 py-4 font-bold text-gray-800">
                    {{ item.stock }} <span class="text-xs text-gray-500 font-medium">{{ item.unit }}</span>
                  </td>
                  <td class="px-4 md:px-6 py-4 font-semibold text-gray-600">{{ item.dateIn }}</td>
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
              <ArchiveIcon class="w-6 h-6" />
            </div>
            <h2 class="text-lg md:text-xl font-extrabold text-gray-800">
              {{ isEditMode ? 'Edit Inventori' : 'Tambah Inventori Baru' }}
            </h2>
          </div>
          <button @click="closeModal" class="p-2 text-gray-400 hover:bg-gray-100 hover:text-gray-700 rounded-lg transition shrink-0">
            <XIcon class="w-5 h-5 md:w-6 md:h-6" />
          </button>
        </div>

        <div class="p-4 md:p-6 overflow-y-auto">
          <form @submit.prevent="saveData" class="space-y-5">
            <div>
              <label class="block mb-1.5 text-sm font-bold text-gray-700">Nama Barang</label>
              <input v-model="formData.name" type="text" required class="bg-gray-50 border border-gray-200 text-gray-900 text-sm rounded-lg focus:ring-[#1a402d] focus:border-[#1a402d] block w-full p-2.5 outline-none transition" placeholder="Cth: Pupuk Urea" />
            </div>
            
            <div class="grid grid-cols-1 md:grid-cols-2 gap-5">
              <div>
                <label class="block mb-1.5 text-sm font-bold text-gray-700">Kategori</label>
                <select v-model="formData.category" required class="bg-gray-50 border border-gray-200 text-gray-900 text-sm rounded-lg focus:ring-[#1a402d] focus:border-[#1a402d] block w-full p-2.5 outline-none transition">
                  <option value="" disabled>Pilih Kategori</option>
                  <option value="Pupuk">Pupuk</option>
                  <option value="Bibit">Bibit</option>
                  <option value="Alat Pertanian">Alat Pertanian</option>
                  <option value="Lainnya">Lainnya</option>
                </select>
              </div>
              
              <div>
                <label class="block mb-1.5 text-sm font-bold text-gray-700">Tanggal Masuk</label>
                <input v-model="formData.dateIn" type="date" required class="bg-gray-50 border border-gray-200 text-gray-900 text-sm rounded-lg focus:ring-[#1a402d] focus:border-[#1a402d] block w-full p-2.5 outline-none transition" />
              </div>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-5">
              <div>
                <label class="block mb-1.5 text-sm font-bold text-gray-700">Stok Awal / Jumlah</label>
                <input v-model.number="formData.stock" type="number" min="0" required class="bg-gray-50 border border-gray-200 text-gray-900 text-sm rounded-lg focus:ring-[#1a402d] focus:border-[#1a402d] block w-full p-2.5 outline-none transition" placeholder="0" />
              </div>
              
              <div>
                <label class="block mb-1.5 text-sm font-bold text-gray-700">Satuan</label>
                <select v-model="formData.unit" required class="bg-gray-50 border border-gray-200 text-gray-900 text-sm rounded-lg focus:ring-[#1a402d] focus:border-[#1a402d] block w-full p-2.5 outline-none transition">
                  <option value="" disabled>Pilih Satuan</option>
                  <option value="Kg">Kilogram (Kg)</option>
                  <option value="Liter">Liter (L)</option>
                  <option value="Karung">Karung</option>
                  <option value="Pcs">Pcs</option>
                </select>
              </div>
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
        <h2 class="text-xl font-extrabold text-gray-800 mb-2">Hapus Data?</h2>
        <p class="text-sm text-gray-500 mb-6">Anda yakin ingin menghapus <strong>{{ itemToDelete?.name }}</strong>? Tindakan ini tidak dapat dibatalkan.</p>
        
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
  Trash2Icon, AlertTriangleIcon 
} from 'lucide-vue-next'

// Sidebar Mobile Toggle
const isSidebarOpen = ref(false)

// ==========================================
// 1. DATA MODEL (CLASS)
// ==========================================
// Class DTO untuk standarisasi antara DB (JSON API) dan UI State
class InventoriModel {
  constructor(data) {
    this.id = data.id || `INV-${Math.floor(Math.random() * 9000) + 1000}`; // Generate ID acak jika baru
    this.name = data.name;
    this.category = data.category;
    this.stock = data.stock;
    this.unit = data.unit;
    this.dateIn = data.dateIn;
  }

  // Mapper dari Response Backend
  static fromJSON(jsonApiData) {
    return new InventoriModel({
      id: jsonApiData.kode_barang,
      name: jsonApiData.nama_barang,
      category: jsonApiData.kategori,
      stock: jsonApiData.jumlah_stok,
      unit: jsonApiData.satuan,
      dateIn: jsonApiData.tanggal_masuk
    });
  }

  // Mapper untuk Post/Put Request ke Backend
  toJSON() {
    return {
      kode_barang: this.id,
      nama_barang: this.name,
      kategori: this.category,
      jumlah_stok: this.stock,
      satuan: this.unit,
      tanggal_masuk: this.dateIn
    }
  }
}

// ==========================================
// 2. SIMULASI DATA API
// ==========================================
const dummyApiJsonResponse = [
  { kode_barang: 'INV-1001', nama_barang: 'Pupuk Urea Kaltim', kategori: 'Pupuk', jumlah_stok: 50, satuan: 'Karung', tanggal_masuk: '2026-05-10' },
  { kode_barang: 'INV-1002', nama_barang: 'Bibit Jagung Bonanza', kategori: 'Bibit', jumlah_stok: 15, satuan: 'Kg', tanggal_masuk: '2026-05-15' },
  { kode_barang: 'INV-1003', nama_barang: 'Cangkul Baja', kategori: 'Alat Pertanian', jumlah_stok: 5, satuan: 'Pcs', tanggal_masuk: '2026-04-20' },
  { kode_barang: 'INV-1004', nama_barang: 'Pupuk NPK Mutiara', kategori: 'Pupuk', jumlah_stok: 30, satuan: 'Karung', tanggal_masuk: '2026-06-01' },
]

// ==========================================
// 3. STATE MANAGEMENT
// ==========================================
const inventoryList = ref([])

// Muat data saat komponen di-render
onMounted(() => {
  // Simulasi fetch()
  inventoryList.value = dummyApiJsonResponse.map(data => InventoriModel.fromJSON(data))
})

// ==========================================
// 4. CRUD LOGIC & MODALS
// ==========================================
const isFormModalOpen = ref(false)
const isEditMode = ref(false)
const editingId = ref(null)

// State form terpusat
const formData = reactive({
  name: '',
  category: '',
  stock: null,
  unit: '',
  dateIn: ''
})

// BUKA MODAL FORM (CREATE/EDIT)
const openModal = (mode, item = null) => {
  if (mode === 'edit' && item) {
    isEditMode.value = true
    editingId.value = item.id
    // Isi form dengan data yang diklik
    formData.name = item.name
    formData.category = item.category
    formData.stock = item.stock
    formData.unit = item.unit
    formData.dateIn = item.dateIn
  } else {
    isEditMode.value = false
    editingId.value = null
    // Reset Form
    formData.name = ''
    formData.category = ''
    formData.stock = null
    formData.unit = ''
    // Set default ke hari ini
    formData.dateIn = new Date().toISOString().split('T')[0]
  }
  isFormModalOpen.value = true
}

const closeModal = () => {
  isFormModalOpen.value = false
}

// SIMPAN DATA (CREATE / UPDATE)
const saveData = () => {
  if (!formData.name || !formData.category || formData.stock === null || !formData.unit) {
    alert("Mohon lengkapi semua field!")
    return
  }

  if (isEditMode.value) {
    // Cari index data yang diubah
    const index = inventoryList.value.findIndex(item => item.id === editingId.value)
    if (index !== -1) {
      // Update data pada list
      inventoryList.value[index] = new InventoriModel({
        id: editingId.value,
        name: formData.name,
        category: formData.category,
        stock: formData.stock,
        unit: formData.unit,
        dateIn: formData.dateIn
      })
    }
  } else {
    // Tambah data baru (ID otomatis terbuat di constructor)
    const newItem = new InventoriModel({
      name: formData.name,
      category: formData.category,
      stock: formData.stock,
      unit: formData.unit,
      dateIn: formData.dateIn
    })
    inventoryList.value.unshift(newItem) // Tambah ke paling atas
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
    inventoryList.value = inventoryList.value.filter(item => item.id !== itemToDelete.value.id)
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