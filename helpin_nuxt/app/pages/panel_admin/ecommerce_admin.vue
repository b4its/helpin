<template>
  <div class="flex h-screen w-full bg-[#F4FBF7] font-sans overflow-hidden relative">
    
    <SidebarAdmin :isOpen="isSidebarOpen" @close="isSidebarOpen = false" />

    <main class="flex-1 flex flex-col overflow-y-auto relative w-full no-scrollbar">
      <header class="flex justify-between items-center px-6 md:px-10 py-5 border-b border-gray-200 bg-white/90 backdrop-blur-md shadow-sm z-10 sticky top-0">
        <div class="flex items-center gap-4">
          <button @click="isSidebarOpen = true" class="md:hidden p-2 -ml-2 text-gray-600 hover:bg-gray-100 rounded-lg transition">
            <MenuIcon class="w-6 h-6" />
          </button>
          <div class="flex items-center gap-3">
            <div class="w-10 h-10 rounded-xl bg-purple-50 flex items-center justify-center text-purple-600 hidden sm:flex">
              <StoreIcon class="w-5 h-5" />
            </div>
            <div>
              <h1 class="text-xl md:text-2xl font-black text-[#19462D] tracking-tight">Custom E-Commerce</h1>
              <p class="text-xs md:text-sm text-gray-500 font-medium mt-0.5">Kelola tata letak kategori dan banner promosi toko</p>
            </div>
          </div>
        </div>
      </header>

      <div class="p-4 md:p-8 flex flex-col gap-8 w-full max-w-[100vw] animate-fade">
        
        <section class="bg-white rounded-3xl shadow-sm border border-gray-100 p-6">
          <h3 class="text-lg font-bold text-[#19462D] flex items-center gap-2 mb-6">
            <TagsIcon class="w-5 h-5 text-blue-500" /> Kategori Produk
          </h3>
          
          <div class="flex flex-col md:flex-row gap-4 mb-6">
            <div class="relative w-full md:flex-1">
              <SearchIcon class="w-4 h-4 absolute left-4 top-1/2 -translate-y-1/2 text-gray-400" />
              <input v-model="searchKategoriNama" type="text" placeholder="Cari Nama Kategori..." class="w-full pl-11 pr-4 py-3.5 border border-gray-200 rounded-xl outline-none focus:border-[#19462D] bg-gray-50 text-sm font-medium transition">
            </div>
            <div class="relative w-full md:flex-1">
              <SearchIcon class="w-4 h-4 absolute left-4 top-1/2 -translate-y-1/2 text-gray-400" />
              <input v-model="searchKategoriNomor" type="text" placeholder="Cari Nomor Urut..." class="w-full pl-11 pr-4 py-3.5 border border-gray-200 rounded-xl outline-none focus:border-[#19462D] bg-gray-50 text-sm font-medium transition">
            </div>
            <button @click="bukaModalTambahKategori" class="bg-blue-500 hover:bg-blue-600 text-white px-8 py-3.5 rounded-xl font-bold flex items-center justify-center gap-2 transition shadow-sm whitespace-nowrap">
              <PlusCircleIcon class="w-5 h-5" /> Tambah Kategori
            </button>
          </div>

          <div class="overflow-x-auto no-scrollbar">
            <table class="w-full text-sm text-left min-w-[600px]">
              <thead class="text-xs text-green-800 bg-green-50 uppercase tracking-widest border-b border-gray-100">
                <tr>
                  <th class="p-4 font-black rounded-tl-lg w-24">Nomor Urut</th>
                  <th class="p-4 font-black">Nama Kategori</th>
                  <th class="p-4 font-black text-center rounded-tr-lg w-32">Aksi</th>
                </tr>
              </thead>
              <tbody class="divide-y divide-gray-50">
                <tr v-if="filteredKategoriData.length === 0">
                  <td colspan="3" class="p-6 text-center text-gray-400 font-bold">Kategori tidak ditemukan</td>
                </tr>
                
                <tr v-for="(cat, i) in filteredKategoriData" :key="i" class="hover:bg-gray-50/50 transition-colors">
                  <td class="p-4 font-black text-[#19462D]">{{ cat.nomorUrut }}</td>
                  <td class="p-4 font-bold text-gray-700 flex items-center gap-2">
                    <TagIcon class="w-4 h-4 text-gray-400" /> {{ cat.nama }}
                  </td>
                  <td class="p-4">
                    <div class="flex items-center justify-center gap-2">
                      <button @click="bukaModalEditKategori(cat)" class="w-8 h-8 rounded-lg bg-yellow-50 text-yellow-600 flex items-center justify-center hover:bg-yellow-500 hover:text-white transition shadow-sm border border-yellow-100">
                        <EditIcon class="w-4 h-4" />
                      </button>
                      <button @click="showDeleteModal = true" class="w-8 h-8 rounded-lg bg-red-50 text-red-600 flex items-center justify-center hover:bg-red-500 hover:text-white transition shadow-sm border border-red-100">
                        <TrashIcon class="w-4 h-4" />
                      </button>
                    </div>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </section>

        <section class="bg-white rounded-3xl shadow-sm border border-gray-100 p-6">
          <h3 class="text-lg font-bold text-[#19462D] flex items-center gap-2 mb-6">
            <LayoutTemplateIcon class="w-5 h-5 text-orange-500" /> Banner Promosi
          </h3>
          
          <div class="flex flex-col md:flex-row gap-4 mb-4">
            <div class="relative w-full md:flex-1">
              <SearchIcon class="w-4 h-4 absolute left-4 top-1/2 -translate-y-1/2 text-gray-400" />
              <input v-model="searchBannerNama" type="text" placeholder="Cari Nama Banner..." class="w-full pl-11 pr-4 py-3.5 border border-gray-200 rounded-xl outline-none focus:border-[#19462D] bg-gray-50 text-sm font-medium transition">
            </div>
            <div class="relative w-full md:flex-1">
              <SearchIcon class="w-4 h-4 absolute left-4 top-1/2 -translate-y-1/2 text-gray-400" />
              <input v-model="searchBannerNomor" type="text" placeholder="Cari Nomor Urut..." class="w-full pl-11 pr-4 py-3.5 border border-gray-200 rounded-xl outline-none focus:border-[#19462D] bg-gray-50 text-sm font-medium transition">
            </div>
            <button @click="bukaModalTambahProduk" class="bg-blue-500 hover:bg-blue-600 text-white px-8 py-3.5 rounded-xl font-bold flex items-center justify-center gap-2 transition shadow-sm whitespace-nowrap">
              <PlusCircleIcon class="w-5 h-5" /> Tambah Banner
            </button>
          </div>
          
          <button @click="bukaModalTambahProduk" class="w-full bg-green-50 hover:bg-green-100 border-2 border-dashed border-green-600 text-green-700 py-4 rounded-xl font-black flex items-center justify-center gap-2 transition mb-8 mt-2 uppercase tracking-widest text-xs">
            <ImagePlusIcon class="w-5 h-5" /> Upload File Gambar Banner
          </button>

          <div class="overflow-x-auto no-scrollbar">
            <table class="w-full text-sm text-left min-w-[700px]">
              <thead class="text-xs text-green-800 bg-green-50 uppercase tracking-widest border-b border-gray-100">
                <tr>
                  <th class="p-4 font-black rounded-tl-lg w-24">Nomor</th>
                  <th class="p-4 font-black">Nama Banner/Produk</th>
                  <th class="p-4 font-black">File Gambar</th>
                  <th class="p-4 font-black text-center rounded-tr-lg w-40">Aksi</th>
                </tr>
              </thead>
              <tbody class="divide-y divide-gray-50">
                <tr v-if="filteredBannerData.length === 0">
                  <td colspan="4" class="p-6 text-center text-gray-400 font-bold">Banner tidak ditemukan</td>
                </tr>

                <tr v-for="(ban, i) in filteredBannerData" :key="i" class="hover:bg-gray-50/50 transition-colors">
                  <td class="p-4 font-black text-[#19462D]">0{{ i + 1 }}</td>
                  <td class="p-4 font-bold text-gray-700">{{ ban.nama }}</td>
                  <td class="p-4">
                    <a href="#" class="inline-flex items-center gap-2 text-blue-600 font-bold bg-blue-50 px-3 py-1.5 rounded-lg border border-blue-100 hover:bg-blue-500 hover:text-white transition">
                      <ImageIcon class="w-4 h-4" /> {{ ban.file }}
                    </a>
                  </td>
                  <td class="p-4">
                    <div class="flex items-center justify-center gap-2">
                      <button @click="showBannerPreviewModal = true" class="w-8 h-8 rounded-lg bg-blue-50 text-blue-600 flex items-center justify-center hover:bg-blue-500 hover:text-white transition shadow-sm border border-blue-100"><EyeIcon class="w-4 h-4" /></button>
                      <button @click="bukaModalEditProduk(ban)" class="w-8 h-8 rounded-lg bg-yellow-50 text-yellow-600 flex items-center justify-center hover:bg-yellow-500 hover:text-white transition shadow-sm border border-yellow-100"><EditIcon class="w-4 h-4" /></button>
                      <button @click="showDeleteModal = true" class="w-8 h-8 rounded-lg bg-red-50 text-red-600 flex items-center justify-center hover:bg-red-500 hover:text-white transition shadow-sm border border-red-100"><TrashIcon class="w-4 h-4" /></button>
                    </div>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </section>

      </div>
    </main>

    <div v-if="showKategoriModal" class="fixed inset-0 z-[100] flex items-center justify-center bg-black/40 backdrop-blur-sm p-4" @click.self="showKategoriModal = false">
      <div class="bg-white rounded-[32px] w-full max-w-lg shadow-2xl relative animate-fade p-6 md:p-8">
        <div class="flex items-center justify-between mb-8">
          <h1 class="text-2xl font-extrabold text-[#19462D]">
            {{ kategoriMode === 'tambah' ? 'Kategori Baru' : 'Edit Kategori' }}
          </h1>
          <button @click="showKategoriModal = false" class="p-2 hover:bg-gray-100 rounded-xl transition">
            <XIcon class="w-6 h-6 text-black" />
          </button>
        </div>
        <div class="space-y-5">
          <div>
            <label class="block text-xs font-bold text-[#19462D] mb-2 uppercase tracking-wide">Nama Kategori</label>
            <input v-model="formKategori.nama" type="text" placeholder="Contoh : Minuman Ringan" class="w-full px-4 py-3.5 rounded-xl border border-gray-200 bg-gray-50 text-sm focus:bg-white focus:outline-none focus:ring-2 focus:ring-[#19462D]/20 focus:border-[#19462D] transition">
          </div>
          <div>
            <label class="block text-xs font-bold text-[#19462D] mb-2 uppercase tracking-wide">Nomor Urut</label>
            <input v-model="formKategori.nomorUrut" type="number" placeholder="Contoh : 01" class="w-full px-4 py-3.5 rounded-xl border border-gray-200 bg-gray-50 text-sm focus:bg-white focus:outline-none focus:ring-2 focus:ring-[#19462D]/20 focus:border-[#19462D] transition">
          </div>
        </div>
        <button @click="simpanDataKategori" class="w-full mt-8 py-4 rounded-2xl bg-[#19462D] text-white font-extrabold tracking-widest hover:bg-[#113620] active:scale-[0.98] transition shadow-lg uppercase text-sm">
          {{ kategoriMode === 'tambah' ? 'SIMPAN KATEGORI' : 'SIMPAN PERUBAHAN' }}
        </button>
      </div>
    </div>


    <div v-if="showProdukModal" class="fixed inset-0 z-[100] flex items-center justify-center bg-black/40 backdrop-blur-sm p-4" @click.self="showProdukModal = false">
      <div class="bg-white rounded-[32px] w-full max-w-4xl max-h-[95vh] overflow-y-auto p-6 md:p-10 shadow-2xl relative animate-fade no-scrollbar">
        <div class="flex items-center gap-4 mb-8">
          <button @click="showProdukModal = false" class="p-2 hover:bg-gray-100 rounded-xl transition flex items-center justify-center">
            <XIcon class="w-8 h-8 text-black" />
          </button>
          <h1 class="text-2xl md:text-3xl font-extrabold text-[#19462D]">
            {{ produkMode === 'tambah' ? 'Tambah Data' : 'Edit' }}
          </h1>
        </div>
        <div class="border border-gray-100 bg-[#F8FAFC] rounded-[24px] p-6 md:p-8">
          <div class="flex items-center gap-3 mb-6 text-xs font-extrabold tracking-widest text-[#19462D] uppercase">
            <span class="w-2.5 h-2.5 rounded-full bg-[#19462D]"></span> 
            {{ produkMode === 'tambah' ? 'TAMBAH PRODUK BARU' : 'EDIT DATA PRODUK' }}
          </div>
          <div class="grid grid-cols-1 md:grid-cols-2 gap-x-8 gap-y-6">
            <div>
              <label class="block text-xs font-bold text-[#19462D] mb-2 uppercase">Nama Produk / Banner</label>
              <input v-model="formProduk.nama" type="text" placeholder="Contoh : Mie Goreng" class="w-full px-4 py-3.5 rounded-xl border border-gray-200 bg-white text-sm focus:border-[#19462D] outline-none">
            </div>
            <div>
              <label class="block text-xs font-bold text-[#19462D] mb-2 uppercase">Harga Jual</label>
              <input v-model="formProduk.hargaJual" type="text" placeholder="Contoh : Rp 23.000,00.." class="w-full px-4 py-3.5 rounded-xl border border-gray-200 bg-white text-sm focus:border-[#19462D] outline-none">
            </div>
            <div class="relative">
              <label class="block text-xs font-bold text-[#19462D] mb-2 uppercase">Harga Beli</label>
              <input v-model="formProduk.hargaBeli" type="text" placeholder="Rp 20.000" class="w-full px-4 py-3.5 rounded-xl border-2>
              ">
             <div>
             </div>
            </div>
            <div>
              <label class="block text-xs font-bold text-[#19462D] mb-2 uppercase">Kategori</label>
              <select v-model="formProduk.kategori" class="w-full px-4 py-3.5 rounded-xl border border-gray-200 bg-white text-sm font-bold outline-none cursor-pointer">
                <option value="" disabled>Pilih Komoditas</option>
                <option value="makanan">Makanan</option>
                <option value="minuman">Minuman</option>
              </select>
            </div>
            <div>
              <label class="block text-xs font-bold text-[#19462D] mb-2 uppercase">Stok</label>
              <input v-model="formProduk.stok" type="text" placeholder="Contoh : 23 Pcs" class="w-full px-4 py-3.5 rounded-xl border border-gray-200 bg-white text-sm outline-none">
            </div>
            <div>
              <label class="block text-xs font-bold text-[#19462D] mb-2 uppercase">Expired</label>
              <input v-model="formProduk.expired" type="date" class="w-full px-4 py-3.5 rounded-xl border border-gray-200 bg-white text-sm font-bold outline-none cursor-pointer text-gray-700">
            </div>
            <div>
              <label class="block text-xs font-bold text-[#19462D] mb-2 uppercase">Input Berat</label>
              <input v-model="formProduk.berat" type="text" class="w-full px-4 py-3.5 rounded-xl border border-gray-200 bg-white text-sm outline-none">
            </div>
            <div>
              <label class="block text-xs font-bold text-[#19462D] mb-2 uppercase">Satuan Berat</label>
              <select v-model="formProduk.satuanBerat" class="w-full px-4 py-3.5 rounded-xl border border-gray-200 bg-white text-sm font-bold outline-none cursor-pointer">
                <option value="" disabled>Pilih Satuan Berat</option>
                <option value="kg">Kilogram (Kg)</option>
                <option value="gr">Gram (Gr)</option>
              </select>
            </div>
          </div>
          <div class="mt-8 flex flex-col gap-4">
            <button class="flex items-center justify-center gap-3 w-full py-4 rounded-2xl border-2 border-[#19462D] bg-white text-[#19462D] font-extrabold hover:bg-green-50 transition">
              <ImagePlusIcon class="w-5 h-5" /> Upload File Gambar
            </button>
            <button @click="simpanDataProduk" class="w-full py-4 rounded-2xl bg-gradient-to-b from-[#2A5E3D] to-[#12361F] text-white font-extrabold tracking-widest hover:opacity-90 active:scale-[0.99] transition shadow-lg uppercase">
              {{ produkMode === 'tambah' ? 'SIMPAN PRODUK / BANNER' : 'SIMPAN PERUBAHAN' }}
            </button>
          </div>
        </div>
      </div>
    </div>


    <div v-if="showBannerPreviewModal" class="fixed inset-0 z-[100] flex items-center justify-center p-4 bg-black/50 backdrop-blur-sm transition-opacity" @click.self="showBannerPreviewModal = false">
      <div class="bg-white rounded-3xl w-full max-w-2xl shadow-xl overflow-hidden animate-fade">
        <div class="flex justify-between items-center p-6 border-b border-gray-100">
          <div class="flex items-center gap-3">
              <div class="w-10 h-10 rounded-full bg-blue-50 flex items-center justify-center text-blue-600"><ImageIcon class="w-5 h-5" /></div>
              <h2 class="text-xl font-black text-[#19462D]">Preview Banner</h2>
          </div>
          <button @click="showBannerPreviewModal = false" class="text-gray-400 hover:text-red-500 transition"><XIcon class="w-6 h-6" /></button>
        </div>
        <div class="p-6 bg-gray-50/50 text-center">
          <img src="https://images.unsplash.com/photo-1604147706283-d7119b5b822c?auto=format&fit=crop&q=80&w=600&h=300" alt="Preview" class="w-full h-auto rounded-2xl shadow-sm mb-4 object-cover max-h-[300px] border border-gray-200">
          <h3 class="text-lg font-black text-gray-800">Promo Idul Fitri</h3>
          <button @click="showBannerPreviewModal = false" class="mt-4 w-full max-w-[200px] py-3.5 bg-[#19462D] text-white font-black rounded-xl hover:bg-[#113620] transition uppercase text-sm mx-auto shadow-lg shadow-green-900/20 tracking-widest">Tutup Preview</button>
        </div>
      </div>
    </div>

    <div v-if="showDeleteModal" class="fixed inset-0 z-[100] flex items-center justify-center p-4 bg-black/50 backdrop-blur-sm">
      <div class="bg-white rounded-2xl w-full max-w-sm p-6 text-center shadow-xl animate-fade">
        <div class="w-16 h-16 bg-red-50 text-red-500 rounded-full flex items-center justify-center mx-auto mb-4"><TrashIcon class="w-8 h-8" /></div>
        <h3 class="text-xl font-black text-[#19462D] mb-6">Menghapus Informasi?</h3>
        <div class="flex gap-4">
          <button @click="showDeleteModal = false" class="flex-1 px-6 py-3.5 bg-gray-100 text-gray-700 font-black rounded-xl hover:bg-gray-200 transition uppercase text-sm">BATAL</button>
          <button @click="showDeleteModal = false" class="flex-1 px-6 py-3.5 bg-red-500 text-white font-black rounded-xl hover:bg-red-600 transition uppercase text-sm shadow-md shadow-red-500/20">YA, HAPUS</button>
        </div>
      </div>
    </div>

  </div>
