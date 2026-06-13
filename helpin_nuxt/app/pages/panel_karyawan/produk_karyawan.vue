<template>
  <div class="flex h-screen w-full bg-[#F4FBF7] font-sans overflow-hidden relative">
    <SidebarKaryawan :isOpen="isSidebarOpen" @close="isSidebarOpen = false" />
    <main class="flex-1 flex flex-col overflow-y-auto relative w-full no-scrollbar">
      <header class="flex justify-between items-center px-6 md:px-10 py-5 border-b border-gray-200 bg-white/90 backdrop-blur-md shadow-sm z-10 sticky top-0">
        <div class="flex items-center gap-4">
          <button @click="isSidebarOpen = true" class="md:hidden p-2 -ml-2 text-gray-600 hover:bg-gray-100 rounded-lg transition"><MenuIcon class="w-6 h-6" /></button>
          <div>
            <h1 class="text-xl md:text-2xl font-black text-[#19462D] tracking-tight">Katalog Produk</h1>
            <p class="text-xs md:text-sm text-gray-500 font-medium mt-0.5">CRUD produk + analisa gizi otomatis + auto kas</p>
          </div>
        </div>
      </header>

      <div class="p-4 md:p-8 flex flex-col gap-6 animate-fade">
        <section class="bg-white rounded-3xl shadow-sm border border-gray-100 p-6">
          <div class="flex flex-col sm:flex-row gap-4 mb-6">
            <div class="relative flex-1">
              <SearchIcon class="w-5 h-5 absolute left-4 top-1/2 -translate-y-1/2 text-gray-400" />
              <input v-model="search" placeholder="Cari Produk..." class="w-full pl-12 pr-4 py-3 border border-gray-200 rounded-xl outline-none focus:border-[#19462D] bg-gray-50 text-sm">
            </div>
            <button @click="openModal()" class="bg-[#19462D] hover:bg-[#113620] text-white px-6 py-3 rounded-xl font-bold flex items-center justify-center gap-2 transition shadow-lg text-sm uppercase tracking-wider shrink-0">
              <PackagePlusIcon class="w-5 h-5" /> Tambah Produk
            </button>
          </div>

          <div v-if="loading" class="py-16 text-center text-gray-400 font-bold">Memuat...</div>
          <div v-else-if="filtered.length === 0" class="py-16 text-center text-gray-300 font-bold">Belum ada produk</div>
          <div v-else class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
            <div v-for="p in filtered" :key="p.id" class="border border-gray-200 rounded-2xl p-4 hover:shadow-lg transition bg-white group relative">
              <img :src="imgUrl(p.image_url)" alt="Product" class="w-full h-32 object-contain mb-4 rounded-xl bg-gray-50" @error="onImgError" />
              <h4 class="text-base font-black text-[#19462D] mb-2 truncate">{{ p.name }}</h4>
              <div class="text-[11px] text-gray-500 text-left space-y-1 mb-3 bg-gray-50 p-3 rounded-xl border border-gray-100">
                <p class="flex justify-between"><span>Kategori:</span> <strong class="text-gray-800">{{ p.product_type || '-' }}</strong></p>
                <p class="flex justify-between"><span>Harga Beli:</span> <strong class="text-gray-800">{{ formatRupiah(p.purchase_price) }}</strong></p>
                <p class="flex justify-between"><span>Harga Jual:</span> <strong class="text-green-600">{{ formatRupiah(p.price) }}</strong></p>
                <p class="flex justify-between"><span>Stok:</span> <strong class="text-gray-800">{{ formatUnit(p.stock, p.unit) }}</strong></p>
                <p class="flex justify-between"><span>Expired:</span> <strong class="text-red-500">{{ p.expired_at ? formatDate(p.expired_at) : '-' }}</strong></p>
                <p class="flex justify-between" v-if="nutriGrade(p)"><span>Nutri-Grade:</span> <strong class="text-blue-600">{{ nutriGrade(p) }}</strong></p>
              </div>
              <div class="flex gap-2">
                <button @click="detail = p" class="flex-1 py-2 bg-blue-50 text-blue-600 rounded-lg text-xs font-bold hover:bg-blue-500 hover:text-white transition">Gizi</button>
                <button @click="openModal(p)" class="py-2 px-3 bg-yellow-50 text-yellow-600 rounded-lg hover:bg-yellow-500 hover:text-white transition"><EditIcon class="w-4 h-4" /></button>
                <button @click="remove(p)" class="py-2 px-3 bg-red-50 text-red-600 rounded-lg hover:bg-red-500 hover:text-white transition"><Trash2Icon class="w-4 h-4" /></button>
              </div>
            </div>
          </div>
        </section>
      </div>
    </main>

    <div v-if="modal" class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50 backdrop-blur-sm">
      <div class="bg-white rounded-2xl w-full max-w-3xl shadow-xl overflow-hidden animate-fade max-h-[92vh] flex flex-col">
        <div class="flex justify-between items-center p-6 border-b border-gray-100 shrink-0">
          <h2 class="text-xl font-bold text-[#19462D]">{{ editId ? 'Edit Produk' : 'Tambah Produk' }}</h2>
          <button @click="modal = false" class="text-gray-400 hover:text-red-500"><XIcon class="w-6 h-6" /></button>
        </div>
        <div class="p-6 bg-gray-50/50 overflow-y-auto flex-1">
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
            <div><label class="block text-xs font-bold text-[#19462D] mb-1.5 uppercase">Nama Produk</label><input v-model="form.name" placeholder="Cth: Mie Goreng" class="w-full p-3 border border-gray-200 rounded-xl bg-white text-sm"></div>
            <div>
              <label class="block text-xs font-bold text-[#19462D] mb-1.5 uppercase">Kategori</label>
              <select v-model="form.product_type" class="w-full p-3 border border-gray-200 rounded-xl bg-white text-sm">
                <option v-for="c in categoryOptions" :key="c" :value="c">{{ c }}</option>
              </select>
            </div>
            <div><label class="block text-xs font-bold text-[#19462D] mb-1.5 uppercase">Harga Jual (Rp)</label><input v-model="priceFmt" type="text" inputmode="numeric" placeholder="0" class="w-full p-3 border border-gray-200 rounded-xl bg-white text-sm"></div>
            <div><label class="block text-xs font-bold text-[#19462D] mb-1.5 uppercase">Harga Beli (Rp)</label><input v-model="purchaseFmt" type="text" inputmode="numeric" placeholder="0" class="w-full p-3 border border-gray-200 rounded-xl bg-white text-sm"></div>
            <div><label class="block text-xs font-bold text-[#19462D] mb-1.5 uppercase">Stok</label><input v-model="stockFmt" type="text" inputmode="numeric" placeholder="0" class="w-full p-3 border border-gray-200 rounded-xl bg-white text-sm"></div>
            <div>
              <label class="block text-xs font-bold text-[#19462D] mb-1.5 uppercase">Satuan</label>
              <select v-model="form.unit" class="w-full p-3 border border-gray-200 rounded-xl bg-white text-sm">
                <option v-for="u in unitOptions" :key="u" :value="u">{{ u }}</option>
              </select>
            </div>
            <div><label class="block text-xs font-bold text-[#19462D] mb-1.5 uppercase">Tanggal Expired</label><input v-model="form.expired_at" type="date" class="w-full p-3 border border-gray-200 rounded-xl bg-white text-sm"></div>
          </div>
          <div class="mb-4">
            <label class="block text-xs font-bold text-[#19462D] mb-1.5 uppercase">Deskripsi</label>
            <textarea v-model="form.description" rows="2" class="w-full p-3 border border-gray-200 rounded-xl bg-white text-sm"></textarea>
          </div>

          <label class="block text-xs font-bold text-[#19462D] mb-1.5 uppercase">Gambar Produk</label>
          <div class="flex items-center gap-4 mb-2">
            <img v-if="form.image_url" :src="imgUrl(form.image_url)" class="w-20 h-20 object-cover rounded-xl border border-gray-200" />
            <label class="flex-1 py-4 border-2 border-dashed border-[#19462D] text-[#19462D] font-bold rounded-xl hover:bg-green-50 transition flex justify-center items-center gap-2 cursor-pointer">
              <ImageIcon class="w-5 h-5" /> {{ uploading ? 'Mengunggah...' : 'Pilih Gambar' }}
              <input type="file" accept="image/*" class="hidden" @change="onFile" />
            </label>
          </div>
          <p class="text-[11px] text-gray-400 mb-2">Kandungan gizi akan dianalisa otomatis di backend (AI/ensemble). Harga beli otomatis tercatat sebagai pengeluaran kas.</p>
        </div>
        <div class="p-6 border-t border-gray-100 shrink-0">
          <button @click="save" :disabled="saving || uploading" class="w-full px-8 py-4 bg-[#19462D] text-white font-bold rounded-xl hover:bg-[#113620] transition uppercase tracking-wider text-sm shadow-md disabled:opacity-50">{{ saving ? 'Menyimpan...' : 'Simpan Produk' }}</button>
        </div>
      </div>
    </div>

    <div v-if="detail" class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50 backdrop-blur-sm" @click.self="detail = null">
      <div class="bg-white rounded-3xl w-full max-w-md shadow-2xl p-8 animate-fade">
        <div class="flex justify-between items-center mb-6">
          <h2 class="text-xl font-black text-[#19462D]">Kandungan Gizi</h2>
          <button @click="detail = null" class="text-gray-400 hover:text-red-500"><XIcon class="w-6 h-6" /></button>
        </div>
        <h3 class="font-black text-gray-800 mb-1">{{ detail.name }}</h3>
        <p class="text-xs text-gray-400 mb-4">{{ detail.product_type || '-' }}</p>
        <div v-if="nutriPer100(detail)" class="space-y-2 text-sm">
          <div v-for="(v, k) in nutriPer100(detail)" :key="k" class="flex justify-between border-b border-gray-50 pb-1.5">
            <span class="text-gray-500 font-bold capitalize">{{ formatKey(k) }}</span>
            <span class="font-black text-gray-800">{{ v }}</span>
          </div>
          <div v-if="nutriGrade(detail)" class="flex justify-between pt-2">
            <span class="text-gray-500 font-bold">Nutri-Grade</span>
            <span class="px-3 py-0.5 bg-blue-100 text-blue-700 rounded-full font-black">{{ nutriGrade(detail) }}</span>
          </div>
        </div>
        <p v-else class="text-sm text-gray-400 italic">{{ detail.nutrition?.note || 'Belum ada data gizi.' }}</p>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive, computed, onMounted } from 'vue'
