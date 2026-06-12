<template>
  <div class="min-h-screen bg-gradient-to-br from-[#f4f7f5] to-[#e1f0e5] flex items-center justify-center p-4 font-sans">
    <div class="w-full max-w-md">
      
      <div class="text-center mb-8">
        <div class="inline-flex items-center justify-center w-16 h-16 bg-[#1a402d] rounded-2xl shadow-xl shadow-green-900/20 mb-4">
          <LeafIcon class="w-8 h-8 text-green-400" />
        </div>
        <h1 class="text-3xl font-black text-gray-800 tracking-tight">HELPIN</h1>
        <p class="text-sm text-gray-500 font-medium mt-1">Platform Koperasi Agro Terpadu</p>
      </div>

      <div class="bg-white rounded-3xl shadow-xl border border-gray-100 overflow-hidden">
        
        <div class="flex border-b border-gray-100">
          <button 
            @click="activeTab = 'login'"
            :class="['flex-1 py-4 text-sm font-black uppercase tracking-widest transition-colors', activeTab === 'login' ? 'text-[#1a402d] border-b-2 border-[#1a402d] bg-green-50/50' : 'text-gray-400 hover:text-gray-600']"
          >
            Masuk
          </button>
          <button 
            @click="activeTab = 'register'"
            :class="['flex-1 py-4 text-sm font-black uppercase tracking-widest transition-colors', activeTab === 'register' ? 'text-[#1a402d] border-b-2 border-[#1a402d] bg-green-50/50' : 'text-gray-400 hover:text-gray-600']"
          >
            Daftar
          </button>
        </div>

        <div class="p-8">
          
          <form v-if="activeTab === 'login'" @submit.prevent="handleLogin" class="space-y-5">
            <div>
              <label class="block text-xs font-black text-gray-500 uppercase tracking-wider mb-2">Email</label>
              <input 
                v-model="loginForm.email" 
                type="email" 
                required 
                placeholder="nama@email.com"
                class="w-full px-4 py-3.5 bg-gray-50 border border-gray-200 rounded-xl text-sm font-medium focus:outline-none focus:ring-2 focus:ring-[#1a402d]/20 focus:border-[#1a402d] transition-all"
              />
            </div>
            <div>
              <label class="block text-xs font-black text-gray-500 uppercase tracking-wider mb-2">Password</label>
              <input 
                v-model="loginForm.password" 
                type="password" 
                required 
                placeholder="••••••••"
                class="w-full px-4 py-3.5 bg-gray-50 border border-gray-200 rounded-xl text-sm font-medium focus:outline-none focus:ring-2 focus:ring-[#1a402d]/20 focus:border-[#1a402d] transition-all"
              />
            </div>

            <p v-if="errorMessage" class="text-sm text-red-500 font-bold bg-red-50 px-4 py-2 rounded-lg">{{ errorMessage }}</p>

            <button 
              type="submit" 
              :disabled="loading"
              class="w-full py-4 bg-[#1a402d] hover:bg-[#143222] text-white font-black rounded-xl shadow-lg shadow-green-900/20 transition-all disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
            >
              <LoaderIcon v-if="loading" class="w-5 h-5 animate-spin" />
              <span>{{ loading ? 'Memproses...' : 'Masuk' }}</span>
            </button>
          </form>

          <form v-if="activeTab === 'register'" @submit.prevent="handleRegister" class="space-y-5">
            <div>
              <label class="block text-xs font-black text-gray-500 uppercase tracking-wider mb-2">Nama Lengkap</label>
              <input 
                v-model="registerForm.name" 
                type="text" 
                required 
                placeholder="Nama lengkap Anda"
                class="w-full px-4 py-3.5 bg-gray-50 border border-gray-200 rounded-xl text-sm font-medium focus:outline-none focus:ring-2 focus:ring-[#1a402d]/20 focus:border-[#1a402d] transition-all"
              />
            </div>
            <div>
              <label class="block text-xs font-black text-gray-500 uppercase tracking-wider mb-2">Email</label>
              <input 
                v-model="registerForm.email" 
                type="email" 
                required 
                placeholder="nama@email.com"
                class="w-full px-4 py-3.5 bg-gray-50 border border-gray-200 rounded-xl text-sm font-medium focus:outline-none focus:ring-2 focus:ring-[#1a402d]/20 focus:border-[#1a402d] transition-all"
              />
            </div>
            <div>
              <label class="block text-xs font-black text-gray-500 uppercase tracking-wider mb-2">Password</label>
              <input 
                v-model="registerForm.password" 
                type="password" 
                required 
                placeholder="Minimal 8 karakter"
                class="w-full px-4 py-3.5 bg-gray-50 border border-gray-200 rounded-xl text-sm font-medium focus:outline-none focus:ring-2 focus:ring-[#1a402d]/20 focus:border-[#1a402d] transition-all"
              />
            </div>
            <div>
              <label class="block text-xs font-black text-gray-500 uppercase tracking-wider mb-2">Peran</label>
              <select 
                v-model="registerForm.role" 
                required
                class="w-full px-4 py-3.5 bg-gray-50 border border-gray-200 rounded-xl text-sm font-medium focus:outline-none focus:ring-2 focus:ring-[#1a402d]/20 focus:border-[#1a402d] transition-all"
              >
                <option value="" disabled>Pilih peran Anda</option>
                <option value="petani">Petani</option>
                <option value="peternak">Peternak</option>
                <option value="pembeli">Pembeli</option>
                <option value="supplier">Supplier</option>
                <option value="admin">Admin Koperasi</option>
              </select>
            </div>

            <p v-if="errorMessage" class="text-sm text-red-500 font-bold bg-red-50 px-4 py-2 rounded-lg">{{ errorMessage }}</p>

            <button 
              type="submit" 
              :disabled="loading"
              class="w-full py-4 bg-[#1a402d] hover:bg-[#143222] text-white font-black rounded-xl shadow-lg shadow-green-900/20 transition-all disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
            >
              <LoaderIcon v-if="loading" class="w-5 h-5 animate-spin" />
              <span>{{ loading ? 'Memproses...' : 'Daftar Akun' }}</span>
            </button>
          </form>

        </div>
      </div>

      <p class="text-center text-xs text-gray-400 font-medium mt-6">&copy; 2026 HELPIN Koperasi Agro Terpadu</p>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive } from 'vue'
