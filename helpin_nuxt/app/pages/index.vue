<script setup lang="ts">
import { ref, shallowRef, onMounted, nextTick } from 'vue'

// ==========================================
// 1. STATE & LOGIC: DROPDOWN LOGIN
// ==========================================
const isLoginDropdownOpen = ref(false)

// ==========================================
// 2. STATE & LOGIC: 3D CAROUSEL SLIDER
// ==========================================
const features = ref([
  { 
    id: 1, 
    title: 'AI Pencatatan Panen', 
    desc: 'Sistem cerdas yang otomatis merekap hasil pertanian, berat gabah, hingga distribusi logistik secara real-time langsung dari sawah.' 
  },
  { 
    id: 2, 
    title: 'AI Problem Solver', 
    desc: 'Asisten virtual berbasis machine learning yang membantu menganalisis kesehatan hewan ternak, memprediksi masa panen, serta rekomendasi pupuk.' 
  },
  { 
    id: 3, 
    title: 'Koperasi Cerdas & Transparan', 
    desc: 'Otomatisasi pembagian Sisa Hasil Usaha (SHU), manajemen kas simpan pinjam anggota, dan monitoring validasi transaksi tanpa kesalahan.' 
  }
])

const currentIndex = ref(1)

const nextSlide = () => {
  currentIndex.value = (currentIndex.value + 1) % features.value.length
}

const prevSlide = () => {
  currentIndex.value = (currentIndex.value - 1 + features.value.length) % features.value.length
}

const getSlideClass = (index: number) => {
  if (index === currentIndex.value) return 'active-slide z-20 opacity-100 center-slide pointer-events-auto'
  if (index === (currentIndex.value - 1 + features.value.length) % features.value.length) return 'prev-slide z-10 opacity-60 rotate-y-left pointer-events-auto'
  if (index === (currentIndex.value + 1) % features.value.length) return 'next-slide z-10 opacity-60 rotate-y-right pointer-events-auto'
  return 'hidden opacity-0 pointer-events-none'
}

// ==========================================
// 3. STATE & LOGIC: LEAFLET MAP (CSR ONLY)
// ==========================================
const mapContainer = ref<HTMLElement | null>(null)
const map = shallowRef<any>(null)

const koperasiLocations = [
  { 
    id: 1, name: 'KUD Tani Makmur Utama', type: 'Pertanian Padi & Palawija', 
    lat: -0.5022, lng: 117.1536, members: 145, status: 'Aktif AI', address: 'Jl. Poros Makroman, Kaltim'
  },
  { 
    id: 2, name: 'Koperasi Susu Sejahtera', type: 'Peternakan Sapi Perah', 
    lat: -0.4284, lng: 116.9853, members: 85, status: 'Aktif AI', address: 'Kawasan Agrowisata Kukar'
  },
  { 
    id: 3, name: 'KUD Agro Jaya', type: 'Perkebunan Sawit Mandiri', 
    lat: -1.2379, lng: 116.8529, members: 310, status: 'Proses Integrasi', address: 'Km. 15 Karang Joang'
  }
]

