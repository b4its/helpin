<template>
  <div class="min-h-screen bg-gradient-to-br from-[#0f2419] via-[#1a402d] to-[#0f2419] flex items-center justify-center p-4 font-sans relative overflow-hidden">
    
    <!-- Background decoration -->
    <div class="absolute inset-0 overflow-hidden pointer-events-none">
      <div class="absolute -top-40 -right-40 w-96 h-96 bg-green-400/10 rounded-full blur-3xl"></div>
      <div class="absolute -bottom-40 -left-40 w-96 h-96 bg-green-600/10 rounded-full blur-3xl"></div>
      <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[600px] h-[600px] bg-green-500/5 rounded-full blur-3xl"></div>
    </div>

    <div class="w-full max-w-md relative z-10">
      
      <!-- Brand Header -->
      <div class="text-center mb-8">
        <div class="inline-flex items-center justify-center w-20 h-20 bg-gradient-to-br from-green-400 to-green-600 rounded-3xl shadow-2xl shadow-green-500/30 mb-5">
          <LeafIcon class="w-10 h-10 text-white" />
        </div>
        <h1 class="text-4xl font-black text-white tracking-tight">HELPIN</h1>
        <p class="text-sm text-green-300/80 font-medium mt-2">Platform Koperasi Agro Terpadu</p>
      </div>

      <!-- Card -->
      <div class="bg-white/95 backdrop-blur-xl rounded-3xl shadow-2xl shadow-black/30 border border-white/20 overflow-hidden">
        
        <!-- Tabs -->
        <div class="flex border-b border-gray-100">
          <button 
            @click="activeTab = 'login'"
            :class="[
              'flex-1 py-4 text-sm font-black uppercase tracking-widest transition-all duration-200',
              activeTab === 'login' 
                ? 'text-[#1a402d] bg-green-50/80 border-b-2 border-[#1a402d]' 
                : 'text-gray-400 hover:text-gray-600 hover:bg-gray-50/50'
            ]"
          >
            Masuk
          </button>
          <button 
            @click="activeTab = 'register'"
            :class="[
              'flex-1 py-4 text-sm font-black uppercase tracking-widest transition-all duration-200',
              activeTab === 'register' 
                ? 'text-[#1a402d] bg-green-50/80 border-b-2 border-[#1a402d]' 
                : 'text-gray-400 hover:text-gray-600 hover:bg-gray-50/50'
            ]"
          >
            Daftar
          </button>
        </div>

        <div class="p-8">

          <!-- LOGIN FORM -->
          <form v-if="activeTab === 'login'" @submit.prevent="handleLogin" class="space-y-5">
            <div>
              <label class="block text-xs font-black text-gray-500 uppercase tracking-wider mb-2">Email</label>
              <input 
                v-model="loginForm.email" 
                type="email" 
                required 
                placeholder="nama@email.com"
                autocomplete="email"
                class="w-full px-4 py-3.5 bg-gray-50 border border-gray-200 rounded-xl text-sm font-medium focus:outline-none focus:ring-2 focus:ring-[#1a402d]/20 focus:border-[#1a402d] transition-all placeholder:text-gray-400"
              />
            </div>
            <div>
              <label class="block text-xs font-black text-gray-500 uppercase tracking-wider mb-2">Password</label>
              <div class="relative">
                <input 
                  v-model="loginForm.password" 
                  :type="showPassword ? 'text' : 'password'" 
                  required 
                  placeholder="••••••••"
                  autocomplete="current-password"
                  class="w-full px-4 py-3.5 pr-12 bg-gray-50 border border-gray-200 rounded-xl text-sm font-medium focus:outline-none focus:ring-2 focus:ring-[#1a402d]/20 focus:border-[#1a402d] transition-all placeholder:text-gray-400"
                />
                <button 
                  type="button"
                  @click="showPassword = !showPassword"
                  class="absolute right-3 top-1/2 -translate-y-1/2 text-gray-400 hover:text-gray-600 transition p-1"
                >
                  <EyeOffIcon v-if="showPassword" class="w-4 h-4" />
                  <EyeIcon v-else class="w-4 h-4" />
                </button>
              </div>
            </div>

            <!-- Error -->
            <div v-if="errorMessage" class="flex items-start gap-2 text-sm text-red-600 font-bold bg-red-50 border border-red-200 px-4 py-3 rounded-xl">
              <AlertCircleIcon class="w-4 h-4 mt-0.5 shrink-0" />
              {{ errorMessage }}
            </div>

            <button 
              type="submit" 
              :disabled="loading"
              class="w-full py-4 bg-[#1a402d] hover:bg-[#143222] active:scale-[0.98] text-white font-black rounded-xl shadow-lg shadow-green-900/20 transition-all duration-200 disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
            >
              <LoaderIcon v-if="loading" class="w-5 h-5 animate-spin" />
              <LogInIcon v-else class="w-5 h-5" />
              <span>{{ loading ? 'Memproses...' : 'Masuk' }}</span>
            </button>

            <div class="text-center">
              <button 
                type="button" 
                @click="activeTab = 'register'"
                class="text-sm text-gray-400 hover:text-[#1a402d] font-bold transition-colors"
              >
                Belum punya akun? <span class="text-[#1a402d] underline underline-offset-2">Daftar sekarang</span>
              </button>
            </div>
          </form>

          <!-- REGISTER FORM -->
          <form v-if="activeTab === 'register'" @submit.prevent="handleRegister" class="space-y-4">
            <div>
              <label class="block text-xs font-black text-gray-500 uppercase tracking-wider mb-2">Nama Lengkap</label>
              <input 
                v-model="registerForm.name" 
                type="text" 
                required 
                placeholder="Nama lengkap Anda"
                autocomplete="name"
                class="w-full px-4 py-3.5 bg-gray-50 border border-gray-200 rounded-xl text-sm font-medium focus:outline-none focus:ring-2 focus:ring-[#1a402d]/20 focus:border-[#1a402d] transition-all placeholder:text-gray-400"
              />
            </div>
            <div>
              <label class="block text-xs font-black text-gray-500 uppercase tracking-wider mb-2">Email</label>
              <input 
                v-model="registerForm.email" 
                type="email" 
                required 
                placeholder="nama@email.com"
                autocomplete="email"
                class="w-full px-4 py-3.5 bg-gray-50 border border-gray-200 rounded-xl text-sm font-medium focus:outline-none focus:ring-2 focus:ring-[#1a402d]/20 focus:border-[#1a402d] transition-all placeholder:text-gray-400"
              />
            </div>
            <div>
              <label class="block text-xs font-black text-gray-500 uppercase tracking-wider mb-2">Password</label>
              <div class="relative">
                <input 
                  v-model="registerForm.password" 
                  :type="showRegPassword ? 'text' : 'password'" 
                  required 
                  placeholder="Minimal 8 karakter"
                  autocomplete="new-password"
                  class="w-full px-4 py-3.5 pr-12 bg-gray-50 border border-gray-200 rounded-xl text-sm font-medium focus:outline-none focus:ring-2 focus:ring-[#1a402d]/20 focus:border-[#1a402d] transition-all placeholder:text-gray-400"
                />
                <button 
                  type="button"
                  @click="showRegPassword = !showRegPassword"
                  class="absolute right-3 top-1/2 -translate-y-1/2 text-gray-400 hover:text-gray-600 transition p-1"
                >
                  <EyeOffIcon v-if="showRegPassword" class="w-4 h-4" />
                  <EyeIcon v-else class="w-4 h-4" />
                </button>
              </div>
              <!-- Password strength indicator -->
              <div v-if="registerForm.password" class="mt-2 flex gap-1">
                <div 
                  v-for="n in 4" :key="n"
                  :class="['h-1 flex-1 rounded-full transition-all', n <= passwordStrength ? strengthColors[passwordStrength - 1] : 'bg-gray-200']"
                ></div>
              </div>
              <p v-if="registerForm.password" class="text-xs mt-1 font-bold" :class="strengthTextColors[passwordStrength - 1]">
                {{ strengthLabels[passwordStrength - 1] }}
              </p>
            </div>
            <div>
              <label class="block text-xs font-black text-gray-500 uppercase tracking-wider mb-2">Peran / Role</label>
              <div class="grid grid-cols-2 gap-2">
                <button 
                  v-for="role in roles" 
                  :key="role.value"
                  type="button"
                  @click="registerForm.role = role.value"
                  :class="[
                    'flex items-center gap-2 px-3 py-3 rounded-xl border-2 text-left transition-all duration-150',
                    registerForm.role === role.value
                      ? 'border-[#1a402d] bg-green-50 text-[#1a402d]'
                      : 'border-gray-200 bg-gray-50 text-gray-600 hover:border-gray-300'
                  ]"
                >
                  <component :is="role.icon" class="w-4 h-4 shrink-0" />
                  <div>
                    <span class="text-xs font-black block">{{ role.label }}</span>
                    <span class="text-[10px] text-gray-400 font-medium">{{ role.desc }}</span>
                  </div>
                </button>
              </div>
            </div>

            <!-- Error -->
            <div v-if="errorMessage" class="flex items-start gap-2 text-sm text-red-600 font-bold bg-red-50 border border-red-200 px-4 py-3 rounded-xl">
              <AlertCircleIcon class="w-4 h-4 mt-0.5 shrink-0" />
              {{ errorMessage }}
            </div>

            <!-- Success -->
            <div v-if="successMessage" class="flex items-start gap-2 text-sm text-green-700 font-bold bg-green-50 border border-green-200 px-4 py-3 rounded-xl">
              <CheckCircleIcon class="w-4 h-4 mt-0.5 shrink-0" />
              {{ successMessage }}
            </div>

            <button 
              type="submit" 
              :disabled="loading || !registerForm.role"
              class="w-full py-4 bg-[#1a402d] hover:bg-[#143222] active:scale-[0.98] text-white font-black rounded-xl shadow-lg shadow-green-900/20 transition-all duration-200 disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2"
            >
              <LoaderIcon v-if="loading" class="w-5 h-5 animate-spin" />
              <UserPlusIcon v-else class="w-5 h-5" />
              <span>{{ loading ? 'Mendaftarkan...' : 'Buat Akun' }}</span>
            </button>

            <div class="text-center">
              <button 
                type="button" 
                @click="activeTab = 'login'"
                class="text-sm text-gray-400 hover:text-[#1a402d] font-bold transition-colors"
              >
                Sudah punya akun? <span class="text-[#1a402d] underline underline-offset-2">Masuk</span>
              </button>
            </div>
          </form>

        </div>
      </div>

      <p class="text-center text-xs text-green-300/50 font-medium mt-6">&copy; 2026 HELPIN Koperasi Agro Terpadu</p>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive, computed, onMounted } from 'vue'