import { MenuIcon, SearchIcon, PackagePlusIcon, ImageIcon, XIcon, EditIcon, Trash2Icon } from 'lucide-vue-next'
import SidebarKaryawan from '~/components/SidebarKaryawan.vue'

const isSidebarOpen = ref(false)
const { list: fetchProducts, create, update, remove: removeProduct } = useProductAdmin()
const { uploadImage } = useUpload()
const { formatRupiah, formatNumber, formatUnit, formatDate, parseNumber } = useFormat()
const toast = useToast()

const products = ref([])
const loading = ref(false)
const saving = ref(false)
const uploading = ref(false)
const search = ref('')
const modal = ref(false)
const editId = ref(null)
const detail = ref(null)
const form = reactive({ name: '', product_type: 'Makanan', price: null, purchase_price: null, stock: 0, unit: 'Pcs', expired_at: '', description: '', image_url: '' })
const categoryOptions = ['Makanan', 'Minuman', 'Snack', 'Sembako', 'Sayuran', 'Buah', 'Pakan', 'Pupuk', 'Bibit', 'Alat Pertanian', 'Lainnya']
const unitOptions = ['Pcs', 'Kg', 'Gram', 'Liter', 'Ml', 'Karung', 'Ikat', 'Box', 'Pack', 'Botol']