onMounted(async () => {
  if (process.client) {
    await nextTick()

    setTimeout(async () => {
      if (!mapContainer.value) return

      const L = await import('leaflet')

      delete (L.Icon.Default.prototype as any)._getIconUrl;
      L.Icon.Default.mergeOptions({
        iconRetinaUrl: 'https://cdnjs.cloudflare.com/ajax/libs/leaflet/1.9.4/images/marker-icon-2x.png',
        iconUrl: 'https://cdnjs.cloudflare.com/ajax/libs/leaflet/1.9.4/images/marker-icon.png',
        shadowUrl: 'https://cdnjs.cloudflare.com/ajax/libs/leaflet/1.9.4/images/marker-shadow.png',
      });

      map.value = L.map(mapContainer.value).setView([-0.75, 117.0], 8)

      L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
        maxZoom: 19,
        attribution: '© OpenStreetMap contributors'
      }).addTo(map.value)

      koperasiLocations.forEach(loc => {
        const marker = L.marker([loc.lat, loc.lng]).addTo(map.value)
        const popupContent = `
          <div style="min-width: 200px; font-family: 'Inter', sans-serif;">
            <h4 style="margin: 0 0 4px 0; font-size: 14px; font-weight: 800; color: #1c4532;">${loc.name}</h4>
            <p style="margin: 0 0 8px 0; font-size: 11px; color: #6b7280;">${loc.address}</p>
            <div style="border-top: 1px solid #f3f4f6; padding-top: 8px; margin-bottom: 8px;">
              <p style="margin: 0 0 4px 0; font-size: 12px; color: #374151;"><strong>Fokus:</strong> ${loc.type}</p>
              <p style="margin: 0; font-size: 12px; color: #374151;"><strong>Anggota:</strong> ${loc.members} Orang</p>
            </div>
            <span style="display: inline-block; background-color: ${loc.status === 'Aktif AI' ? '#dcfce7' : '#fef08a'}; color: ${loc.status === 'Aktif AI' ? '#166534' : '#854d0e'}; padding: 4px 8px; border-radius: 9999px; font-size: 10px; font-weight: 800; text-transform: uppercase; letter-spacing: 0.05em;">
              ${loc.status}
            </span>
          </div>
        `
        marker.bindPopup(popupContent)
      })

      setTimeout(() => {
        if (map.value) map.value.invalidateSize()
      }, 100)

    }, 150)
  }
})
</script>

