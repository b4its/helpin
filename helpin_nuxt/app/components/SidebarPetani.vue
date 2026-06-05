<template>
  <div>
    <div 
      v-if="isOpen" 
      @click="$emit('close')" 
      class="fixed inset-0 bg-black/60 z-30 md:hidden backdrop-blur-sm transition-opacity"
    ></div>

    <aside 
      :class="[
        'w-[280px] bg-[#1a402d] text-white flex flex-col justify-between shrink-0 fixed md:relative inset-y-0 left-0 z-40 transform transition-transform duration-300 h-screen',
        isOpen ? 'translate-x-0' : '-translate-x-full md:translate-x-0'
      ]"
    >
      <div class="flex flex-col h-full overflow-hidden">
        <div class="px-6 md:px-8 py-8 flex justify-between items-center shrink-0">
          <div class="flex items-baseline gap-1">
            <h1 class="text-3xl font-black tracking-tighter text-white">HELPIN</h1>
            <span class="text-[10px] uppercase font-bold text-green-400 tracking-widest">Services</span>
          </div>
          <button @click="$emit('close')" class="md:hidden text-white hover:text-red-400 transition">
            <XIcon class="w-6 h-6" />
          </button>
        </div>

        <div class="px-6 md:px-8 mb-6 border-b border-[#23533b] pb-6 bg-[#173a28] shrink-0">
          <div class="flex items-center gap-2 mb-2">
            <div class="w-2 h-2 rounded-full bg-green-500 animate-pulse"></div>
            <span class="text-xs font-semibold text-green-400 uppercase tracking-widest">Petani Aktif</span>
          </div>
          <h2 class="text-xl font-bold mb-2 text-white">Petani Suki</h2>
          <span class="inline-block px-3 py-1 text-[10px] font-bold border border-green-600 text-green-400 rounded uppercase">
            Petani
          </span>
        </div>

        <nav class="flex-1 overflow-y-auto no-scrollbar pl-4 space-y-1">
          <NuxtLink 
            v-for="menu in menus" 
            :key="menu.name" 
            :to="menu.path"
            :class="[
              'relative flex items-center gap-4 px-4 py-4 transition-all duration-300 group',
              isActive(menu.path)
              ? 'active-menu bg-[#f4f7f5] text-[#1a402d] rounded-l-full font-black shadow-[-5px_0_10px_rgba(0,0,0,0.1)]' 
              : 'text-gray-400 hover:text-white font-bold hover:translate-x-1'
            ]"
          >
            <component :is="menu.icon" class="w-5 h-5 group-hover:scale-110 transition-transform" />
            <span class="tracking-tight">{{ menu.name }}</span>
          </NuxtLink>
        </nav>

        <div class="p-6 shrink-0 mt-auto">
          <div class="bg-[#143222] rounded-xl p-4 flex items-center justify-between border border-white/5 shadow-inner">
            <div class="min-w-0 text-white">
              <span class="text-[10px] text-gray-500 font-black uppercase block tracking-tighter">Authorized Admin</span>
              <p class="text-sm font-bold truncate pr-2 italic">Admin Suki SUPER</p>
            </div>
            <LogOutIcon class="w-5 h-5 text-red-400 cursor-pointer hover:scale-110 transition" />
          </div>
        </div>
      </div>
    </aside>
  </div>
</template>

<script setup>
import { 
  LayoutDashboardIcon, ArchiveIcon, MapIcon, WheatIcon, 
  HistoryIcon, XIcon, LogOutIcon, 
  HomeIcon
} from 'lucide-vue-next'

defineProps({
  isOpen: Boolean
})

defineEmits(['close'])

const route = useRoute()

// DAFTAR MENU TERPUSAT UNTUK PANEL PETANI
const menus = [
  { name: 'Dashboard', icon: LayoutDashboardIcon, path: '/panel_petani/dashboard_petani' },
  { name: 'Inventori', icon: ArchiveIcon, path: '/panel_petani/inventori_petani' },
  { name: 'Lahan', icon: MapIcon, path: '/panel_petani/lahan_petani' },
  { name: 'Hasil Panen', icon: WheatIcon, path: '/panel_petani/hasil_panen_petani' },
  { name: 'Riwayat Panen', icon: HistoryIcon, path: '/panel_petani/riwayat_panen_petani' },
  { name: 'Landing Page', icon: HomeIcon, path: '/' },
]

// Deteksi otomatis halaman aktif berdasarkan URL
const isActive = (path) => {
  return route.path === path
}
</script>

<style scoped>
.no-scrollbar::-webkit-scrollbar { display: none; }
.no-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }

/* Inverted Border Radius - Efek lengkungan mewah di menu aktif */
.active-menu::before {
  content: ""; position: absolute; right: 0; top: -24px; width: 24px; height: 24px;
  background-color: transparent; border-bottom-right-radius: 24px;
  box-shadow: 12px 12px 0 12px #f4f7f5; pointer-events: none;
}

.active-menu::after {
  content: ""; position: absolute; right: 0; bottom: -24px; width: 24px; height: 24px;
  background-color: transparent; border-top-right-radius: 24px;
  box-shadow: 12px -12px 0 12px #f4f7f5; pointer-events: none;
}
</style>