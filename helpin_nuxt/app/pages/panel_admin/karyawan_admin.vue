<template>
  <div class="flex h-screen w-full bg-[#F4FBF7] font-sans overflow-hidden relative">
    <SidebarAdmin :isOpen="isSidebarOpen" @close="isSidebarOpen = false" />
    <main class="flex-1 flex flex-col overflow-y-auto relative w-full no-scrollbar">
      <header class="flex justify-between items-center px-6 md:px-10 py-5 border-b border-gray-200 bg-white/90 backdrop-blur-md shadow-sm z-10 sticky top-0">
        <div class="flex items-center gap-4">
          <button @click="isSidebarOpen = true" class="md:hidden p-2 -ml-2 text-gray-600 hover:bg-gray-100 rounded-lg transition"><MenuIcon class="w-6 h-6" /></button>
          <div>
            <h1 class="text-xl md:text-2xl font-black text-[#19462D] tracking-tight">Akun Karyawan</h1>
            <p class="text-xs md:text-sm text-gray-500 font-medium mt-0.5">Manajemen akun karyawan (role: karyawan)</p>
          </div>
        </div>
      </header>

      <div class="p-4 md:p-8 flex flex-col gap-6 animate-fade">
        <div class="flex justify-end">
          <button @click="openModal()" class="bg-[#19462D] text-white px-6 py-3 rounded-xl font-bold shadow-lg flex items-center gap-2 hover:bg-[#113620] transition">
            <PlusIcon class="w-5 h-5" /> Tambah Karyawan
          </button>
        </div>

        <section class="bg-white rounded-3xl shadow-sm border border-gray-100 p-6">
          <div v-if="loading" class="py-16 text-center text-gray-400 font-bold">Memuat...</div>
          <div v-else class="overflow-x-auto no-scrollbar">
            <table class="w-full text-sm text-left min-w-[700px]">
              <thead class="text-xs text-green-800 bg-green-50 uppercase tracking-widest border-b border-gray-100">
                <tr>
                  <th class="p-4 font-black rounded-tl-lg">Nama</th>
                  <th class="p-4 font-black">Email</th>
                  <th class="p-4 font-black">Kontak</th>
                  <th class="p-4 font-black text-center rounded-tr-lg">Aksi</th>
                </tr>
              </thead>
              <tbody class="divide-y divide-gray-50">
                <tr v-if="list.length === 0"><td colspan="4" class="py-12 text-center text-gray-300 font-bold">Belum ada karyawan</td></tr>
                <tr v-for="k in list" :key="k.id" class="hover:bg-gray-50/50 transition-colors">
                  <td class="p-4 font-bold text-gray-800">{{ k.name }}</td>
                  <td class="p-4 text-gray-600">{{ k.email }}</td>
                  <td class="p-4 text-gray-600">{{ k.phone || '-' }}</td>
                  <td class="p-4 text-center">
                    <div class="flex justify-center gap-2">
                      <button @click="openModal(k)" class="w-8 h-8 rounded-lg bg-yellow-50 text-yellow-600 flex items-center justify-center hover:bg-yellow-500 hover:text-white transition"><EditIcon class="w-4 h-4" /></button>
                      <button @click="remove(k)" class="w-8 h-8 rounded-lg bg-red-50 text-red-600 flex items-center justify-center hover:bg-red-500 hover:text-white transition"><Trash2Icon class="w-4 h-4" /></button>
                    </div>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </section>
      </div>
    </main>

    <div v-if="modal" class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50 backdrop-blur-sm" @click.self="modal = false">
      <div class="bg-white rounded-3xl w-full max-w-lg shadow-2xl p-8 animate-fade">
        <div class="flex justify-between items-center mb-6">
          <h2 class="text-xl font-black text-[#19462D]">{{ editId ? 'Edit Karyawan' : 'Tambah Karyawan' }}</h2>
          <button @click="modal = false" class="text-gray-400 hover:text-red-500"><XIcon class="w-6 h-6" /></button>
        </div>
        <div class="space-y-4">
          <div><label class="block text-xs font-bold text-[#19462D] mb-1.5 uppercase">Nama Lengkap</label><input v-model="form.name" class="w-full p-3 border border-gray-200 rounded-xl bg-gray-50 text-sm" /></div>
          <div v-if="!editId"><label class="block text-xs font-bold text-[#19462D] mb-1.5 uppercase">Email</label><input v-model="form.email" type="email" class="w-full p-3 border border-gray-200 rounded-xl bg-gray-50 text-sm" /></div>
          <div><label class="block text-xs font-bold text-[#19462D] mb-1.5 uppercase">Kontak</label><input v-model="form.phone" class="w-full p-3 border border-gray-200 rounded-xl bg-gray-50 text-sm" /></div>
          <div><label class="block text-xs font-bold text-[#19462D] mb-1.5 uppercase">{{ editId ? 'Password Baru (opsional)' : 'Password (min 8)' }}</label><input v-model="form.password" type="password" class="w-full p-3 border border-gray-200 rounded-xl bg-gray-50 text-sm" /></div>
        </div>
        <div class="flex gap-3 mt-6">
          <button @click="modal = false" class="flex-1 py-3 bg-gray-100 text-gray-500 rounded-xl font-black uppercase text-xs">Batal</button>
          <button @click="save" :disabled="saving" class="flex-1 py-3 bg-[#19462D] text-white rounded-xl font-black uppercase text-xs disabled:opacity-50">{{ saving ? '...' : 'Simpan' }}</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive, onMounted } from 'vue'