<template>
  <div class="relative overflow-hidden w-full min-h-screen bg-[#f8fdf9] text-gray-800 antialiased font-sans">
    
    <div class="absolute top-0 left-0 w-full h-[900px] bg-[radial-gradient(ellipse_at_top,_var(--tw-gradient-stops))] from-green-100/70 via-[#f8fdf9] to-transparent -z-10 pointer-events-none"></div>

    <header class="container mx-auto px-6 py-5 flex items-center justify-between relative z-50">
      <div class="flex items-center gap-12">
        <div class="text-3xl font-extrabold text-[#1c4532] tracking-tighter flex items-center gap-0.5">
          HELP<span class="text-[#a4d233]">IN</span>
        </div>
        <nav class="hidden lg:flex items-center gap-8 text-sm font-semibold text-gray-600">
          <a href="#" class="text-[#1c4532] hover:text-green-700 transition">Beranda</a>
          <a href="#" class="hover:text-green-700 transition">Fitur AI</a>
          <a href="#" class="hover:text-green-700 transition">Harga</a>
          <a href="#" class="hover:text-green-700 transition">Testimoni</a>
        </nav>
      </div>
      
      <div class="flex items-center gap-5">
        
        <div class="relative">
          <button 
            @click="isLoginDropdownOpen = !isLoginDropdownOpen"
            class="text-sm font-bold transition flex items-center gap-1.5 focus:outline-none"
            :class="isLoginDropdownOpen ? 'text-[#1c4532]' : 'text-gray-700 hover:text-[#1c4532]'"
          >
            Masuk
            <svg class="w-3.5 h-3.5 transition-transform duration-200" :class="{ 'rotate-180': isLoginDropdownOpen }" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M19 9l-7 7-7-7"></path>
            </svg>
          </button>

          <div v-if="isLoginDropdownOpen" @click="isLoginDropdownOpen = false" class="fixed inset-0 z-40"></div>

          <Transition name="fade-down">
            <div v-if="isLoginDropdownOpen" class="absolute top-full right-0 mt-3 w-48 bg-white rounded-2xl shadow-xl border border-gray-100 overflow-hidden z-50 py-2">
              <NuxtLink 
                to="panel_petani/dashboard_petani" 
                @click="isLoginDropdownOpen = false"
                class="flex items-center gap-3 px-5 py-3 text-sm font-bold text-gray-600 hover:bg-green-50 hover:text-[#1c4532] transition-colors"
              >
                <svg class="w-4 h-4 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z"></path></svg>
                Sistem Panel
              </NuxtLink>
              <NuxtLink 
                to="/ecommerce/beranda" 
                @click="isLoginDropdownOpen = false"
                class="flex items-center gap-3 px-5 py-3 text-sm font-bold text-gray-600 hover:bg-green-50 hover:text-[#1c4532] transition-colors"
              >
                <svg class="w-4 h-4 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M16 11V7a4 4 0 00-8 0v4M5 9h14l1 12H4L5 9z"></path></svg>
                E-Commerce
              </NuxtLink>
            </div>
          </Transition>
        </div>

        <button class="bg-[#1c4532] text-white px-6 py-2.5 rounded-full text-sm font-bold hover:bg-green-950 transition shadow-md shadow-green-900/10 relative z-50">
          Mulai Gratis
        </button>
      </div>
    </header>

    <section class="container mx-auto px-6 pt-12 pb-24 flex flex-col lg:flex-row items-center gap-12 relative z-10">
      <div class="w-full lg:w-1/2 flex flex-col items-start text-left">
        <div class="inline-flex items-center gap-2 px-3 py-1.5 rounded-full bg-white border border-green-200/80 text-[11px] font-bold text-green-700 mb-6 shadow-sm">
          <span class="w-2 h-2 rounded-full bg-green-500 animate-pulse"></span>
          Generasi AI Tercepat v99.9
        </div>
        <h1 class="text-4xl md:text-5xl lg:text-6xl font-extrabold text-[#1c4532] leading-[1.15] mb-6">
          Koperasi Maju<br />Berbasis AI, <br />
          <span class="text-[#a4d233]">Akselerasi Bisnis.</span>
        </h1>
        <p class="text-gray-500 text-base md:text-lg mb-8 max-w-xl leading-relaxed">
          Platform Koperasi AI tercanggih yang mengubah Pencatatan Manual menjadi <strong class="text-gray-800 font-semibold">Otomatis</strong> dalam beberapa waktu, tanpa ribet, siap pakai, dan otomatis dioptimalkan.
        </p>
        <div class="flex flex-wrap gap-4 w-full sm:w-auto">
          <button class="bg-[#1c4532] text-white px-8 py-4 rounded-full font-bold text-sm hover:bg-green-950 transition shadow-xl shadow-green-900/20 w-full sm:w-auto">
            Mulai Coba Gratis
          </button>
          <button class="bg-white text-gray-800 border border-gray-200 px-8 py-4 rounded-full font-bold text-sm hover:bg-gray-50 transition flex items-center justify-center gap-2 shadow-sm w-full sm:w-auto">
            <svg class="w-4 h-4 text-green-600" fill="currentColor" viewBox="0 0 24 24"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 14.5v-9l6 4.5-6 4.5z"/></svg>
            Lihat Demo
          </button>
        </div>
      </div>

      <div class="w-full lg:w-1/2 flex justify-center relative">
        <div class="bg-white/70 backdrop-blur-xl border border-white p-5 rounded-[32px] shadow-2xl w-full max-w-md aspect-[4/3] flex flex-col transform hover:-translate-y-1 transition duration-500">
          <div class="flex items-center gap-1.5 mb-5 border-b border-gray-100 pb-3">
            <div class="w-3 h-3 rounded-full bg-red-400"></div>
            <div class="w-3 h-3 rounded-full bg-yellow-400"></div>
            <div class="w-3 h-3 rounded-full bg-green-400"></div>
          </div>
          <div class="flex-1 flex flex-col items-center justify-center border-2 border-dashed border-green-200/60 rounded-2xl bg-green-50/30 p-6">
            <svg class="w-10 h-10 text-green-500 animate-spin mb-3" fill="none" viewBox="0 0 24 24">
              <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
              <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
            <p class="text-sm text-green-800 font-bold tracking-wide">Menyelesaikan Masalah...</p>
            <p class="text-xs text-gray-400 mt-1">Sistem menganalisis data peternakan & tani</p>
          </div>
        </div>
      </div>
    </section>

    <section class="border-y border-gray-200/60 bg-white/40 backdrop-blur-md py-8">
      <div class="container mx-auto px-6">
        <p class="text-center text-[10px] font-extrabold tracking-widest text-gray-400 uppercase mb-6">MITRA • Telah Digunakan Oleh</p>
        <div class="flex justify-center items-center gap-8 md:gap-14 flex-wrap opacity-50 grayscale">
          <div v-for="i in 5" :key="i" class="flex items-center gap-2">
            <svg class="w-5 h-5 text-[#1c4532]" fill="currentColor" viewBox="0 0 24 24"><path d="M12 7V3H2v18h20V7H12zm-2 12H4v-2h6v2zm0-4H4v-2h6v2zm0-4H4V7h6v2zm10 8h-8v-2h8v2zm0-4h-8v-2h8v2zm0-4h-8V9h8v2z"/></svg>
            <span class="font-black text-xs tracking-wider text-[#1c4532]">BUMN / KUD {{i}}</span>
          </div>
        </div>
      </div>
    </section>

    <section class="py-24 overflow-hidden perspective-1500 relative">
      <div class="text-center mb-16 px-6">
        <span class="bg-green-100/60 text-green-800 text-[10px] font-extrabold px-3 py-1 rounded-full uppercase tracking-widest">Fitur Masa Depan Koperasi</span>
        <h2 class="text-3xl md:text-4xl font-extrabold text-[#1c4532] mt-3">Otomatisasi Menggunakan AI</h2>
      </div>

      <div class="relative w-full max-w-5xl mx-auto h-[380px] flex items-center justify-center px-4">
        <div class="absolute inset-0 w-full h-full flex items-center justify-center transform-style-preserve-3d pointer-events-none">
          <div 
            v-for="(feature, index) in features" 
            :key="feature.id"
            class="absolute w-[300px] sm:w-[360px] md:w-[410px] bg-white rounded-[28px] shadow-2xl shadow-gray-200/80 p-8 border border-gray-100 transition-all"
            :class="getSlideClass(index)"
          >
            <div class="flex items-center gap-4 mb-6">
              <div class="w-12 h-12 rounded-xl bg-green-50 flex items-center justify-center text-green-600">
                <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 24 24"><path d="M19 8l-4 4h3c0 3.31-2.69 6-6 6-1.01 0-1.97-.25-2.8-.7l-1.46 1.46C8.97 19.54 10.43 20 12 20c4.42 0 8-3.58 8-8h3l-4-4zM6 12c0-3.31 2.69-6 6-6 1.01 0 1.97.25 2.8.7l1.46-1.46C15.03 4.46 13.57 4 12 4c-4.42 0-8 3.58-8 8H1l4 4 4-4H6z"/></svg>
              </div>
              <div>
                <p class="text-[10px] font-black text-gray-400 uppercase tracking-widest">Generative UI</p>
                <div class="flex gap-2 items-center mt-0.5">
                  <span class="text-[11px] font-bold text-gray-500">10 Minute</span>
                  <span class="w-1 h-1 rounded-full bg-gray-300"></span>
                  <span class="text-[11px] font-bold text-green-600">Easy to use</span>
                </div>
              </div>
            </div>
            <h3 class="text-xl font-bold text-gray-800 mb-3">{{ feature.title }}</h3>
            <p class="text-xs sm:text-sm text-gray-500 leading-relaxed">{{ feature.desc }}</p>
          </div>
        </div>

        <button @click="prevSlide" class="absolute left-2 md:left-8 z-[60] bg-green-500 hover:bg-green-600 text-white p-3.5 rounded-full shadow-xl transition-all hover:scale-105 active:scale-95 flex items-center justify-center cursor-pointer pointer-events-auto border-2 border-transparent focus:outline-none focus:border-green-200">
          <svg class="w-5 h-5 stroke-2" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M15 19l-7-7 7-7"/></svg>
        </button>
        <button @click="nextSlide" class="absolute right-2 md:right-8 z-[60] bg-green-500 hover:bg-green-600 text-white p-3.5 rounded-full shadow-xl transition-all hover:scale-105 active:scale-95 flex items-center justify-center cursor-pointer pointer-events-auto border-2 border-transparent focus:outline-none focus:border-green-200">
          <svg class="w-5 h-5 stroke-2" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M9 5l7 7-7 7"/></svg>
        </button>
      </div>
      
      <div class="flex justify-center items-center gap-1.5 mt-4 relative z-[60]">
        <button 
          v-for="(f, i) in features" 
          :key="i" 
          @click="currentIndex = i"
          class="h-2 rounded-full transition-all duration-300 pointer-events-auto cursor-pointer" 
          :class="i === currentIndex ? 'bg-green-600 w-5' : 'bg-gray-300 w-2'"
        ></button>
      </div>
    </section>

    <section class="py-20 bg-gradient-to-b from-transparent to-green-50/30">
      <div class="container mx-auto px-6 max-w-6xl">
        <div class="text-center mb-16">
          <span class="bg-green-100/60 text-green-800 text-[10px] font-extrabold px-3 py-1 rounded-full uppercase tracking-widest">Testimoni</span>
          <h2 class="text-3xl md:text-4xl font-extrabold text-[#1c4532] mt-3">Kisah Sukses Koperasi Bersama Kami</h2>
        </div>

        <div class="flex flex-col lg:flex-row items-center justify-center gap-12">
          
          <div class="w-full lg:w-1/2 p-2 bg-white rounded-3xl shadow-xl border border-gray-100 relative z-10">
            <ClientOnly fallback-tag="div" fallback="Memuat Peta Interaktif...">
              <div 
                ref="mapContainer" 
                class="w-full aspect-[16/10] bg-gray-50 rounded-2xl overflow-hidden relative z-0"
                style="min-height: 380px;"
              ></div>
            </ClientOnly>
          </div>

          <div class="w-full lg:w-[420px]">
            <div class="bg-white p-8 rounded-[28px] shadow-xl border border-gray-50 relative">
              <div class="flex items-center gap-4 mb-5">
                <div class="w-12 h-12 rounded-full bg-emerald-700 text-white flex items-center justify-center font-bold text-sm shadow-inner">
                  SA
                </div>
                <div>
                  <h4 class="font-extrabold text-gray-800 text-base">Siti Aminah</h4>
                  <p class="text-xs text-gray-400 font-semibold">Ibu Rumah Tangga / Peternak Susu</p>
                </div>
              </div>
              <p class="text-gray-600 text-sm italic leading-relaxed mb-6">
                "Menghemat biaya sewa web developer puluhan juta. Sangat disarankan untuk UMKM, kluster petani rintisan, dan kelompok ternak desa yang baru merintis!"
              </p>
              <div class="flex text-yellow-400 gap-0.5">
                <svg v-for="i in 5" :key="i" class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20"><path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z"/></svg>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>

    <section class="py-6 relative z-20 -mt-10">
      <div class="container mx-auto px-6 max-w-5xl">
        <div class="bg-white rounded-[32px] shadow-2xl shadow-gray-200/60 p-8 md:p-10 flex flex-col md:flex-row justify-between items-center gap-8 border border-gray-100">
          <div class="text-center w-full">
            <h3 class="text-3xl md:text-4xl font-black text-[#1c4532] mb-1">150k+</h3>
            <p class="text-xs font-bold text-gray-400 uppercase tracking-wide">Koperasi Digital Terbantu</p>
          </div>
          <div class="hidden md:block w-px h-12 bg-gray-200"></div>
          <div class="text-center w-full">
            <h3 class="text-3xl md:text-4xl font-black text-[#1c4532] mb-1">Rp 2.4T</h3>
            <p class="text-xs font-bold text-gray-400 uppercase tracking-wide">Transaksi Terproses</p>
          </div>
          <div class="hidden md:block w-px h-12 bg-gray-200"></div>
          <div class="text-center w-full">
            <h3 class="text-3xl md:text-4xl font-black text-[#1c4532] mb-1">3x Lipat</h3>
            <p class="text-xs font-bold text-gray-400 uppercase tracking-wide">Efisiensi Biaya Operasional</p>
          </div>
        </div>
      </div>
    </section>

    <section class="py-24 text-center">
      <div class="container mx-auto px-6 max-w-3xl">
         <span class="bg-green-100/60 text-green-800 text-[10px] font-extrabold px-4 py-1.5 rounded-full uppercase tracking-widest mb-6 inline-block">Coba Sekarang</span>
         <div class="bg-white p-10 md:p-14 rounded-[36px] shadow-xl border border-green-100/40 relative overflow-hidden">
            <div class="absolute -top-24 -right-24 w-64 h-64 bg-green-100/50 rounded-full blur-3xl opacity-60 pointer-events-none"></div>
            
            <h2 class="text-3xl md:text-4xl font-black text-[#1c4532] mb-4 relative z-10">Coba Keajaiban AI Sekarang</h2>
            <p class="text-sm text-gray-400 font-medium mb-8 max-w-md mx-auto relative z-10 leading-relaxed">Biarkan AI yang bekerja untuk anda, anda cukup duduk dan bersantai memantau perkembangan ekosistem agro.</p>
            
            <button class="bg-[#1c4532] text-white px-10 py-4 rounded-full font-bold text-sm hover:bg-green-950 transition shadow-lg inline-flex items-center gap-2.5 relative z-10">
              <svg class="w-4 h-4 text-[#a4d233]" fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M11.3 1.046A1 1 0 0112 2v5h4a1 1 0 01.82 1.573l-7 10A1 1 0 018 18v-5H4a1 1 0 01-.82-1.573l7-10a1 1 0 011.12-.38z" clip-rule="evenodd"/></svg>
              Mulai
            </button>
         </div>
      </div>
    </section>

  </div>
