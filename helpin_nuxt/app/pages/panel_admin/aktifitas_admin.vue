<template>
  <div class="flex h-screen w-full bg-[#F4FBF7] font-sans overflow-hidden relative">
    <SidebarAdmin :isOpen="isSidebarOpen" @close="isSidebarOpen = false" />
    <main class="flex-1 flex flex-col overflow-y-auto relative w-full no-scrollbar">
      <header class="flex justify-between items-center px-6 md:px-10 py-5 border-b border-gray-200 bg-white/90 backdrop-blur-md shadow-sm z-10 sticky top-0">
        <div class="flex items-center gap-4">
          <button @click="isSidebarOpen = true" class="md:hidden p-2 -ml-2 text-gray-600 hover:bg-gray-100 rounded-lg transition"><MenuIcon class="w-6 h-6" /></button>
          <div>
            <h1 class="text-xl md:text-2xl font-black text-[#19462D] tracking-tight">Log Aktifitas</h1>
            <p class="text-xs md:text-sm text-gray-500 font-medium mt-0.5">Audit trail seluruh sistem (MongoDB + blockchain hash)</p>
          </div>
        </div>
      </header>

      <div class="p-4 md:p-8 flex flex-col gap-6 animate-fade">
        <section class="bg-white rounded-3xl shadow-sm border border-gray-100 p-6">
          <div class="flex flex-col sm:flex-row justify-between gap-4 mb-6">
            <h3 class="text-lg font-bold text-[#19462D] flex items-center gap-2"><ClipboardListIcon class="w-5 h-5" /> Rekam Aktifitas</h3>
            <div class="relative w-full sm:w-72">
              <SearchIcon class="w-4 h-4 absolute left-3 top-1/2 -translate-y-1/2 text-gray-400" />
              <input v-model="search" placeholder="Cari judul / pengguna / hash..." class="w-full pl-9 pr-4 py-2.5 border border-gray-200 rounded-xl outline-none focus:border-[#19462D] bg-gray-50 text-sm">
            </div>
          </div>

          <div v-if="loading" class="py-16 text-center text-gray-400 font-bold">Memuat...</div>
          <div v-else class="overflow-x-auto no-scrollbar">
            <table class="w-full text-sm text-left min-w-[800px]">
              <thead class="text-xs text-green-800 bg-green-50 uppercase tracking-widest border-b border-gray-100">
                <tr>
                  <th class="p-4 font-black rounded-tl-lg">Judul Aktifitas</th>
                  <th class="p-4 font-black">Pengguna</th>
                  <th class="p-4 font-black">Hash</th>
                  <th class="p-4 font-black">Waktu</th>
                  <th class="p-4 font-black text-center rounded-tr-lg">Detail</th>
                </tr>
              </thead>
              <tbody class="divide-y divide-gray-50">
                <tr v-if="filtered.length === 0"><td colspan="5" class="py-12 text-center text-gray-300 font-bold">Belum ada aktivitas tercatat</td></tr>
                <tr v-for="(a, i) in filtered" :key="i" class="hover:bg-gray-50/50 transition-colors">
                  <td class="p-4">
                    <p class="font-bold text-gray-800">{{ a.title }}</p>
                    <span :class="['text-[9px] font-black px-2 py-0.5 rounded uppercase', actionClass(a.action_type)]">{{ a.action_type }}</span>
                  </td>
                  <td class="p-4 font-medium text-gray-600">{{ a.username }}</td>
                  <td class="p-4">
                    <div class="flex items-center gap-2 bg-gray-100 px-3 py-1.5 rounded-lg w-fit border border-gray-200">
                      <FingerprintIcon class="w-4 h-4 text-gray-500" />
                      <span class="text-[10px] font-mono font-bold text-gray-600">{{ shortHash(a.blockchain_hash) }}</span>
                    </div>
                  </td>
                  <td class="p-4 text-gray-500 font-medium">{{ fmtDate(a.created_at) }}</td>
                  <td class="p-4 text-center">
                    <button @click="detail = a" class="w-8 h-8 rounded-lg bg-blue-50 text-blue-600 flex items-center justify-center hover:bg-blue-500 hover:text-white mx-auto transition"><EyeIcon class="w-4 h-4" /></button>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </section>
      </div>
    </main>

    <div v-if="detail" class="fixed inset-0 z-[100] flex items-center justify-center bg-black/40 backdrop-blur-sm p-4 overflow-y-auto" @click.self="detail = null">
      <div class="bg-white rounded-[32px] w-full max-w-4xl my-auto shadow-2xl animate-fade flex flex-col max-h-[90vh]">
        <div class="flex justify-between items-center p-6 border-b border-gray-100 sticky top-0 bg-white rounded-t-[32px] z-10">
          <h1 class="text-2xl font-extrabold text-[#19462D]">Detail Log Aktifitas</h1>
          <button @click="detail = null" class="p-2 hover:bg-gray-100 rounded-xl"><XIcon class="w-7 h-7 text-black" /></button>
        </div>
        <div class="p-6 overflow-y-auto flex flex-col gap-5 no-scrollbar">
          <div class="border border-gray-100 bg-[#F8FAFC] rounded-2xl p-6">
            <div class="text-xs font-extrabold tracking-widest text-[#19462D] uppercase mb-4">Informasi Utama</div>
            <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
              <Field label="Judul" :value="detail.title" />
              <Field label="Tabel Terdampak" :value="detail.table_affected" />
              <Field label="Nama Pengguna" :value="detail.username" />
              <div class="md:col-span-2"><Field label="Deskripsi" :value="detail.description" /></div>
              <Field label="Tipe Aksi" :value="detail.action_type" />
            </div>
          </div>

          <div class="border border-gray-100 bg-[#F8FAFC] rounded-2xl p-6">
            <div class="text-xs font-extrabold tracking-widest text-[#19462D] uppercase mb-4">Jejak Kredensial & Lokasi</div>
            <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
              <Field label="IP Address" :value="detail.ip_address || '-'" />
              <Field label="Koordinat (LAT)" :value="detail.lat ?? '-'" />
              <Field label="Koordinat (LONG)" :value="detail.long ?? '-'" />
              <div class="md:col-span-3"><Field label="User Agent" :value="detail.user_agent || '-'" /></div>
            </div>
          </div>

          <div class="border border-gray-100 bg-[#F8FAFC] rounded-2xl p-6">
            <div class="text-xs font-extrabold tracking-widest text-[#19462D] uppercase mb-4">Perbandingan Data (JSON)</div>
            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
              <div>
                <h4 class="text-xs font-extrabold text-gray-600 mb-2">Data Lama (Old)</h4>
                <pre class="bg-white border border-gray-100 rounded-xl p-3 text-[11px] font-mono text-gray-700 overflow-x-auto">{{ pretty(detail.old_data) }}</pre>
              </div>
              <div>
                <h4 class="text-xs font-extrabold text-gray-600 mb-2">Data Baru (New)</h4>
                <pre class="bg-white border border-gray-100 rounded-xl p-3 text-[11px] font-mono text-gray-700 overflow-x-auto">{{ pretty(detail.new_data) }}</pre>
              </div>
            </div>
          </div>

          <div class="border border-gray-100 bg-[#F8FAFC] rounded-2xl p-6">
            <div class="text-xs font-extrabold tracking-widest text-[#19462D] uppercase mb-4">Integritas Keamanan</div>
            <Field label="Blockchain Hash" :value="detail.blockchain_hash" mono />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, h } from 'vue'
