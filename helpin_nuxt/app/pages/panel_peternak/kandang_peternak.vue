<template>
  <div class="flex h-screen w-full bg-[#f4f7f5] font-sans overflow-hidden relative text-slate-900">

    <SidebarPeternak :isOpen="isSidebarOpen" @close="isSidebarOpen = false" />

    <main class="flex-1 flex flex-col overflow-y-auto relative w-full no-scrollbar">
      <header class="flex justify-between items-center px-6 md:px-10 py-6 border-b border-gray-200 bg-white shadow-sm z-10 sticky top-0">
        <div class="flex items-center gap-4">
          <button @click="isSidebarOpen = true" class="md:hidden p-2 text-gray-600 hover:bg-gray-100 rounded-lg transition"><MenuIcon class="w-6 h-6" /></button>
          <div>
            <h1 class="text-xl md:text-2xl font-extrabold text-gray-800 leading-tight">Manajemen Kandang</h1>
            <p class="text-xs md:text-sm text-gray-500 font-medium mt-0.5">Okupansi, kondisi otomatis & analisa pakan</p>
          </div>
        </div>
        <div class="flex items-center gap-2 bg-green-100 px-3 py-1.5 md:px-4 md:py-2 rounded-lg border border-green-200">
          <div class="w-2 h-2 rounded-full bg-green-600 shrink-0 animate-pulse"></div>
          <span class="text-xs md:text-sm font-bold text-green-600">ONLINE</span>
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
          <div v-if="loading" class="py-20 flex justify-center"><div class="animate-spin rounded-full h-8 w-8 border-b-2 border-[#1a402d]"></div></div>
          <div v-else class="overflow-x-auto no-scrollbar">
            <table class="w-full text-sm text-left border-collapse min-w-[900px]">
              <thead>
                <tr class="bg-gray-50/50 text-[11px] font-black text-gray-400 uppercase tracking-[0.15em] border-b border-gray-100">
                  <th class="px-8 py-5">Identitas Kandang</th>
                  <th class="px-8 py-5">Tipe Ternak</th>
                  <th class="px-8 py-5 text-center">Okupansi</th>
                  <th class="px-8 py-5">Kondisi (auto)</th>
                  <th class="px-8 py-5 text-center">Aksi</th>
                </tr>
              </thead>
              <tbody class="divide-y divide-gray-50">
                <tr v-if="kandangList.length === 0"><td colspan="5" class="py-16 text-center font-bold text-gray-300">Belum ada kandang</td></tr>
                <tr v-for="item in kandangList" :key="item.id" class="hover:bg-gray-50 transition-colors group">
                  <td class="px-8 py-6">
                    <div class="flex flex-col font-black text-gray-800 uppercase text-base">
                      {{ item.name }} <small class="text-[10px] text-gray-400 font-bold italic tracking-widest normal-case">{{ item.location || 'Tanpa lokasi' }}</small>
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
                        <div :class="['h-full rounded-full', item.occupancy >= item.capacity ? 'bg-red-500' : 'bg-[#1a402d]']" :style="`width: ${Math.min((item.occupancy / Math.max(item.capacity,1)) * 100, 100)}%`"></div>
                      </div>
                    </div>
                  </td>
                  <td class="px-8 py-6">
                    <span :class="['px-3 py-1 rounded-full text-[10px] font-black uppercase tracking-widest', conditionClass(item.condition)]">
                      {{ item.condition || 'Kosong' }}
                    </span>
                  </td>
                  <td class="px-8 py-6 text-center">
                    <div class="flex justify-center gap-2">
                      <button @click="openFeedAnalysis(item)" class="p-2.5 bg-[#1a402d] text-white rounded-xl hover:bg-[#143222] transition-all flex items-center gap-1 text-xs font-bold px-3" title="Analisa Pakan"><SoupIcon class="w-4 h-4" /> Analisa Pakan</button>
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

    <!-- CRUD Modal -->
    <div v-if="isModalOpen" class="fixed inset-0 z-50 flex items-center justify-center p-4">
      <div class="absolute inset-0 bg-[#0c1a13]/80 backdrop-blur-md" @click="closeModal"></div>
      <div class="bg-white rounded-[40px] w-full max-w-2xl shadow-2xl relative z-10 flex flex-col animate-in">
        <div class="bg-gradient-to-br from-[#1a402d] to-[#2d5c41] p-10 text-white rounded-t-[40px]">
          <h2 class="text-3xl font-black italic tracking-tighter">{{ isEditMode ? 'Update Kandang' : 'Kandang Baru' }}</h2>
          <p class="text-white/60 text-sm mt-1">Kondisi kandang akan dihitung otomatis oleh sistem</p>
        </div>
        <div class="p-10 space-y-6">
          <div class="grid grid-cols-2 gap-6">
            <div>
              <label class="text-[10px] font-black text-gray-400 uppercase mb-2 block">Nama Kandang</label>
              <input v-model="formData.name" type="text" class="w-full px-5 py-3.5 bg-gray-50 border border-gray-100 rounded-2xl font-bold" />
            </div>
            <div>
              <label class="text-[10px] font-black text-gray-400 uppercase mb-2 block">Jenis Ternak</label>
              <select v-model="formData.livestockType" class="w-full px-5 py-3.5 bg-gray-50 border border-gray-100 rounded-2xl font-bold">
                <option v-for="j in jenisOptions" :key="j" :value="j">{{ j }}</option>
              </select>
            </div>
          </div>
          <div class="grid grid-cols-2 gap-6">
            <div>
              <label class="text-[10px] font-black text-gray-400 uppercase mb-2 block">Kapasitas (ekor)</label>
              <input v-model.number="formData.capacity" type="number" min="1" class="w-full px-5 py-3.5 bg-gray-50 border border-gray-100 rounded-2xl font-bold" />
            </div>
            <div>
              <label class="text-[10px] font-black text-gray-400 uppercase mb-2 block">Koordinat Lokasi</label>
              <div class="grid grid-cols-2 gap-2">
                <input :value="latText" readonly placeholder="Lat" class="px-3 py-3.5 bg-gray-100 border border-gray-200 rounded-2xl font-mono text-xs text-gray-600 w-full" />
                <input :value="lngText" readonly placeholder="Long" class="px-3 py-3.5 bg-gray-100 border border-gray-200 rounded-2xl font-mono text-xs text-gray-600 w-full" />
              </div>
            </div>
          </div>
          <div>
            <label class="text-[10px] font-black text-gray-400 uppercase mb-2 block">Pilih Lokasi di Peta (klik untuk set titik)</label>
            <div class="relative w-full h-52 bg-gray-100 rounded-2xl overflow-hidden border border-gray-200">
              <div ref="mapContainer" class="w-full h-full">
                <div v-if="!mapLoaded" class="absolute inset-0 flex flex-col items-center justify-center gap-2 text-gray-400">
                  <MapPinIcon class="w-8 h-8" /><p class="text-xs font-bold">Memuat peta...</p>
                </div>
              </div>
            </div>
          </div>
          <div class="flex gap-4">
            <button @click="closeModal" class="flex-1 py-4 bg-gray-100 text-gray-500 rounded-2xl font-black uppercase text-xs">Batal</button>
            <button @click="saveData" :disabled="saving" class="flex-1 py-4 bg-[#1a402d] text-white rounded-2xl font-black shadow-xl uppercase text-xs disabled:opacity-50">{{ saving ? 'Menyimpan...' : 'Simpan' }}</button>
          </div>
        </div>
      </div>
    </div>

    <!-- Feed Analysis Modal -->
    <div v-if="isFeedOpen" class="fixed inset-0 z-50 flex items-center justify-center p-4">
      <div class="absolute inset-0 bg-[#0c1a13]/90 backdrop-blur-xl" @click="isFeedOpen = false"></div>
      <div class="bg-white rounded-[40px] w-full max-w-4xl max-h-[92vh] overflow-hidden shadow-2xl relative z-10 flex flex-col animate-in">
        <div class="bg-gradient-to-br from-[#1a402d] to-[#2d5c41] p-8 text-white relative shrink-0">
          <button @click="isFeedOpen = false" class="absolute top-6 right-6 p-2 bg-white/10 hover:bg-white/20 rounded-full"><XIcon class="w-5 h-5" /></button>
          <div class="flex items-center gap-2 mb-2">
            <span class="px-3 py-1 bg-green-400 text-[#1a402d] text-[10px] font-black rounded-full uppercase tracking-widest">Analisa Pakan AI</span>
            <span v-if="feedResult" class="text-white/50 text-xs font-mono">{{ feedResult.analysis_source }}</span>
          </div>
          <h2 class="text-3xl font-black italic tracking-tighter">{{ feedPen?.name }} — {{ feedPen?.livestockType }}</h2>
        </div>

        <div class="p-8 overflow-y-auto no-scrollbar">
          <div v-if="feedLoading" class="py-16 flex flex-col items-center gap-3 text-gray-400">
            <div class="animate-spin rounded-full h-10 w-10 border-b-2 border-[#1a402d]"></div>
            <p class="font-bold text-sm">Menganalisa kecocokan pakan...</p>
          </div>
          <div v-else-if="feedError" class="py-12 text-center">
            <p class="text-red-500 font-bold mb-1">{{ feedError }}</p>
            <p class="text-gray-400 text-sm">Pastikan kandang berisi ternak & ada pakan (inventori "Pakan" atau produk dengan kandungan nutrisi).</p>
          </div>
          <div v-else-if="feedResult" class="space-y-8">
            <!-- Ringkasan ternak + proyeksi kualitas -->
            <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
              <div class="bg-gray-50 p-5 rounded-3xl border border-gray-100">
                <p class="text-[10px] font-black text-gray-400 uppercase">Populasi</p>
                <p class="text-2xl font-black text-[#1a402d]">{{ feedResult.livestock_count }}</p>
              </div>
              <div class="bg-gray-50 p-5 rounded-3xl border border-gray-100">
                <p class="text-[10px] font-black text-gray-400 uppercase">Rerata Bobot</p>
                <p class="text-2xl font-black text-slate-800">{{ feedResult.avg_weight_kg }}<small class="text-xs"> kg</small></p>
              </div>
              <div class="bg-green-50 p-5 rounded-3xl border border-green-100">
                <p class="text-[10px] font-black text-green-500 uppercase">Grade Proyeksi</p>
                <p class="text-2xl font-black text-green-700">{{ feedResult.quality_projection?.grade }}</p>
              </div>
              <div class="bg-blue-50 p-5 rounded-3xl border border-blue-100">
                <p class="text-[10px] font-black text-blue-500 uppercase">ADG Proyeksi</p>
                <p class="text-2xl font-black text-blue-700">{{ feedResult.quality_projection?.projected_adg_kg }}<small class="text-xs"> kg/hr</small></p>
              </div>
            </div>

            <div class="bg-[#1a402d] text-white rounded-3xl p-6 grid grid-cols-2 md:grid-cols-4 gap-4">
              <div><p class="text-[10px] text-white/50 uppercase font-black">Target Protein</p><p class="text-lg font-black">{{ feedResult.target_nutrition?.protein }}%</p></div>
              <div><p class="text-[10px] text-white/50 uppercase font-black">Target TDN</p><p class="text-lg font-black">{{ feedResult.target_nutrition?.energy_tdn }}%</p></div>
              <div><p class="text-[10px] text-white/50 uppercase font-black">FCR</p><p class="text-lg font-black">{{ feedResult.quality_projection?.fcr }}</p></div>
              <div><p class="text-[10px] text-white/50 uppercase font-black">Target Bobot (90hr)</p><p class="text-lg font-black">{{ feedResult.quality_projection?.target_weight_kg }} kg</p></div>
            </div>

            <!-- Top tier pakan -->
            <div>
              <h3 class="text-xs font-black text-gray-700 uppercase tracking-[0.2em] mb-4 flex items-center gap-2"><SoupIcon class="w-4 h-4 text-orange-500" /> Top Pakan Terbaik</h3>
              <div v-if="(feedResult.top_feeds||[]).length === 0" class="text-sm text-gray-400 italic p-4 bg-gray-50 rounded-2xl">Belum ada kandidat pakan. Tambahkan inventori kategori "Pakan" atau produk pakan dengan kandungan nutrisi.</div>
              <div v-else class="space-y-3">
                <div v-for="(feed, idx) in feedResult.top_feeds" :key="idx" class="flex items-center gap-4 p-4 rounded-3xl border" :class="idx === 0 ? 'border-green-300 bg-green-50/50' : 'border-gray-100 bg-white'">
                  <div class="w-10 h-10 rounded-2xl flex items-center justify-center font-black text-white shrink-0" :class="idx === 0 ? 'bg-green-600' : 'bg-gray-300'">#{{ idx + 1 }}</div>
                  <div class="flex-1 min-w-0">
                    <p class="font-black text-gray-800 truncate">{{ feed.name }} <span class="text-[10px] font-bold text-gray-400 uppercase">({{ feed.source }})</span></p>
                    <p class="text-[11px] text-gray-500 font-medium">Protein {{ feed.nutrition?.protein }}% · TDN {{ feed.nutrition?.energy_tdn }}% · ADG {{ feed.projected_adg_kg }} kg/hr</p>
                  </div>
                  <div class="text-right shrink-0">
                    <p class="text-xl font-black" :class="idx === 0 ? 'text-green-700' : 'text-slate-700'">{{ feed.compatibility_percent }}%</p>
                    <p class="text-[10px] font-black uppercase text-gray-400">Grade {{ feed.grade }}</p>
                  </div>
                </div>
              </div>
            </div>

            <!-- Rekomendasi -->
            <div v-if="(feedResult.recommendations||[]).length" class="bg-orange-50/40 border border-orange-100 rounded-3xl p-6">
              <h3 class="text-xs font-black text-orange-700 uppercase tracking-[0.2em] mb-3">Rekomendasi</h3>
              <ul class="space-y-2">
                <li v-for="(r, i) in feedResult.recommendations" :key="i" class="text-sm font-medium text-slate-700 flex gap-2"><span class="text-orange-500 font-black">•</span> {{ r }}</li>
              </ul>
            </div>
          </div>
        </div>
        <div class="p-6 bg-gray-50 border-t border-gray-100 flex justify-end shrink-0">
          <button @click="runFeedAnalysis" :disabled="feedLoading" class="px-8 py-3 bg-[#1a402d] text-white rounded-2xl font-black uppercase text-xs shadow-lg disabled:opacity-50">{{ feedLoading ? 'Menganalisa...' : 'Analisa Ulang' }}</button>
        </div>
      </div>
    </div>

  </div>
