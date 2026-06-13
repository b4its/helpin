<template>
  <div class="flex h-screen w-full bg-[#F4FBF7] font-sans overflow-hidden relative">
    <SidebarAdmin :isOpen="isSidebarOpen" @close="isSidebarOpen = false" />
    <main class="flex-1 flex flex-col overflow-y-auto relative w-full no-scrollbar">
      <header class="flex justify-between items-center px-6 md:px-10 py-5 border-b border-gray-200 bg-white/90 backdrop-blur-md shadow-sm z-10 sticky top-0">
        <div class="flex items-center gap-4">
          <button @click="isSidebarOpen = true" class="md:hidden p-2 -ml-2 text-gray-600 hover:bg-gray-100 rounded-lg transition"><MenuIcon class="w-6 h-6" /></button>
          <div>
            <h1 class="text-xl md:text-2xl font-black text-[#19462D] tracking-tight">Akun Pengguna</h1>
            <p class="text-xs md:text-sm text-gray-500 font-medium mt-0.5">Seluruh pengguna terdaftar & perannya</p>
          </div>
        </div>
      </header>

      <div class="p-4 md:p-8 flex flex-col gap-6 animate-fade">
        <section class="grid grid-cols-2 md:grid-cols-4 gap-4">
          <div v-for="s in roleStats" :key="s.label" class="bg-white p-5 rounded-2xl border border-gray-100 shadow-sm">
            <p class="text-[10px] font-black text-gray-400 uppercase">{{ s.label }}</p>
            <p class="text-2xl font-black text-[#19462D]">{{ s.value }}</p>
          </div>
        </section>

        <section class="bg-white rounded-3xl shadow-sm border border-gray-100 p-6">
          <div class="flex flex-col sm:flex-row justify-between gap-4 mb-6">
            <h3 class="text-lg font-bold text-[#19462D] flex items-center gap-2"><div class="w-2 h-2 rounded-full bg-[#19462D]"></div> Daftar Pengguna</h3>
            <div class="relative w-full sm:w-64">
              <SearchIcon class="w-4 h-4 absolute left-3 top-1/2 -translate-y-1/2 text-gray-400" />
              <input v-model="search" placeholder="Cari nama / email..." class="w-full pl-9 pr-4 py-2.5 border border-gray-200 rounded-xl outline-none focus:border-[#19462D] bg-gray-50 text-sm" />
            </div>
          </div>

          <div v-if="loading" class="py-16 text-center text-gray-400 font-bold">Memuat...</div>
          <div v-else class="overflow-x-auto no-scrollbar">
            <table class="w-full text-sm text-left min-w-[700px]">
              <thead class="text-xs text-green-800 bg-green-50 uppercase tracking-widest border-b border-gray-100">
                <tr>
                  <th class="p-4 font-black rounded-tl-lg">Nama</th>
                  <th class="p-4 font-black">Email</th>
                  <th class="p-4 font-black">Kontak</th>
                  <th class="p-4 font-black">Role</th>
                  <th class="p-4 font-black text-center rounded-tr-lg">Detail</th>
                </tr>
              </thead>
              <tbody class="divide-y divide-gray-50">
                <tr v-if="filtered.length === 0"><td colspan="5" class="py-12 text-center text-gray-300 font-bold">Tidak ada data</td></tr>
                <tr v-for="u in filtered" :key="u.id" class="hover:bg-gray-50/50 transition-colors">
                  <td class="p-4 font-bold text-gray-800">{{ u.name }}</td>
                  <td class="p-4 text-gray-600">{{ u.email }}</td>
                  <td class="p-4 text-gray-600">{{ u.phone || '-' }}</td>
                  <td class="p-4"><span :class="['px-3 py-1 rounded-full text-[10px] font-black uppercase', roleClass(u.role)]">{{ roleLabel(u.role) }}</span></td>
                  <td class="p-4 text-center">
                    <button @click="detail = u" class="w-8 h-8 rounded-lg bg-blue-50 text-blue-600 flex items-center justify-center hover:bg-blue-500 hover:text-white mx-auto transition"><EyeIcon class="w-4 h-4" /></button>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </section>
      </div>
    </main>

    <div v-if="detail" class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50 backdrop-blur-sm" @click.self="detail = null">
      <div class="bg-white rounded-3xl w-full max-w-md shadow-2xl p-8 animate-fade">
        <div class="flex justify-between items-center mb-6">
          <h2 class="text-xl font-black text-[#19462D]">Detail Pengguna</h2>
          <button @click="detail = null" class="text-gray-400 hover:text-red-500"><XIcon class="w-6 h-6" /></button>
        </div>
        <div class="space-y-3 text-sm">
          <div class="flex justify-between"><span class="text-gray-400 font-bold">Nama</span><span class="font-black text-gray-800">{{ detail.name }}</span></div>
          <div class="flex justify-between"><span class="text-gray-400 font-bold">Email</span><span class="font-bold text-gray-700">{{ detail.email }}</span></div>
          <div class="flex justify-between"><span class="text-gray-400 font-bold">Kontak</span><span class="font-bold text-gray-700">{{ detail.phone || '-' }}</span></div>
          <div class="flex justify-between items-center"><span class="text-gray-400 font-bold">Role</span><span :class="['px-3 py-1 rounded-full text-[10px] font-black uppercase', roleClass(detail.role)]">{{ roleLabel(detail.role) }}</span></div>
          <div class="flex justify-between"><span class="text-gray-400 font-bold">Terdaftar</span><span class="font-bold text-gray-700">{{ formatDate(detail.created_at) }}</span></div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { MenuIcon, SearchIcon, EyeIcon, XIcon } from 'lucide-vue-next'
import SidebarAdmin from '~/components/SidebarAdmin.vue'

const isSidebarOpen = ref(false)
const { listUsers } = useAdmin()
const users = ref([])
const loading = ref(false)
const search = ref('')
const detail = ref(null)

onMounted(async () => {
  loading.value = true
  try { users.value = (await listUsers()) || [] } catch (e) { console.error(e) } finally { loading.value = false }
})

const filtered = computed(() => users.value.filter(u =>
  u.name?.toLowerCase().includes(search.value.toLowerCase()) || u.email?.toLowerCase().includes(search.value.toLowerCase())
))

const roleLabel = (r) => r && r.trim() ? r : 'Tidak ada'
const roleClass = (r) => {
  switch (r) {
    case 'admin': return 'bg-purple-100 text-purple-700'
    case 'karyawan': return 'bg-blue-100 text-blue-700'
    case 'petani': return 'bg-green-100 text-green-700'
    case 'peternak': return 'bg-orange-100 text-orange-700'
    case 'supplier': return 'bg-cyan-100 text-cyan-700'
    case 'pembeli': return 'bg-gray-100 text-gray-600'
    default: return 'bg-gray-100 text-gray-400'
  }
}
const roleStats = computed(() => {
  const c = (r) => users.value.filter(u => u.role === r).length
  return [
    { label: 'Total Pengguna', value: users.value.length },
    { label: 'Petani', value: c('petani') },
    { label: 'Peternak', value: c('peternak') },
    { label: 'Pembeli/Lainnya', value: c('pembeli') },
  ]
})
const formatDate = (d) => d ? new Date(d).toLocaleDateString('id-ID', { day: '2-digit', month: 'short', year: 'numeric' }) : '-'
</script>

<style scoped>
.no-scrollbar::-webkit-scrollbar { display: none; }
.no-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }
.animate-fade { animation: fadeIn 0.3s ease; }
@keyframes fadeIn { from { opacity: 0; transform: translateY(10px); } to { opacity: 1; transform: translateY(0); } }
</style>