</template>

<script setup>
import { ref, reactive, computed } from 'vue'
import { MenuIcon, StoreIcon, TagsIcon, TagIcon, PlusCircleIcon, LayoutTemplateIcon, ImagePlusIcon, ImageIcon, EditIcon, TrashIcon, EyeIcon, XIcon, FileTextIcon, SearchIcon } from 'lucide-vue-next'
import SidebarAdmin from '~/components/SidebarAdmin.vue'

const isSidebarOpen = ref(false)
const showDeleteModal = ref(false)
const showBannerPreviewModal = ref(false)

// ==========================================
// LOGIKA SEARCH & DATA KATEGORI
// ==========================================
const searchKategoriNama = ref('')
const searchKategoriNomor = ref('')
const showKategoriModal = ref(false)
const kategoriMode = ref('tambah')
const formKategori = reactive({ nama: '', nomorUrut: '' })

const kategoriData = ref([
  { nomorUrut: '01', nama: 'Makanan' },
  { nomorUrut: '02', nama: 'Minuman' },
  { nomorUrut: '03', nama: 'Sembako' },
])

// Filter data Kategori berdasarkan input Search
const filteredKategoriData = computed(() => {
  return kategoriData.value.filter(cat => {
    const matchNama = cat.nama.toLowerCase().includes(searchKategoriNama.value.toLowerCase())
    const matchNomor = cat.nomorUrut.includes(searchKategoriNomor.value)
    return matchNama && matchNomor
  })
})