import { 
  LeafIcon, LoaderIcon, LogInIcon, UserPlusIcon, EyeIcon, EyeOffIcon,
  AlertCircleIcon, CheckCircleIcon, SproutIcon, MilkIcon, ShoppingBagIcon,
  TruckIcon, ShieldIcon
} from 'lucide-vue-next'

definePageMeta({ layout: false })

const { login, register, user, initAuth } = useAuth()

// Redirect if already logged in
onMounted(async () => {
  await initAuth()
  if (user.value?.role) {
    navigateTo(getRedirectPath(user.value.role))
  }
})

const activeTab = ref('login')
const loading = ref(false)
const errorMessage = ref('')
const successMessage = ref('')
const showPassword = ref(false)
const showRegPassword = ref(false)

const loginForm = reactive({ email: '', password: '' })
const registerForm = reactive({ name: '', email: '', password: '', role: '' })

const roles = [
  { value: 'petani',   label: 'Petani',    desc: 'Panel pertanian',  icon: SproutIcon },
  { value: 'peternak', label: 'Peternak',  desc: 'Panel peternakan', icon: MilkIcon },
  { value: 'pembeli',  label: 'Pembeli',   desc: 'Belanja produk',   icon: ShoppingBagIcon },
  { value: 'supplier', label: 'Supplier',  desc: 'Kelola produk',    icon: TruckIcon },
  { value: 'admin',    label: 'Admin',     desc: 'Akses penuh',      icon: ShieldIcon },
]