</template>

<script setup>
import { ref, reactive, computed, onMounted, nextTick } from 'vue'
import { HomeIcon, MenuIcon, XIcon, PlusIcon, EditIcon, Trash2Icon, SoupIcon, MapPinIcon } from 'lucide-vue-next'

const isSidebarOpen = ref(false)
const { list: fetchPens, create: createPen, update: updatePen, remove: removePen, analyzeFeed, getFeedAnalysis } = usePen()
const jenisOptions = ['Unggas', 'Mamalia', 'Ruminansia', 'Serangga', 'Aves', 'Ikan', 'Reptil', 'Lainnya']

const loading = ref(false)
const saving = ref(false)

const mapPenItem = (item) => ({
  id: item.id,
  name: item.name,
  capacity: item.capacity,
  occupancy: item.occupancy ?? 0,
  livestockType: item.pen_type || 'Sapi',
  location: item.location || '',
  condition: item.condition || 'Kosong'
})

const kandangList = ref([])
const isModalOpen = ref(false)
const isEditMode = ref(false)
const editingId = ref(null)
const formData = reactive({ name: '', capacity: 10, livestockType: 'Mamalia', location: '', latitude: null, longitude: null })

// Leaflet map state
const mapContainer = ref(null)
const mapLoaded = ref(false)
let mapInstance = null
let markerInstance = null
const latText = computed(() => formData.latitude != null ? Number(formData.latitude).toFixed(5) : '')
const lngText = computed(() => formData.longitude != null ? Number(formData.longitude).toFixed(5) : '')