</template>

<style scoped>
/* CSS LEAFLET & SLIDER */
@import url('https://unpkg.com/leaflet@1.9.4/dist/leaflet.css');

.perspective-1500 { perspective: 1500px; }
.transform-style-preserve-3d { transform-style: preserve-3d; }

.active-slide, .prev-slide, .next-slide {
  transition: transform 0.6s cubic-bezier(0.25, 1, 0.5, 1), opacity 0.6s ease;
  will-change: transform, opacity;
}

.center-slide { transform: translateX(0) scale(1) rotateY(0deg); }
.rotate-y-left { transform: translateX(-55%) scale(0.86) rotateY(18deg); }
.rotate-y-right { transform: translateX(55%) scale(0.86) rotateY(-18deg); }

@media (min-width: 768px) {
  .rotate-y-left { transform: translateX(-80%) scale(0.88) rotateY(22deg); }
  .rotate-y-right { transform: translateX(80%) scale(0.88) rotateY(-22deg); }
}

:deep(.leaflet-pane) { z-index: 10 !important; }
:deep(.leaflet-top), :deep(.leaflet-bottom) { z-index: 20 !important; }
:deep(.leaflet-popup-content-wrapper) {
  border-radius: 12px;
  box-shadow: 0 10px 25px -5px rgba(0, 0, 0, 0.1), 0 8px 10px -6px rgba(0, 0, 0, 0.1);
  border: 1px solid #f3f4f6;
  padding: 0;
}
:deep(.leaflet-popup-content) { margin: 14px; }
:deep(.leaflet-popup-tip) { background: white; border: 1px solid #f3f4f6; }

/* Animasi untuk Modal Dropdown */
.fade-down-enter-active,
.fade-down-leave-active {
  transition: opacity 0.2s ease, transform 0.2s cubic-bezier(0.175, 0.885, 0.32, 1.275);
}
.fade-down-enter-from,
.fade-down-leave-to {
  opacity: 0;
  transform: translateY(-10px);
}
</style>