import { LeafIcon, LoaderIcon } from 'lucide-vue-next'

const { login, register, user } = useAuth()

const activeTab = ref('login')
const loading = ref(false)
const errorMessage = ref('')

const loginForm = reactive({
  email: '',
  password: ''
})

const registerForm = reactive({
  name: '',
  email: '',
  password: '',
  role: ''
})

const getRedirectPath = (role) => {
  const routes = {
    petani: '/panel_petani/dashboard_petani',
    peternak: '/panel_peternak/dashboard_peternak',
    admin: '/panel_hybrid/kasir',
    supplier: '/panel_hybrid/kasir',
    pembeli: '/ecommerce/produk'
  }
  return routes[role] || '/'
}

const handleLogin = async () => {
  errorMessage.value = ''
  loading.value = true
  try {
    const res = await login(loginForm.email, loginForm.password)
    const role = user.value?.role || res.role
    const redirectPath = getRedirectPath(role)
    navigateTo(redirectPath)
  } catch (e) {
    console.error('Login failed:', e)
    errorMessage.value = 'Email atau password salah. Silakan coba lagi.'
  } finally {
    loading.value = false
  }
}

const handleRegister = async () => {
  errorMessage.value = ''
  if (registerForm.password.length < 8) {
    errorMessage.value = 'Password minimal 8 karakter.'
    return
  }
  if (!registerForm.role) {
    errorMessage.value = 'Silakan pilih peran Anda.'
    return
  }
  
  loading.value = true
  try {
    await register({
      name: registerForm.name,
      email: registerForm.email,
      password: registerForm.password,
      role: registerForm.role
    })
    const role = user.value?.role || registerForm.role
    const redirectPath = getRedirectPath(role)
    navigateTo(redirectPath)
  } catch (e) {
    console.error('Register failed:', e)
    // Show actual error from backend if available
    const serverMsg = e?.data?.error?.message || e?.message || ''
    if (serverMsg.includes('already')) {
      errorMessage.value = 'Email sudah terdaftar. Gunakan email lain.'
    } else if (serverMsg) {
      errorMessage.value = `Registrasi gagal: ${serverMsg}`
    } else {
      errorMessage.value = 'Registrasi gagal. Periksa koneksi ke server.'
    }
  } finally {
    loading.value = false
  }
}
</script>