const initMap = async () => {
  await nextTick()
  if (!mapContainer.value || typeof window === 'undefined') return
  if (!window.L) {
    const link = document.createElement('link')
    link.rel = 'stylesheet'; link.href = 'https://unpkg.com/leaflet@1.9.4/dist/leaflet.css'
    document.head.appendChild(link)
    await new Promise((resolve) => {
      const s = document.createElement('script')
      s.src = 'https://unpkg.com/leaflet@1.9.4/dist/leaflet.js'; s.onload = resolve
      document.head.appendChild(s)
    })
  }
  const L = window.L
  const lat = formData.latitude || -6.2088
  const lng = formData.longitude || 106.8456
  mapInstance = L.map(mapContainer.value).setView([lat, lng], 12)
  L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', { attribution: '© OpenStreetMap' }).addTo(mapInstance)
  if (formData.latitude && formData.longitude) markerInstance = L.marker([lat, lng]).addTo(mapInstance)
  mapInstance.on('click', (e) => {
    formData.latitude = parseFloat(e.latlng.lat.toFixed(6))
    formData.longitude = parseFloat(e.latlng.lng.toFixed(6))
    if (markerInstance) markerInstance.setLatLng([e.latlng.lat, e.latlng.lng])
    else markerInstance = L.marker([e.latlng.lat, e.latlng.lng]).addTo(mapInstance)
  })
  mapLoaded.value = true
  setTimeout(() => mapInstance && mapInstance.invalidateSize(), 300)
}
const destroyMap = () => { if (mapInstance) { mapInstance.remove(); mapInstance = null; markerInstance = null; mapLoaded.value = false } }