// Password strength
const passwordStrength = computed(() => {
  const p = registerForm.password
  if (!p) return 0
  let s = 0
  if (p.length >= 8) s++
  if (/[A-Z]/.test(p)) s++
  if (/[0-9]/.test(p)) s++
  if (/[^A-Za-z0-9]/.test(p)) s++
  return Math.max(1, s)
})
const strengthColors   = ['bg-red-400', 'bg-orange-400', 'bg-yellow-400', 'bg-green-500']
const strengthTextColors = ['text-red-500', 'text-orange-500', 'text-yellow-600', 'text-green-600']
const strengthLabels   = ['Sangat lemah', 'Lemah', 'Sedang', 'Kuat']

const getRedirectPath = (role) => {
  const map = {
    petani:   '/panel_petani/dashboard_petani',
    peternak: '/panel_peternak/dashboard_peternak',
    admin:    '/panel_admin/dashboard_admin',
    karyawan: '/panel_karyawan/dashboard_karyawan',
    pembeli:  '/ecommerce/produk',
    supplier: '/ecommerce/produk',
  }
  return map[role] || '/'
}

const handleLogin = async () => {
  errorMessage.value = ''
  loading.value = true
  try {
    const res = await login(loginForm.email, loginForm.password)
    const role = user.value?.role || res?.role
    navigateTo(getRedirectPath(role))
  } catch (e) {
    const msg = e?.data?.message || e?.message || ''
    if (msg.toLowerCase().includes('invalid') || msg.toLowerCase().includes('wrong') || msg.toLowerCase().includes('password')) {
      errorMessage.value = 'Email atau password salah.'
    } else if (msg.toLowerCase().includes('not found') || msg.toLowerCase().includes('user')) {
      errorMessage.value = 'Akun tidak ditemukan.'
    } else {
      errorMessage.value = 'Gagal terhubung ke server. Periksa koneksi Anda.'
    }
  } finally {
    loading.value = false
  }
}

const handleRegister = async () => {
  errorMessage.value = ''
  successMessage.value = ''

  if (registerForm.password.length < 8) {
    errorMessage.value = 'Password minimal 8 karakter.'
    return
  }
  if (!registerForm.role) {
    errorMessage.value = 'Pilih peran akun Anda.'
    return
  }

  loading.value = true
  try {
    await register({
      name: registerForm.name,
      email: registerForm.email,
      password: registerForm.password,
      role: registerForm.role,
    })
    const role = user.value?.role || registerForm.role
    navigateTo(getRedirectPath(role))
  } catch (e) {
    const msg = e?.data?.message || e?.message || ''
    if (msg.toLowerCase().includes('already') || msg.toLowerCase().includes('exists') || msg.toLowerCase().includes('duplicate')) {
      errorMessage.value = 'Email sudah terdaftar. Gunakan email lain atau masuk.'
    } else if (msg) {
      errorMessage.value = `Registrasi gagal: ${msg}`
    } else {
      errorMessage.value = 'Gagal membuat akun. Periksa koneksi ke server.'
    }
  } finally {
    loading.value = false
  }
}
</script>