const load = async () => {
  loading.value = true
  try { products.value = (await fetchProducts()) || [] } catch (e) { console.error(e); toast.error('Gagal memuat produk', e?.data?.message) } finally { loading.value = false }
}
onMounted(load)

const filtered = computed(() => products.value.filter(p => p.name?.toLowerCase().includes(search.value.toLowerCase())))

const priceFmt = computed({
  get: () => (form.price === null || form.price === '' ? '' : formatNumber(form.price, 0)),
  set: (v) => { form.price = v === '' ? null : parseNumber(v) },
})
const purchaseFmt = computed({
  get: () => (form.purchase_price === null || form.purchase_price === '' ? '' : formatNumber(form.purchase_price, 0)),
  set: (v) => { form.purchase_price = v === '' ? null : parseNumber(v) },
})
const stockFmt = computed({
  get: () => (form.stock === null || form.stock === '' ? '' : formatNumber(form.stock, 0)),
  set: (v) => { form.stock = v === '' ? 0 : parseNumber(v) },
})

const imgUrl = (u) => {
  if (!u) return 'data:image/svg+xml;utf8,<svg xmlns="http://www.w3.org/2000/svg" width="120" height="120"><rect width="100%" height="100%" fill="%23f1f5f9"/></svg>'
  if (u.startsWith('http') || u.startsWith('data:')) return u
  return u
}
const onImgError = (e) => { e.target.style.opacity = '0.3' }
const nutriPer100 = (p) => p?.nutrition?.per_100g || null
const nutriGrade = (p) => p?.nutrition?.nutri_grade || null
const formatKey = (k) => k.replace(/_/g, ' ').replace('kcal', '(kkal)').replace(' g', ' (g)').replace(' mg', ' (mg)')

