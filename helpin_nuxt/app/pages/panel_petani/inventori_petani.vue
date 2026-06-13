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
            <h1 class="text-xl md:text-2xl font-extrabold text-gray-800 leading-tight">Inventori Saya</h1>
            <p class="text-xs md:text-sm text-gray-500 font-medium mt-0.5">
              Kelola barang Anda — pupuk, bibit, dan alat pertanian yang akan dipakai di lahan
            </p>
          </div>
        </div>
        <div class="flex items-center gap-2 bg-green-100 px-3 py-1.5 md:px-4 md:py-2 rounded-lg border border-green-200">
          <div class="w-2 h-2 rounded-full bg-green-600 shrink-0 animate-pulse"></div>
          <span class="text-xs md:text-sm font-bold text-green-600 hidden sm:block">STATUS ONLINE</span>
        </div>
      </header>

      <div class="p-4 md:p-8 flex flex-col gap-5 w-full max-w-[100vw]">
        
        <!-- Info banner -->
        <div class="bg-blue-50 border border-blue-200 rounded-xl p-4 flex items-start gap-3">
          <InfoIcon class="w-5 h-5 text-blue-500 shrink-0 mt-0.5" />
          <div class="text-sm text-blue-700">
            <span class="font-bold">Cara kerja:</span> Tambahkan barang Anda di sini dulu (pupuk, bibit, alat pertanian). 
            Saat membuat lahan, Anda bisa memilih bibit apa yang ditanam dan pupuk/alat yang dipakai dari daftar ini.
          </div>
        </div>

        <!-- Summary by category -->
        <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
          <div v-for="cat in categorySummary" :key="cat.name" 
            class="bg-white rounded-xl border border-gray-100 shadow-sm p-4 flex items-center gap-3">
            <div class="w-10 h-10 rounded-xl flex items-center justify-center shrink-0" :class="cat.bgClass">
              <component :is="cat.icon" class="w-5 h-5" :class="cat.iconClass" />
            </div>
            <div>
              <p class="text-[10px] font-black text-gray-400 uppercase">{{ cat.name }}</p>
              <p class="text-xl font-black text-gray-800">{{ cat.count }}</p>
            </div>
          </div>
        </div>

        <!-- Table -->
        <section class="bg-white rounded-2xl shadow-sm border border-gray-100 p-4 md:p-6 w-full flex flex-col flex-1">
          <div class="flex flex-col sm:flex-row justify-between items-start sm:items-center mb-5 gap-4">
            <div class="flex items-center gap-2">
              <div class="w-2 h-2 rounded-full bg-[#1a402d]"></div>
              <h2 class="text-base md:text-lg font-bold text-gray-800">Daftar Inventori</h2>
              <span class="text-xs bg-gray-100 text-gray-500 px-2 py-0.5 rounded-full font-bold">{{ inventoryList.length }} item</span>
            </div>
            <button @click="openModal()" class="w-full sm:w-auto px-4 py-2.5 font-bold text-white bg-[#1a402d] hover:bg-[#143222] rounded-lg transition shadow-md flex items-center justify-center gap-2">
              <PlusIcon class="w-5 h-5" /> Tambah Barang
            </button>
          </div>

          <div v-if="loading" class="flex justify-center items-center py-20">
            <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-[#1a402d]"></div>
          </div>

          <div v-else class="overflow-x-auto -mx-4 md:mx-0 px-4 md:px-0">
            <table class="w-full text-sm text-left min-w-[600px]">
              <thead class="text-xs text-green-800 uppercase bg-[#e1f0e5] font-bold">
                <tr>
                  <th class="px-4 md:px-6 py-4 rounded-l-lg">Nama Barang</th>
                  <th class="px-4 md:px-6 py-4">Kategori</th>
                  <th class="px-4 md:px-6 py-4">Stok</th>
                  <th class="px-4 md:px-6 py-4">Tgl Masuk</th>
                  <th class="px-4 md:px-6 py-4 rounded-r-lg text-center">Aksi</th>
                </tr>
              </thead>
              <tbody>
                <tr v-if="inventoryList.length === 0">
                  <td colspan="5" class="text-center py-14 text-gray-400">
                    <div class="flex flex-col items-center gap-3">
                      <ArchiveIcon class="w-12 h-12 text-gray-200" />
                      <div>
                        <p class="font-bold">Belum ada inventori</p>
                        <p class="text-xs mt-1">Tambah pupuk, bibit, atau alat pertanian milik Anda</p>
                      </div>
                    </div>
                  </td>
                </tr>
                <tr v-for="item in inventoryList" :key="item.id"
                  class="border-b border-gray-50 last:border-0 hover:bg-gray-50/70 transition-colors">
                  <td class="px-4 md:px-6 py-4 font-bold text-gray-800">{{ item.name }}</td>
                  <td class="px-4 md:px-6 py-4">
                    <span class="px-2.5 py-1 rounded-full text-xs font-bold"
                      :class="categoryClass(item.category)">
                      {{ item.category }}
                    </span>
                  </td>
                  <td class="px-4 md:px-6 py-4 font-bold text-gray-800">
                    {{ parseFloat(item.quantity).toFixed(1) }}
                    <span class="text-xs text-gray-500 font-medium">{{ item.unit }}</span>
                  </td>
                  <td class="px-4 md:px-6 py-4 font-medium text-gray-500">{{ formatDate(item.created_at) }}</td>
                  <td class="px-4 md:px-6 py-4 flex justify-center gap-2">
                    <button @click="openModal(item)" class="bg-[#eab308] hover:bg-yellow-600 text-white p-2 rounded-md transition shadow-sm" title="Edit">
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

    <!-- Form Modal -->
    <div v-if="isFormModalOpen" class="fixed inset-0 z-50 flex items-center justify-center p-4">
      <div class="absolute inset-0 bg-black/60 backdrop-blur-sm" @click="closeModal"></div>
      <div class="bg-white rounded-2xl w-full max-w-lg shadow-2xl relative z-10 flex flex-col max-h-[95vh] animate-modal">
        <div class="flex justify-between items-center p-5 border-b border-gray-100 shrink-0">
          <div class="flex items-center gap-3">
            <div class="p-2 bg-green-100 text-green-700 rounded-lg">
              <ArchiveIcon class="w-5 h-5" />
            </div>
            <h2 class="text-lg font-extrabold text-gray-800">
              {{ isEditMode ? 'Edit Barang' : 'Tambah Barang Baru' }}
            </h2>
          </div>
          <button @click="closeModal" class="p-2 text-gray-400 hover:bg-gray-100 rounded-lg transition">
            <XIcon class="w-5 h-5" />
          </button>
        </div>

        <div class="p-5 overflow-y-auto space-y-4">
          <div>
            <label class="block mb-1.5 text-sm font-bold text-gray-700">Nama Barang <span class="text-red-500">*</span></label>
            <input v-model="formData.name" type="text" required
              class="bg-gray-50 border border-gray-200 text-gray-900 text-sm rounded-lg focus:ring-[#1a402d] focus:border-[#1a402d] block w-full p-2.5 outline-none transition"
              placeholder="Cth: Pupuk Urea 46%, Bibit Padi IR64, Cangkul..." />
          </div>

          <div class="grid grid-cols-2 gap-4">
            <div>
              <label class="block mb-1.5 text-sm font-bold text-gray-700">Kategori <span class="text-red-500">*</span></label>
              <select v-model="formData.category" required
                class="bg-gray-50 border border-gray-200 text-gray-900 text-sm rounded-lg focus:ring-[#1a402d] focus:border-[#1a402d] block w-full p-2.5 outline-none transition">
                <option value="" disabled>Pilih kategori</option>
                <option value="Pupuk">🌿 Pupuk</option>
                <option value="Bibit">🌱 Bibit</option>
                <option value="Alat Pertanian">🔧 Alat Pertanian</option>
                <option value="Lainnya">📦 Lainnya</option>
              </select>
            </div>
            <div>
              <label class="block mb-1.5 text-sm font-bold text-gray-700">Satuan <span class="text-red-500">*</span></label>
              <select v-model="formData.unit" required
                class="bg-gray-50 border border-gray-200 text-gray-900 text-sm rounded-lg focus:ring-[#1a402d] focus:border-[#1a402d] block w-full p-2.5 outline-none transition">
                <option value="" disabled>Pilih satuan</option>
                <option value="Kg">Kilogram (Kg)</option>
                <option value="Liter">Liter (L)</option>
                <option value="Karung">Karung</option>
                <option value="Pcs">Pcs</option>
                <option value="Botol">Botol</option>
                <option value="Gram">Gram</option>
              </select>
            </div>
          </div>

          <div>
            <label class="block mb-1.5 text-sm font-bold text-gray-700">Jumlah / Stok <span class="text-red-500">*</span></label>
            <input v-model.number="formData.quantity" type="number" min="0" step="0.1" required
              class="bg-gray-50 border border-gray-200 text-gray-900 text-sm rounded-lg focus:ring-[#1a402d] focus:border-[#1a402d] block w-full p-2.5 outline-none transition"
              placeholder="Masukkan jumlah..." />
          </div>

          <!-- Tip berdasarkan kategori -->
          <div v-if="formData.category === 'Bibit'" class="bg-green-50 border border-green-200 rounded-lg p-3 text-xs text-green-700">
            <span class="font-bold">💡 Tips Bibit:</span> Nama bibit akan menjadi komoditas lahan saat Anda memilih bibit ini di form Lahan. 
            Contoh: "Bibit Padi IR64" → lahan otomatis bertipe "Padi IR64".
          </div>
          <div v-if="formData.category === 'Pupuk'" class="bg-blue-50 border border-blue-200 rounded-lg p-3 text-xs text-blue-700">
            <span class="font-bold">💡 Tips Pupuk:</span> Pupuk yang Anda daftarkan bisa dipilih saat membuat lahan untuk dicatat sebagai input nutrisi lahan tersebut.
          </div>
        </div>

        <div class="p-5 border-t border-gray-100 flex justify-end gap-3 bg-gray-50 rounded-b-2xl shrink-0">
          <button @click="closeModal" type="button" class="px-5 py-2.5 font-bold text-gray-600 hover:bg-gray-200 rounded-lg transition">
            Batal
          </button>
          <button @click="saveData" type="button" :disabled="saving"
            class="px-5 py-2.5 font-bold text-white bg-[#1a402d] hover:bg-[#143222] rounded-lg transition shadow-md flex items-center gap-2 disabled:opacity-50">
            <LoaderIcon v-if="saving" class="w-4 h-4 animate-spin" />
            {{ saving ? 'Menyimpan...' : (isEditMode ? 'Update' : 'Simpan') }}
          </button>
        </div>
      </div>
    </div>

    <!-- Delete Modal -->
    <div v-if="isDeleteModalOpen" class="fixed inset-0 z-50 flex items-center justify-center p-4">
      <div class="absolute inset-0 bg-black/60 backdrop-blur-sm" @click="closeDeleteModal"></div>
      <div class="bg-white rounded-2xl w-full max-w-sm shadow-2xl relative z-10 p-6 text-center animate-modal">
        <div class="w-14 h-14 bg-red-100 rounded-full flex items-center justify-center mx-auto mb-4 text-red-500">
          <Trash2Icon class="w-7 h-7" />
        </div>
        <h2 class="text-lg font-extrabold text-gray-800 mb-1">Hapus Barang?</h2>
        <p class="text-sm text-gray-500 mb-5">
          Hapus <strong>{{ itemToDelete?.name }}</strong>? Jika barang ini sedang dipakai di lahan, kaitan tersebut juga akan hilang.
        </p>
        <div class="flex gap-3">
          <button @click="closeDeleteModal" class="flex-1 py-2.5 font-bold text-gray-600 bg-gray-100 hover:bg-gray-200 rounded-lg transition">Batal</button>
          <button @click="executeDelete" :disabled="saving" class="flex-1 py-2.5 font-bold text-white bg-red-500 hover:bg-red-600 rounded-lg transition disabled:opacity-50">Hapus</button>
        </div>
      </div>
    </div>

  </div>
</template>

<script setup>
import { ref, reactive, computed, onMounted } from 'vue'
import { 
  ArchiveIcon, InfoIcon, MenuIcon, XIcon, PlusIcon, EditIcon, 
  Trash2Icon, LoaderIcon, SproutIcon, FlaskConicalIcon, WrenchIcon, PackageIcon
} from 'lucide-vue-next'

const isSidebarOpen = ref(false)
const loading = ref(false)
const saving = ref(false)

const { listAllInventory, createInventory, updateInventory, deleteInventory } = useLand()

const inventoryList = ref([])

const categoryClass = (cat) => ({
  'bg-blue-100 text-blue-700': cat === 'Pupuk',
  'bg-green-100 text-green-700': cat === 'Bibit',
  'bg-orange-100 text-orange-700': cat === 'Alat Pertanian',
  'bg-gray-100 text-gray-700': cat === 'Lainnya',
})

const categorySummary = computed(() => {
  const counts = {}
  inventoryList.value.forEach(i => { counts[i.category] = (counts[i.category] || 0) + 1 })
  return [
    { name: 'Pupuk', count: counts['Pupuk'] || 0, bgClass: 'bg-blue-50', iconClass: 'text-blue-600', icon: FlaskConicalIcon },
    { name: 'Bibit', count: counts['Bibit'] || 0, bgClass: 'bg-green-50', iconClass: 'text-green-600', icon: SproutIcon },
    { name: 'Alat', count: counts['Alat Pertanian'] || 0, bgClass: 'bg-orange-50', iconClass: 'text-orange-500', icon: WrenchIcon },
    { name: 'Lainnya', count: counts['Lainnya'] || 0, bgClass: 'bg-gray-50', iconClass: 'text-gray-500', icon: PackageIcon },
  ]
})

const formatDate = (d) => {
  if (!d) return '-'
  return new Date(d).toLocaleDateString('id-ID', { day: '2-digit', month: 'short', year: 'numeric' })
}

onMounted(async () => {
  loading.value = true
  try {
    inventoryList.value = await listAllInventory() || []
  } catch (e) {
    console.error(e)
    inventoryList.value = []
  } finally {
    loading.value = false
  }
})

// CRUD
const isFormModalOpen = ref(false)
const isEditMode = ref(false)
const editingId = ref(null)
const formData = reactive({ name: '', category: '', quantity: null, unit: '' })

const openModal = (item = null) => {
  if (item) {
    isEditMode.value = true
    editingId.value = item.id
    formData.name = item.name
    formData.category = item.category
    formData.quantity = parseFloat(item.quantity)
    formData.unit = item.unit
  } else {
    isEditMode.value = false
    editingId.value = null
    formData.name = ''
    formData.category = ''
    formData.quantity = null
    formData.unit = ''
  }
  isFormModalOpen.value = true
}

const closeModal = () => { isFormModalOpen.value = false }

const saveData = async () => {
  if (!formData.name || !formData.category || formData.quantity === null || !formData.unit) {
    useToast().warning('Data belum lengkap', 'Lengkapi semua field')
    return
  }
  saving.value = true
  try {
    const payload = { name: formData.name, category: formData.category, quantity: formData.quantity, unit: formData.unit }
    if (isEditMode.value) {
      const updated = await updateInventory(editingId.value, payload)
      const idx = inventoryList.value.findIndex(i => i.id === editingId.value)
      if (idx !== -1) inventoryList.value[idx] = updated || { ...inventoryList.value[idx], ...payload }
    } else {
      const created = await createInventory(payload)
      inventoryList.value.unshift(created || { ...payload, id: crypto.randomUUID(), created_at: new Date().toISOString() })
    }
    closeModal()
  } catch (e) {
    console.error(e)
    useToast().error('Gagal menyimpan', e?.data?.error?.message || e?.data?.message)
  } finally {
    saving.value = false
  }
}

const isDeleteModalOpen = ref(false)
const itemToDelete = ref(null)
const confirmDelete = (item) => { itemToDelete.value = item; isDeleteModalOpen.value = true }
const closeDeleteModal = () => { isDeleteModalOpen.value = false; setTimeout(() => { itemToDelete.value = null }, 200) }
const executeDelete = async () => {
  if (!itemToDelete.value) return
  saving.value = true
  try {
    await deleteInventory(itemToDelete.value.id)
    inventoryList.value = inventoryList.value.filter(i => i.id !== itemToDelete.value.id)
    closeDeleteModal()
  } catch (e) {
    useToast().error('Gagal menghapus', e?.data?.message)
  } finally {
    saving.value = false
  }
}
</script>

<style scoped>
.animate-modal { animation: modalIn 0.2s ease forwards; }
@keyframes modalIn {
  from { opacity: 0; transform: scale(0.97) translateY(8px); }
  to   { opacity: 1; transform: scale(1) translateY(0); }
}
</style>