const conditionClass = (c) => {
  switch (c) {
    case 'Optimal': return 'bg-green-50 text-green-600'
    case 'Padat': return 'bg-yellow-50 text-yellow-600'
    case 'Perlu Perhatian': return 'bg-orange-50 text-orange-600'
    case 'Overkapasitas': return 'bg-red-50 text-red-600'
    case 'Buruk': return 'bg-red-50 text-red-600'
    default: return 'bg-gray-100 text-gray-500'
  }
}

onMounted(async () => {
  loading.value = true
  try {
    const data = await fetchPens()
    kandangList.value = (data || []).map(mapPenItem)
  } catch (e) { console.error(e); kandangList.value = [] }
  finally { loading.value = false }
})

const totalOccupancy = computed(() => kandangList.value.reduce((a, c) => a + (c.occupancy || 0), 0))

const openModal = async (mode, item = null) => {
  if (mode === 'edit' && item) {
    isEditMode.value = true; editingId.value = item.id
    let lat = null, lng = null
    if (item.location && item.location.includes(',')) {
      const [a, b] = item.location.split(',').map(s => parseFloat(s.trim()))
      if (!isNaN(a) && !isNaN(b)) { lat = a; lng = b }
    }
    Object.assign(formData, { name: item.name, capacity: item.capacity, livestockType: item.livestockType, location: item.location, latitude: lat, longitude: lng })
  } else {
    isEditMode.value = false; editingId.value = null
    Object.assign(formData, { name: '', capacity: 10, livestockType: 'Mamalia', location: '', latitude: null, longitude: null })
  }
  isModalOpen.value = true
  await nextTick()
  setTimeout(() => initMap(), 100)
}
const closeModal = () => { destroyMap(); isModalOpen.value = false }