const bukaModalTambahKategori = () => {
  kategoriMode.value = 'tambah'
  formKategori.nama = ''
  formKategori.nomorUrut = ''
  showKategoriModal.value = true
}

const bukaModalEditKategori = (cat) => {
  kategoriMode.value = 'edit'
  formKategori.nama = cat.nama
  formKategori.nomorUrut = cat.nomorUrut
  showKategoriModal.value = true
}

const simpanDataKategori = () => {
  showKategoriModal.value = false
}

// ==========================================
// LOGIKA SEARCH & DATA BANNER/PRODUK
// ==========================================
const searchBannerNama = ref('')
const searchBannerNomor = ref('')
const showProdukModal = ref(false)
const produkMode = ref('tambah')
const formProduk = reactive({
  nama: '', hargaJual: '', hargaBeli: '', kategori: '', stok: '', expired: '', berat: '', satuanBerat: ''
})

const bannerData = ref([
  { nomor: '01', nama: 'Promo Idul Fitri 1', file: 'Banner_promo_1.jpg' },
  { nomor: '02', nama: 'Promo Akhir Bulan', file: 'Banner_promo_2.jpg' },
  { nomor: '03', nama: 'Cuci Gudang', file: 'Banner_promo_3.jpg' },
])

// Filter data Banner berdasarkan input Search
const filteredBannerData = computed(() => {
  return bannerData.value.filter(ban => {
    const matchNama = ban.nama.toLowerCase().includes(searchBannerNama.value.toLowerCase())
    const matchNomor = ban.nomor.includes(searchBannerNomor.value)
    return matchNama && matchNomor
  })
})

const bukaModalTambahProduk = () => {
  produkMode.value = 'tambah'
  Object.keys(formProduk).forEach(key => formProduk[key] = '')
  showProdukModal.value = true
}

const bukaModalEditProduk = (ban) => {
  produkMode.value = 'edit'
  Object.keys(formProduk).forEach(key => formProduk[key] = '')
  formProduk.nama = ban.nama
  showProdukModal.value = true
}

const simpanDataProduk = () => {
  showProdukModal.value = false
}
</script>

<style scoped>
.no-scrollbar::-webkit-scrollbar { display: none; }
.no-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }
.animate-fade { animation: fadeIn 0.3s cubic-bezier(0.16, 1, 0.3, 1); }
@keyframes fadeIn { from { opacity: 0; transform: translateY(10px); } to { opacity: 1; transform: translateY(0); } }
</style>