import { MenuIcon, SearchIcon, EyeIcon, XIcon, FingerprintIcon, ClipboardListIcon } from 'lucide-vue-next'
import SidebarAdmin from '~/components/SidebarAdmin.vue'

// Small inline field display component
const Field = (props) => h('div', {}, [
  h('label', { class: 'block text-xs font-bold text-[#19462D] mb-2 uppercase' }, props.label),
  h('div', { class: ['w-full px-4 py-3 rounded-xl border border-gray-200 bg-white text-sm font-bold text-gray-800 break-all', props.mono ? 'font-mono' : ''] }, String(props.value ?? '-'))
])
Field.props = ['label', 'value', 'mono']

const isSidebarOpen = ref(false)
const { listActivities } = useAdmin()
const items = ref([])
const loading = ref(false)
const search = ref('')
const detail = ref(null)

onMounted(async () => {
  loading.value = true
  try { items.value = (await listActivities()) || [] } catch (e) { console.error(e) } finally { loading.value = false }
})

const filtered = computed(() => items.value.filter(a => {
  const q = search.value.toLowerCase()
  return (a.title || '').toLowerCase().includes(q) || (a.username || '').toLowerCase().includes(q) || (a.blockchain_hash || '').toLowerCase().includes(q)
}))

const shortHash = (h) => h ? (h.length > 16 ? h.slice(0, 16) + '…' : h) : '-'
const actionClass = (t) => {
  switch (t) {
    case 'CREATED': return 'bg-green-100 text-green-700'
    case 'UPDATED': return 'bg-orange-100 text-orange-600'
    case 'DELETED': return 'bg-red-100 text-red-600'
    case 'PAYMENT': return 'bg-blue-100 text-blue-700'
    default: return 'bg-gray-100 text-gray-600'
  }
}
const fmtDate = (d) => {
  const v = d?.$date || d
  return v ? new Date(v).toLocaleString('id-ID', { day: '2-digit', month: 'short', hour: '2-digit', minute: '2-digit' }) : '-'
}
const pretty = (o) => { try { return JSON.stringify(o ?? {}, null, 2) } catch { return '{}' } }
</script>

<style scoped>
.no-scrollbar::-webkit-scrollbar { display: none; }
.no-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }
.animate-fade { animation: fadeIn 0.3s ease; }
@keyframes fadeIn { from { opacity: 0; transform: translateY(10px); } to { opacity: 1; transform: translateY(0); } }
</style>
