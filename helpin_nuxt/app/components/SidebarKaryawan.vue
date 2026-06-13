<template>
  <div>
    <aside :class="['fixed inset-y-0 left-0 z-50 w-[280px] bg-[#19462D] text-white flex flex-col justify-between transition-transform duration-300 ease-in-out md:translate-x-0 md:static', isOpen ? 'translate-x-0' : '-translate-x-full']">
      
      <div class="pt-8 overflow-y-auto no-scrollbar flex-1">
        <div class="text-center mb-8">
          <h1 class="text-4xl font-black tracking-widest text-white m-0 leading-none">H<span class="relative after:content-[''] after:absolute after:top-1/2 after:left-0 after:right-0 after:h-[3px] after:bg-white after:-translate-y-1/2">E</span>LPIN</h1>
          <span class="text-[10px] tracking-[4px] opacity-80 ml-[60px]">SERVICES</span>
        </div>

        <div class="px-8 mb-8">
          <p class="text-[10px] text-white/60 flex items-center gap-2 mb-1"><span class="w-2 h-2 rounded-full bg-[#10B981]"></span> PENGGUNA AKTIF</p>
          <h2 class="text-xl font-bold text-white mb-2">Karyawan Suki</h2>
          <span class="inline-block px-3 py-1 bg-[#166534] text-white text-[10px] font-bold rounded tracking-widest">KARYAWAN</span>
        </div>

        <nav class="flex flex-col pl-5 space-y-1">
          <NuxtLink v-for="menu in menus" :key="menu.id" :to="menu.path"
            class="group relative w-full flex items-center gap-4 px-6 py-3.5 text-[15px] font-bold text-white rounded-l-full transition-all hover:bg-white/10 [&.router-link-exact-active]:bg-[#F4FBF7] [&.router-link-exact-active]:!text-[#19462D]"
          >
            
            <div class="hidden group-[.router-link-exact-active]:block absolute right-0 -top-[30px] w-[30px] h-[30px] bg-transparent rounded-br-[30px] shadow-[10px_10px_0_10px_#F4FBF7] pointer-events-none"></div>
            
            <component :is="menu.icon" class="w-5 h-5 flex-shrink-0 opacity-90 group-[.router-link-exact-active]:opacity-100" />
            <span>{{ menu.label }}</span>
            
            <div class="hidden group-[.router-link-exact-active]:block absolute right-0 -bottom-[30px] w-[30px] h-[30px] bg-transparent rounded-tr-[30px] shadow-[10px_-10px_0_10px_#F4FBF7] pointer-events-none"></div>
            
          </NuxtLink>
        </nav>
      </div>

      <div class="p-6 md:p-8 bg-[#143D25] flex items-center justify-between flex-shrink-0">
        <div>
          <p class="text-[10px] text-white/60 flex items-center gap-2 mb-1"><span class="w-2 h-2 rounded-full bg-[#10B981]"></span> LOGGED IN AS</p>
          <h3 class="text-sm font-bold text-white m-0">Admin Suki SUPER</h3>
        </div>
        <button class="p-1 hover:bg-red-500/20 rounded-lg transition">
          <LogOutIcon class="w-6 h-6 text-red-400" />
        </button>
      </div>
    </aside>

    <div v-if="isOpen" @click="$emit('close')" class="fixed inset-0 bg-black/50 z-40 md:hidden"></div>
  </div>
</template>

<script setup>
import { 
  LayoutDashboardIcon, UsersIcon, TruckIcon, PackageIcon, 
  ShoppingCartIcon, FileTextIcon, WalletIcon, LogOutIcon,
  RefreshCwIcon // <-- 1. Import ikon baru di sini
} from 'lucide-vue-next'

defineProps({ isOpen: Boolean })
defineEmits(['close'])

const menus = [
  // Pastikan 'path' di bawah ini sama PERSIS dengan nama file kamu (tanpa .vue)
  { id: 'dashboard', label: 'Dashboard', path: '/panel_karyawan/dashboard_karyawan', icon: LayoutDashboardIcon },
  { id: 'pengguna', label: 'Pengguna', path: '/panel_karyawan/pengguna_karyawan', icon: UsersIcon },
  { id: 'supplier', label: 'Supplier', path: '/panel_karyawan/supplier_karyawan', icon: TruckIcon },
  { id: 'produk', label: 'Produk', path: '/panel_karyawan/produk_karyawan', icon: PackageIcon },
  { id: 'kasir', label: 'Kasir', path: '/panel_karyawan/kasir_karyawan', icon: ShoppingCartIcon },
  { id: 'transaksi', label: 'Transaksi', path: '/panel_karyawan/transaksi_karyawan', icon: FileTextIcon },
  { id: 'kas', label: 'Kas', path: '/panel_karyawan/kas_karyawan', icon: WalletIcon },
  { id: 'hybrid', label: 'Hybrid', path: '/panel_hybrid/dashboard', icon: RefreshCwIcon } // <-- 2. Tambahkan menu Hybrid di paling bawah
]
</script>

<style scoped>
.no-scrollbar::-webkit-scrollbar { display: none; }
.no-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }
</style>