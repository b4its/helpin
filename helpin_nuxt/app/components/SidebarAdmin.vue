<template>
  <div>
    <div 
      v-if="isOpen" 
      @click="$emit('close')" 
      class="fixed inset-0 bg-black/60 z-30 md:hidden backdrop-blur-sm transition-opacity"
    ></div>

    <aside 
      :class="[
        'w-64 h-screen bg-gradient-to-b from-[#0F8901] to-[#042400] text-white flex flex-col justify-between shrink-0 fixed md:relative inset-y-0 left-0 z-40 transform transition-transform duration-300 h-screen',
        isOpen ? 'translate-x-0' : '-translate-x-full md:translate-x-0'
      ]"
    >
      <div class="flex flex-col h-full overflow-hidden">
        
        <div class="px-6 md:px-8 py-8 flex justify-between items-center shrink-0">
          <div class="flex flex-col">
            <h1 class="text-3xl font-black tracking-tighter text-white">HELP<span class="text-green-400">IN</span></h1>
            <span class="text-[10px] uppercase font-bold text-white/80 tracking-[0.3em] pl-1">Services</span>
          </div>
          <button @click="$emit('close')" class="md:hidden text-white hover:text-red-400 transition">
            <XIcon class="w-6 h-6" />
          </button>
        </div>

        <div class="px-6 md:px-8 mb-6 pb-6 border-b border-[#235b3c] shrink-0">
          <div class="flex items-center gap-2 mb-2">
            <div class="w-2 h-2 rounded-full bg-blue-500 animate-pulse"></div>
            <span class="text-[10px] font-semibold text-gray-300 uppercase tracking-widest">Pengguna Aktif</span>
          </div>
          <h2 class="text-xl font-bold mb-2 text-white truncate">{{ user?.name || 'Pengguna' }}</h2>
          <span class="inline-block px-3 py-1 text-[10px] font-bold bg-green-500 text-white rounded uppercase tracking-wider">
            {{ user?.role || '—' }}
          </span>
        </div>

        <nav 
          ref="sidebarNav"
          @scroll="handleSidebarScroll"
          class="flex-1 overflow-y-auto no-scrollbar pl-4 space-y-1 mb-4"
        >
          <template v-for="menu in menus" :key="menu.name">
            
            <NuxtLink 
              v-if="!menu.sub"
              :to="menu.path"
              @click="handleMenuClick"
              :class="[
                'relative flex items-center gap-4 px-4 py-3.5 transition-all duration-300 group',
                isActive(menu.path)
                ? 'active-menu bg-[#F4FBF7] text-[#19462D] rounded-l-full font-black shadow-[-5px_0_10px_rgba(0,0,0,0.1)]' 
                : 'text-gray-300 hover:text-white font-bold hover:translate-x-1'
              ]"
            >
              <component :is="menu.icon" class="w-5 h-5 group-hover:scale-110 transition-transform" />
              <span class="tracking-tight text-sm">{{ menu.name }}</span>
            </NuxtLink>

            <div v-else class="flex flex-col">
              <button 
                @click="isAkunOpen = !isAkunOpen"
                class="relative flex items-center justify-between w-full px-4 py-3.5 text-gray-300 hover:text-white font-bold transition-all"
              >
                <div class="flex items-center gap-4">
                  <component :is="menu.icon" class="w-5 h-5" />
                  <span class="tracking-tight text-sm">{{ menu.name }}</span>
                </div>
                <ChevronDownIcon :class="['w-4 h-4 transition-transform', isAkunOpen ? 'rotate-180' : '']" />
              </button>
              
              <div v-show="isAkunOpen" class="pl-8 flex flex-col space-y-1 mt-1">
                <NuxtLink 
                  v-for="sub in menu.sub" :key="sub.name"
                  :to="sub.path"
                  @click="handleMenuClick"
                  :class="[
                    'relative flex items-center gap-3 px-4 py-2.5 transition-all duration-300',
                    isActive(sub.path)
                    ? 'active-menu bg-[#F4FBF7] text-[#19462D] rounded-l-full font-black' 
                    : 'text-gray-400 hover:text-white font-semibold hover:translate-x-1'
                  ]"
                >
                  <component :is="sub.icon" class="w-4 h-4" />
                  <span class="text-xs">{{ sub.name }}</span>
                </NuxtLink>
              </div>
            </div>
          </template>
        </nav>

        <div class="p-6 shrink-0 bg-[#143D25] mt-auto flex items-center justify-between">
          <div>
            <span class="text-[10px] text-gray-400 font-bold uppercase block tracking-wider">Logged In As</span>
            <p class="text-sm font-bold text-white truncate">{{ user?.name || 'Pengguna' }}</p>
          </div>
          <button @click="onLogout" class="text-red-400 hover:text-red-500 hover:scale-110 transition">
            <LogOutIcon class="w-5 h-5" />
          </button>
        </div>

      </div>
    </aside>
  </div>