const openModal = (p = null) => {
  if (p) {
    editId.value = p.id
    Object.assign(form, { name: p.name, product_type: p.product_type || 'Makanan', price: p.price, purchase_price: p.purchase_price || 0, stock: p.stock, unit: p.unit || 'Pcs', expired_at: p.expired_at || '', description: p.description || '', image_url: p.image_url || '' })
  } else {
    editId.value = null
    Object.assign(form, { name: '', product_type: 'Makanan', price: null, purchase_price: null, stock: 0, unit: 'Pcs', expired_at: '', description: '', image_url: '' })
  }
  modal.value = true
}

const onFile = async (e) => {
  const file = e.target.files?.[0]
  if (!file) return
  uploading.value = true
  try { form.image_url = await uploadImage(file) }
  catch (err) { console.error(err); toast.error('Gagal mengunggah gambar') }
  finally { uploading.value = false }
}

const save = async () => {
  if (!form.name || !form.price || !form.unit) { toast.warning('Lengkapi data', 'Nama, harga jual, dan satuan wajib diisi'); return }
  saving.value = true
  try {
    const payload = {
      name: form.name,
      product_type: form.product_type || null,
      price: Number(form.price),
      purchase_price: form.purchase_price ? Number(form.purchase_price) : 0,
      stock: Number(form.stock) || 0,
      unit: form.unit,
      expired_at: form.expired_at || null,
      description: form.description || null,
      image_url: form.image_url || null,
    }
    if (editId.value) await update(editId.value, payload)
    else await create(payload)
    modal.value = false
    toast.success(editId.value ? 'Produk diperbarui' : 'Produk ditambahkan', form.name)
    await load()
  } catch (e) { console.error(e); toast.error('Gagal menyimpan produk', e?.data?.error?.message || e?.data?.message) }
  finally { saving.value = false }
}

const remove = async (p) => {
  if (!confirm(`Hapus produk ${p.name}?`)) return
  try { await removeProduct(p.id); toast.success('Produk dihapus', p.name); await load() } catch (e) { toast.error('Gagal menghapus produk', e?.data?.message) }
}
</script>

<style scoped>
.no-scrollbar::-webkit-scrollbar { display: none; }
.no-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }
.animate-fade { animation: fadeIn 0.4s cubic-bezier(0.16, 1, 0.3, 1); }
@keyframes fadeIn { from { opacity: 0; transform: translateY(10px); } to { opacity: 1; transform: translateY(0); } }
</style>