const saveData = async () => {
  if (!formData.name) { useToast().warning('Data belum lengkap', 'Nama kandang wajib diisi'); return }
  saving.value = true
  try {
    const location = (formData.latitude != null && formData.longitude != null)
      ? `${formData.latitude}, ${formData.longitude}`
      : (formData.location || null)
    const payload = { name: formData.name, capacity: formData.capacity, pen_type: formData.livestockType, location }
    if (isEditMode.value) {
      const updated = await updatePen(editingId.value, payload)
      const idx = kandangList.value.findIndex(i => i.id === editingId.value)
      if (idx !== -1) kandangList.value[idx] = mapPenItem(updated)
    } else {
      const created = await createPen(payload)
      kandangList.value.unshift(mapPenItem(created))
    }
    closeModal()
  } catch (e) { console.error(e); useToast().error('Gagal menyimpan kandang', e?.data?.error?.message || e?.data?.message) }
  finally { saving.value = false }
}

const confirmDelete = async (item) => {
  if (!confirm(`Hapus ${item.name}?`)) return
  try {
    await removePen(item.id)
    kandangList.value = kandangList.value.filter(i => i.id !== item.id)
  } catch (e) { console.error(e); useToast().error('Gagal menghapus kandang', e?.data?.message) }
}

// ===== Feed Analysis =====
const isFeedOpen = ref(false)
const feedPen = ref(null)
const feedResult = ref(null)
const feedLoading = ref(false)
const feedError = ref('')

const openFeedAnalysis = async (item) => {
  feedPen.value = item
  feedResult.value = null
  feedError.value = ''
  isFeedOpen.value = true
  // Coba ambil analisa terakhir; jika belum ada, jalankan analisa baru
  try {
    const history = await getFeedAnalysis(item.id)
    if (history && history.length > 0) {
      feedResult.value = history[0].data || history[0]
    } else {
      await runFeedAnalysis()
    }
  } catch {
    await runFeedAnalysis()
  }
}

const runFeedAnalysis = async () => {
  if (!feedPen.value) return
  feedLoading.value = true
  feedError.value = ''
  try {
    feedResult.value = await analyzeFeed(feedPen.value.id)
  } catch (e) {
    feedError.value = e?.data?.error?.message || e?.data?.message || 'Gagal menganalisa pakan.'
  } finally {
    feedLoading.value = false
  }
}
</script>

<style scoped>
.no-scrollbar::-webkit-scrollbar { display: none; }
.no-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }
.animate-in { animation: fadeIn 0.4s ease-out forwards; }
@keyframes fadeIn { from { opacity: 0; transform: translateY(20px); } to { opacity: 1; transform: translateY(0); } }
</style>