</template>

<script setup>
import { ref, onMounted, watch, nextTick } from 'vue'
import { useRoute } from 'vue-router'
import { useState } from '#app'
import { 
  LayoutDashboardIcon, UsersIcon, UserCircleIcon, UserIcon, 
  TruckIcon, BoxIcon, ShoppingCartIcon, ReceiptIcon, 
  WalletIcon, ActivityIcon, StoreIcon, XIcon, LogOutIcon, ChevronDownIcon,
  RefreshCwIcon // <-- Ikon baru untuk menu Hybrid
} from 'lucide-vue-next'

const props = defineProps({ isOpen: Boolean })
const emit = defineEmits(['close'])

const { user, logout } = useAuth()
const onLogout = () => logout()

const route = useRoute()
const isAkunOpen = ref(true)

// Kunci utama penahan posisi scroll di simpan di state global Nuxt
const sidebarNav = ref(null)
const savedScrollTop = useState('adminSidebarScrollPosition', () => 0)

// Fungsi untuk merekam posisi scroll secara real-time saat discroll oleh user
const handleSidebarScroll = (event) => {
  savedScrollTop.value = event.target.scrollTop
}

// Kembalikan posisi scroll tepat setelah komponen dimuat di layar
onMounted(() => {
  if (sidebarNav.value) {
    sidebarNav.value.scrollTop = savedScrollTop.value
  }
})

// Dengar perubahan rute jalan, paksa posisi container tetep ngestay di koordinat pixel terakhir
watch(() => route.path, () => {
  nextTick(() => {
    if (sidebarNav.value) {
      sidebarNav.value.scrollTop = savedScrollTop.value
    }
  })
})

const menus = [
  { name: 'Dashboard', icon: LayoutDashboardIcon, path: '/panel_admin/dashboard_admin' },
  { 
    name: 'Akun', icon: UsersIcon, 
    sub: [
      { name: 'Karyawan', icon: UserIcon, path: '/panel_admin/karyawan_admin' },
        { name: 'Pengguna', icon: UserCircleIcon, path: '/panel_admin/pengguna_admin' }
    ]
  },
  { name: 'Supplier', icon: TruckIcon, path: '/panel_admin/supplier_admin' },
  { name: 'Produk', icon: BoxIcon, path: '/panel_admin/produk_admin' },
  { name: 'Kasir', icon: ShoppingCartIcon, path: '/panel_admin/kasir_admin' },
  { name: 'Transaksi', icon: ReceiptIcon, path: '/panel_admin/transaksi_admin' },
  { name: 'Kas', icon: WalletIcon, path: '/panel_admin/kas_admin' },
  { name: 'Aktifitas', icon: ActivityIcon, path: '/panel_admin/aktifitas_admin' },
  { name: 'Custom Ecomerce', icon: StoreIcon, path: '/panel_admin/ecommerce_admin' },
  { name: 'Hybrid', icon: RefreshCwIcon, path: '/panel_hybrid/dashboard' } // <-- Tambahan Menu Hybrid
]

const isActive = (path) => {
  return route.path === path || route.path === path.toLowerCase();
}

const handleMenuClick = () => {
  if (window.innerWidth < 768) {
    emit('close')
  }
}
</script>

<style scoped>
.no-scrollbar::-webkit-scrollbar { display: none; }
.no-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }
.active-menu::before {
  content: ""; position: absolute; right: 0; top: -24px; width: 24px; height: 24px;
  background-color: transparent; border-bottom-right-radius: 24px;
  box-shadow: 12px 12px 0 12px #F4FBF7; pointer-events: none;
}
.active-menu::after {
  content: ""; position: absolute; right: 0; bottom: -24px; width: 24px; height: 24px;
  background-color: transparent; border-top-right-radius: 24px;
  box-shadow: 12px -12px 0 12px #F4FBF7; pointer-events: none;
}
</style>