import { MenuIcon, PlusIcon, EditIcon, Trash2Icon, XIcon } from 'lucide-vue-next'
import SidebarAdmin from '~/components/SidebarAdmin.vue'

const isSidebarOpen = ref(false)
const { listEmployees, createEmployee, updateEmployee, deleteEmployee } = useAdmin()
const toast = useToast()
const list = ref([])
const loading = ref(false)
const saving = ref(false)
const modal = ref(false)
const editId = ref(null)
const form = reactive({ name: '', email: '', phone: '', password: '' })

const load = async () => {
  loading.value = true
  try { list.value = (await listEmployees()) || [] } catch (e) { console.error(e); toast.error('Gagal memuat karyawan', e?.data?.message) } finally { loading.value = false }
}
onMounted(load)

const openModal = (k = null) => {
  if (k) { editId.value = k.id; Object.assign(form, { name: k.name, email: k.email, phone: k.phone || '', password: '' }) }
  else { editId.value = null; Object.assign(form, { name: '', email: '', phone: '', password: '' }) }
  modal.value = true
}

const save = async () => {
  if (!form.name) { toast.warning('Data belum lengkap', 'Nama wajib diisi'); return }
  saving.value = true
  try {
    if (editId.value) {
      await updateEmployee(editId.value, { name: form.name, phone: form.phone, password: form.password || undefined })
    } else {
      if (!form.email || form.password.length < 8) { toast.warning('Data belum lengkap', 'Email wajib & password minimal 8 karakter'); saving.value = false; return }
      await createEmployee({ name: form.name, email: form.email, phone: form.phone, password: form.password })
    }
    modal.value = false
    toast.success(editId.value ? 'Karyawan diperbarui' : 'Karyawan ditambahkan', form.name)
    await load()
  } catch (e) { console.error(e); toast.error('Gagal menyimpan karyawan', e?.data?.error?.message || e?.data?.message) }
  finally { saving.value = false }
}

const remove = async (k) => {
  if (!confirm(`Hapus karyawan ${k.name}?`)) return
  try { await deleteEmployee(k.id); toast.success('Karyawan dihapus', k.name); await load() } catch (e) { toast.error('Gagal menghapus karyawan', e?.data?.message) }
}
</script>

<style scoped>
.no-scrollbar::-webkit-scrollbar { display: none; }
.no-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }
.animate-fade { animation: fadeIn 0.3s ease; }
@keyframes fadeIn { from { opacity: 0; transform: translateY(10px); } to { opacity: 1; transform: translateY(0); } }
</style>
