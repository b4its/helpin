<script setup lang="ts">
import { ref, shallowRef, onMounted, nextTick } from 'vue'

// ==========================================
// 1. STATE & LOGIC: DROPDOWN LOGIN
// ==========================================
const isLoginDropdownOpen = ref(false)

// ==========================================
// 2. STATE & LOGIC: SCROLL ANIMATION (DIRECTIVE)
// ==========================================
// Custom directive v-reveal untuk animasi saat elemen masuk viewport
const vReveal = {
  mounted: (el: HTMLElement, binding: any) => {
    // Default class untuk state awal (tersembunyi & bergeser ke bawah)
    const delayClass = binding.value?.delay || 'delay-0'
    const directionClass = binding.value?.direction || 'translate-y-12'
    
    el.classList.add('opacity-0', directionClass, 'transition-all', 'duration-1000', 'ease-[cubic-bezier(0.25,0.1,0.25,1)]', delayClass)
    
    const observer = new IntersectionObserver((entries) => {
      entries.forEach(entry => {
        if (entry.isIntersecting) {
          // Hapus class transisi awal, ganti ke state normal
          el.classList.remove('opacity-0', directionClass)
          el.classList.add('opacity-100', 'translate-y-0', 'translate-x-0')
          observer.unobserve(el) // Hanya jalankan sekali
        }
      })
    }, { 
      threshold: 0.15, // Memicu animasi ketika 15% elemen terlihat
      rootMargin: '0px 0px -50px 0px' 
    })
    
    observer.observe(el)
  }
}

// ==========================================
// 3. STATE & LOGIC: 3D CAROUSEL SLIDER
// ==========================================
const features = ref([
  { 
    id: 1, 
    title: 'AI Pencatatan Panen', 
    desc: 'Sistem cerdas yang otomatis merekap hasil pertanian, berat gabah, hingga distribusi logistik secara real-time langsung dari sawah.',
    icon: 'M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15'
  },
  { 
    id: 2, 
    title: 'AI Problem Solver', 
    desc: 'Asisten virtual berbasis machine learning yang membantu menganalisis kesehatan hewan ternak, memprediksi masa panen, serta rekomendasi pupuk.',
    icon: 'M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z'
  },
  { 
    id: 3, 
    title: 'Koperasi Transparan', 
    desc: 'Otomatisasi pembagian Sisa Hasil Usaha (SHU), manajemen kas simpan pinjam anggota, dan monitoring validasi transaksi tanpa kesalahan.',
    icon: 'M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z'
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
  if (index === currentIndex.value) return 'active-slide z-20 opacity-100 center-slide pointer-events-auto border-green-200 bg-white shadow-2xl shadow-green-900/10'
  if (index === (currentIndex.value - 1 + features.value.length) % features.value.length) return 'prev-slide z-10 opacity-40 rotate-y-left pointer-events-auto bg-gray-50'
  if (index === (currentIndex.value + 1) % features.value.length) return 'next-slide z-10 opacity-40 rotate-y-right pointer-events-auto bg-gray-50'
  return 'hidden opacity-0 pointer-events-none'
}

// ==========================================
// 4. STATE & LOGIC: LEAFLET MAP
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

      L.tileLayer('https://{s}.basemaps.cartocdn.com/rastertiles/voyager/{z}/{x}/{y}{r}.png', {
        maxZoom: 19,
        attribution: '© OpenStreetMap contributors © CARTO'
      }).addTo(map.value)

      koperasiLocations.forEach(loc => {
        const marker = L.marker([loc.lat, loc.lng]).addTo(map.value)
        const statusColor = loc.status === 'Aktif AI' ? '#166534' : '#854d0e';
        const statusBg = loc.status === 'Aktif AI' ? '#dcfce7' : '#fef08a';
        
        const popupContent = `
          <div style="min-width: 220px; font-family: 'Inter', system-ui, sans-serif;">
            <h4 style="margin: 0 0 4px 0; font-size: 15px; font-weight: 800; color: #1c4532;">${loc.name}</h4>
            <p style="margin: 0 0 12px 0; font-size: 12px; color: #6b7280;">${loc.address}</p>
            <div style="border-top: 1px solid #e5e7eb; padding-top: 10px; margin-bottom: 12px; display: flex; flex-direction: column; gap: 4px;">
              <div style="display: flex; justify-content: space-between; font-size: 12px;">
                <span style="color: #6b7280;">Fokus:</span>
                <strong style="color: #374151;">${loc.type}</strong>
              </div>
              <div style="display: flex; justify-content: space-between; font-size: 12px;">
                <span style="color: #6b7280;">Anggota:</span>
                <strong style="color: #374151;">${loc.members} Orang</strong>
              </div>
            </div>
            <span style="display: inline-block; width: 100%; text-align: center; background-color: ${statusBg}; color: ${statusColor}; padding: 6px 0; border-radius: 6px; font-size: 11px; font-weight: 700; letter-spacing: 0.02em;">
              ${loc.status}
            </span>
          </div>
        `
        marker.bindPopup(popupContent)
      })

      setTimeout(() => {
        if (map.value) map.value.invalidateSize()
      }, 200)

    }, 200)
  }
})
</script>

<template>
  <div class="relative overflow-hidden w-full min-h-screen bg-[#f8fdf9] text-gray-800 antialiased font-sans selection:bg-green-200 selection:text-green-900">
    
    <div class="absolute top-0 left-0 w-full h-[900px] overflow-hidden -z-10 pointer-events-none">
      <div class="absolute top-[-10%] left-[-10%] w-[60%] h-[60%] bg-green-200/40 rounded-full blur-[120px] mix-blend-multiply animate-blob"></div>
      <div class="absolute top-[10%] right-[-10%] w-[50%] h-[50%] bg-[#a4d233]/20 rounded-full blur-[120px] mix-blend-multiply animate-blob animation-delay-2000"></div>
    </div>

<header class="fixed top-0 left-0 w-full z-[100] bg-[#f8fdf9]/80 backdrop-blur-md border-b border-gray-200/50 shadow-sm transition-all duration-300">
      <div class="container mx-auto px-6 py-4 flex items-center justify-between">
        
        <div class="flex items-center gap-12" v-reveal="{ direction: 'translate-y-[-20px]' }">
          <div class="flex items-center cursor-pointer hover:scale-105 transition-transform">
            <img src="/assets/helpin_light_logo.png" alt="Logo Helpin Services" class="h-10 w-auto" />
          </div>
          <nav class="hidden lg:flex items-center gap-8 text-sm font-semibold text-gray-600">
            <a href="#beranda" class="text-[#1c4532] hover:text-green-700 transition">Beranda</a>
            <a href="#fitur" class="hover:text-green-700 transition">Fitur AI</a>
            <a href="#testimoni" class="hover:text-green-700 transition">Testimoni</a>
          </nav>
        </div>
        
<div class="flex items-center gap-5" v-reveal="{ direction: 'translate-y-[-20px]', delay: 'delay-100' }">
          <div class="relative">
            <button 
              @click="isLoginDropdownOpen = !isLoginDropdownOpen"
              class="text-sm font-bold transition flex items-center gap-1.5 focus:outline-none"
              :class="isLoginDropdownOpen ? 'text-[#1c4532]' : 'text-gray-700 hover:text-[#1c4532]'"
            >
              Masuk
              <svg class="w-4 h-4 transition-transform duration-300" :class="{ 'rotate-180': isLoginDropdownOpen }" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M19 9l-7 7-7-7"></path>
              </svg>
            </button>

            <div v-if="isLoginDropdownOpen" @click="isLoginDropdownOpen = false" class="fixed inset-0 z-40"></div>

            <Transition name="fade-down">
              <div v-if="isLoginDropdownOpen" class="absolute top-full right-0 mt-5 w-56 bg-white rounded-2xl shadow-xl border border-gray-100 overflow-hidden z-50 py-3">
                
                <div class="px-3 pb-2 border-b border-gray-50 mb-2">
                  <span class="px-3 text-[10px] font-extrabold text-gray-400 uppercase tracking-widest">Sistem Panel</span>
                  
                  <div class="mt-1 flex flex-col">
                    <NuxtLink 
                      to="/panel_petani/dashboard_petani" 
                      @click="isLoginDropdownOpen = false"
                      class="flex items-center gap-3 px-3 py-2.5 rounded-xl text-sm font-bold text-gray-600 hover:bg-green-50 hover:text-[#1c4532] transition-colors"
                    >
                      <div class="w-6 h-6 rounded-md bg-green-100 flex items-center justify-center text-green-600">
                        <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 3v4M3 5h4M6 17v4m-2-2h4m5-16l2.286 6.857L21 12l-5.714 2.143L13 21l-2.286-6.857L5 12l5.714-2.143L13 3z"></path></svg>
                      </div>
                      Panel Petani
                    </NuxtLink>
                    
                    <NuxtLink 
                      to="/panel_peternak/dashboard_peternak" 
                      @click="isLoginDropdownOpen = false"
                      class="flex items-center gap-3 px-3 py-2.5 rounded-xl text-sm font-bold text-gray-600 hover:bg-orange-50 hover:text-orange-700 transition-colors"
                    >
                      <div class="w-6 h-6 rounded-md bg-orange-100 flex items-center justify-center text-orange-600">
                        <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4"></path></svg>
                      </div>
                      Panel Peternak
                    </NuxtLink>
                  </div>
                </div>

                <div class="px-3">
                  <NuxtLink 
                    to="/ecommerce/beranda" 
                    @click="isLoginDropdownOpen = false"
                    class="flex items-center gap-3 px-3 py-2.5 rounded-xl text-sm font-bold text-gray-600 hover:bg-blue-50 hover:text-blue-700 transition-colors"
                  >
                    <div class="w-6 h-6 rounded-md bg-blue-100 flex items-center justify-center text-blue-600">
                      <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 11V7a4 4 0 00-8 0v4M5 9h14l1 12H4L5 9z"></path></svg>
                    </div>
                    E-Commerce
                  </NuxtLink>
                </div>

              </div>
            </Transition>
          </div>

          <button class="bg-[#1c4532] text-white px-6 py-2.5 rounded-full text-sm font-bold hover:bg-green-900 hover:scale-105 active:scale-95 transition-all shadow-lg shadow-green-900/20 relative z-50">
            Mulai Gratis
          </button>
        </div>

      </div>
    </header>

    <section id="beranda" class="container mt-6 mx-auto px-6 pt-16 pb-28 flex flex-col lg:flex-row items-center gap-16 relative z-10">
      <div class="w-full lg:w-[55%] flex flex-col items-start text-left">
        <div v-reveal="{ direction: 'translate-y-8' }" class="inline-flex items-center gap-2 px-4 py-2 rounded-full bg-white border border-green-200/80 text-xs font-bold text-green-700 mb-8 shadow-sm">
          <span class="relative flex h-2.5 w-2.5">
            <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-green-400 opacity-75"></span>
            <span class="relative inline-flex rounded-full h-2.5 w-2.5 bg-green-500"></span>
          </span>
           Hybrid System Cerdas Pertama di Indonesia
        </div>
        <h1 v-reveal="{ direction: 'translate-y-8', delay: 'delay-100' }" class="text-4xl md:text-6xl lg:text-[4rem] font-extrabold text-[#1c4532] leading-[1.1] mb-6">
          Koperasi Maju<br />Berbasis AI, <br />
          <span class="text-[#a4d233]">Akselerasi Bisnis.</span>
        </h1>
        <p v-reveal="{ direction: 'translate-y-8', delay: 'delay-200' }" class="text-gray-500 text-lg mb-10 max-w-xl leading-relaxed">
          Platform manajemen agrikultur tercanggih yang mengubah pencatatan manual menjadi otomatis dalam hitungan detik. Tanpa ribet, siap pakai, dan terpantau secara presisi.
        </p>
        <div v-reveal="{ direction: 'translate-y-8', delay: 'delay-300' }" class="flex flex-wrap gap-4 w-full sm:w-auto">
          <button class="bg-[#1c4532] text-white px-8 py-4 rounded-full font-bold text-base hover:bg-green-900 hover:shadow-green-900/40 hover:-translate-y-1 transition-all duration-300 shadow-xl shadow-green-900/20 w-full sm:w-auto group flex items-center justify-center gap-2">
            Mulai Coba Gratis
            <svg class="w-4 h-4 group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2.5" d="M14 5l7 7m0 0l-7 7m7-7H3"></path></svg>
          </button>
          <button class="bg-white text-gray-800 border border-gray-200 px-8 py-4 rounded-full font-bold text-base hover:bg-gray-50 hover:-translate-y-1 transition-all duration-300 flex items-center justify-center gap-2 shadow-sm w-full sm:w-auto">
            <svg class="w-5 h-5 text-green-600" fill="currentColor" viewBox="0 0 24 24"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 14.5v-9l6 4.5-6 4.5z"/></svg>
            Lihat Demo
          </button>
        </div>
      </div>

      <div v-reveal="{ direction: 'translate-x-12', delay: 'delay-200' }" class="w-full lg:w-[45%] flex justify-center lg:justify-end relative">
        <div class="bg-white/80 backdrop-blur-xl border border-white/80 p-6 rounded-[32px] shadow-2xl w-full max-w-lg flex flex-col relative z-10 animate-float">
          <div class="absolute -inset-4 bg-gradient-to-tr from-green-300/30 to-[#a4d233]/20 rounded-[40px] blur-2xl -z-10"></div>
          
          <div class="flex items-center justify-between mb-6 border-b border-gray-100 pb-4">
            <div class="flex items-center gap-2">
              <div class="w-3 h-3 rounded-full bg-red-400 hover:bg-red-500 transition-colors cursor-pointer"></div>
              <div class="w-3 h-3 rounded-full bg-yellow-400 hover:bg-yellow-500 transition-colors cursor-pointer"></div>
              <div class="w-3 h-3 rounded-full bg-green-400 hover:bg-green-500 transition-colors cursor-pointer"></div>
            </div>
            <div class="flex items-center gap-1.5">
              <span class="w-2 h-2 rounded-full bg-green-500 animate-pulse"></span>
              <span class="text-[10px] font-bold text-gray-400 uppercase tracking-wider">AI Node Active</span>
            </div>
          </div>
          
          <div class="space-y-4">
            <div class="bg-gray-50 border border-gray-100 p-4 rounded-2xl flex items-center gap-4 hover:bg-green-50/50 transition-colors cursor-default">
              <div class="w-10 h-10 rounded-full bg-green-100 flex items-center justify-center text-green-600">
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
              </div>
              <div class="flex-1">
                <div class="h-2 w-1/2 bg-gray-200 rounded mb-2 overflow-hidden relative">
                  <div class="absolute top-0 left-0 h-full w-full bg-gradient-to-r from-transparent via-white/50 to-transparent animate-[shimmer_2s_infinite]"></div>
                </div>
                <div class="h-2 w-1/3 bg-gray-200 rounded"></div>
              </div>
              <span class="text-xs font-bold text-green-600">+12%</span>
            </div>

            <div class="border-2 border-dashed border-green-200/80 rounded-2xl bg-green-50/50 p-6 flex flex-col items-center justify-center text-center relative overflow-hidden group">
              <div class="absolute inset-0 bg-green-100/20 translate-y-full group-hover:translate-y-0 transition-transform duration-500"></div>
              <svg class="w-8 h-8 text-green-500 animate-[spin_3s_linear_infinite] mb-3 relative z-10" fill="none" viewBox="0 0 24 24">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
              </svg>
              <p class="text-sm text-green-800 font-bold tracking-wide relative z-10">Menganalisis Kualitas Panen...</p>
              <p class="text-xs text-gray-500 mt-1 relative z-10">Computer vision memproses data satelit</p>
            </div>
          </div>
        </div>
      </div>
    </section>

    <section v-reveal class="border-y border-gray-200/60 bg-white/60 backdrop-blur-md py-10">
      <div class="container mx-auto px-6">
        <p class="text-center text-xs font-extrabold tracking-[0.2em] text-gray-400 uppercase mb-8">Telah Dipercaya Oleh</p>
        <div class="flex justify-center items-center gap-10 md:gap-16 flex-wrap opacity-60 grayscale hover:grayscale-0 transition-all duration-500">
          <div v-for="i in 5" :key="i" class="flex items-center gap-2 hover:scale-110 transition-transform duration-300 cursor-pointer">
            <svg class="w-6 h-6 text-[#1c4532]" fill="currentColor" viewBox="0 0 24 24"><path d="M12 7V3H2v18h20V7H12zm-2 12H4v-2h6v2zm0-4H4v-2h6v2zm0-4H4V7h6v2zm10 8h-8v-2h8v2zm0-4h-8v-2h8v2zm0-4h-8V9h8v2z"/></svg>
            <span class="font-black text-sm tracking-wider text-[#1c4532]">MITRA AGRO {{i}}</span>
          </div>
        </div>
      </div>
    </section>

    <section id="fitur" class="py-28 overflow-hidden perspective-1500 relative bg-gradient-to-b from-transparent to-green-50/20">
      <div v-reveal="{ direction: 'translate-y-8' }" class="text-center mb-20 px-6 relative z-10">
        <span class="bg-green-100 text-green-800 text-xs font-extrabold px-4 py-1.5 rounded-full uppercase tracking-widest inline-block mb-4 shadow-sm hover:shadow-md transition-shadow">Infrastruktur Inti</span>
        <h2 class="text-3xl md:text-5xl font-extrabold text-[#1c4532]">Otomatisasi Menggunakan AI</h2>
      </div>

      <div v-reveal="{ direction: 'translate-y-12', delay: 'delay-100' }" class="relative w-full max-w-6xl mx-auto h-[420px] flex items-center justify-center px-4">
        <div class="absolute inset-0 w-full h-full flex items-center justify-center transform-style-preserve-3d pointer-events-none">
          <div 
            v-for="(feature, index) in features" 
            :key="feature.id"
            class="absolute w-[320px] sm:w-[400px] md:w-[480px] rounded-[32px] p-8 md:p-10 transition-all duration-700 ease-out border"
            :class="getSlideClass(index)"
          >
            <div class="flex flex-col gap-6">
              <div class="flex items-center justify-between">
                <div class="w-14 h-14 rounded-2xl bg-green-100/50 flex items-center justify-center text-green-600 border border-green-100 shadow-inner">
                  <svg class="w-7 h-7" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" :d="feature.icon" />
                  </svg>
                </div>
                <div class="flex items-center gap-2 bg-gray-50 px-3 py-1.5 rounded-full border border-gray-100">
                  <span class="w-2 h-2 rounded-full bg-green-500 animate-pulse"></span>
                  <span class="text-xs font-bold text-gray-500">Real-time</span>
                </div>
              </div>
              
              <div>
                <h3 class="text-2xl font-extrabold text-gray-800 mb-4">{{ feature.title }}</h3>
                <p class="text-sm md:text-base text-gray-500 leading-relaxed">{{ feature.desc }}</p>
              </div>
            </div>
          </div>
        </div>

        <button @click="prevSlide" class="absolute left-4 md:left-12 z-[60] bg-white text-[#1c4532] hover:bg-green-50 p-4 rounded-full shadow-lg border border-gray-100 transition-transform hover:scale-110 active:scale-95 focus:outline-none">
          <svg class="w-6 h-6 stroke-2" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M15 19l-7-7 7-7"/></svg>
        </button>
        <button @click="nextSlide" class="absolute right-4 md:right-12 z-[60] bg-white text-[#1c4532] hover:bg-green-50 p-4 rounded-full shadow-lg border border-gray-100 transition-transform hover:scale-110 active:scale-95 focus:outline-none">
          <svg class="w-6 h-6 stroke-2" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" d="M9 5l7 7-7 7"/></svg>
        </button>
      </div>
      
      <div class="flex justify-center items-center gap-2 mt-8 relative z-[60]">
        <button 
          v-for="(f, i) in features" 
          :key="i" 
          @click="currentIndex = i"
          class="h-2 rounded-full transition-all duration-300 focus:outline-none" 
          :class="i === currentIndex ? 'bg-green-600 w-8' : 'bg-gray-300 w-2 hover:bg-gray-400'"
          :aria-label="`Slide ${i + 1}`"
        ></button>
      </div>
    </section>

    <section id="testimoni" class="py-24 bg-white relative">
      <div class="container mx-auto px-6 max-w-7xl">
        <div v-reveal="{ direction: 'translate-y-8' }" class="text-center mb-16">
          <h2 class="text-3xl md:text-4xl font-extrabold text-[#1c4532]">Pantauan Ekosistem Aktif</h2>
          <p class="text-gray-500 mt-4 max-w-2xl mx-auto">Kami mengelola data dari berbagai klaster tani dan peternakan di seluruh wilayah dengan presisi tinggi.</p>
        </div>

        <div class="grid grid-cols-1 lg:grid-cols-12 gap-10 items-center">
          
          <div v-reveal="{ direction: 'translate-x-[-20px]' }" class="lg:col-span-7 bg-gray-50 p-3 rounded-[32px] border border-gray-100 shadow-inner hover:shadow-md transition-shadow duration-300">
            <ClientOnly fallback-tag="div" fallback="Memuat Peta Interaktif...">
              <div 
                ref="mapContainer" 
                class="w-full aspect-video rounded-[24px] overflow-hidden relative z-0 shadow-sm"
                style="min-height: 400px;"
              ></div>
            </ClientOnly>
          </div>

          <div v-reveal="{ direction: 'translate-x-[20px]', delay: 'delay-100' }" class="lg:col-span-5 flex flex-col justify-center">
            <div class="bg-[#f8fdf9] p-10 rounded-[32px] border border-green-100 shadow-xl shadow-green-900/5 relative group hover:-translate-y-1 transition-transform duration-300">
              <div class="absolute -top-6 -left-6 text-green-200 group-hover:scale-110 transition-transform duration-300">
                <svg class="w-16 h-16" fill="currentColor" viewBox="0 0 24 24"><path d="M14.017 21v-7.391c0-5.704 3.731-9.57 8.983-10.609l.995 2.151c-2.432.917-3.995 3.638-3.995 5.849h4v10h-9.983zm-14.017 0v-7.391c0-5.704 3.748-9.57 9-10.609l.996 2.151c-2.433.917-3.996 3.638-3.996 5.849h3.983v10h-9.983z"/></svg>
              </div>
              
              <div class="relative z-10">
                <div class="flex text-yellow-400 gap-1 mb-6">
                  <svg v-for="i in 5" :key="i" class="w-5 h-5 animate-[pulse_2s_ease-in-out_infinite]" :style="`animation-delay: ${i * 100}ms`" fill="currentColor" viewBox="0 0 20 20"><path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z"/></svg>
                </div>
                <p class="text-gray-700 text-lg md:text-xl font-medium leading-relaxed mb-8">
                  "Sistem otomatisasi yang menghemat biaya operasional kami. Sangat disarankan untuk UMKM, kluster petani rintisan, dan kelompok ternak desa. UI/UX-nya luar biasa intuitif!"
                </p>
                <div class="flex items-center gap-4">
                  <div class="w-14 h-14 rounded-full bg-[#1c4532] text-white flex items-center justify-center font-bold text-lg shadow-md">
                    SA
                  </div>
                  <div>
                    <h4 class="font-extrabold text-gray-900 text-lg">Siti Aminah</h4>
                    <p class="text-sm text-green-600 font-semibold">Ketua Koperasi Susu Makmur</p>
                  </div>
                </div>
              </div>
            </div>
          </div>

        </div>
      </div>
    </section>

    <section class="py-12 relative z-20 bg-[#1c4532] mt-10 overflow-hidden">
      <div class="absolute top-0 right-0 w-64 h-64 bg-green-600/20 rounded-full blur-3xl translate-x-1/2 -translate-y-1/2"></div>
      
      <div class="container mx-auto px-6 max-w-6xl relative z-10">
        <div class="flex flex-col md:flex-row justify-between items-center gap-12 py-8">
          <div v-reveal="{ direction: 'translate-y-10' }" class="text-center w-full hover:scale-105 transition-transform duration-300">
            <h3 class="text-4xl md:text-5xl font-black text-white mb-2">150k+</h3>
            <p class="text-xs font-bold text-green-300 uppercase tracking-widest">Koperasi Terbantu</p>
          </div>
          <div class="hidden md:block w-px h-16 bg-green-700/50"></div>
          <div v-reveal="{ direction: 'translate-y-10', delay: 'delay-100' }" class="text-center w-full hover:scale-105 transition-transform duration-300">
            <h3 class="text-4xl md:text-5xl font-black text-white mb-2">Rp 2.4T</h3>
            <p class="text-xs font-bold text-green-300 uppercase tracking-widest">Volume Transaksi</p>
          </div>
          <div class="hidden md:block w-px h-16 bg-green-700/50"></div>
          <div v-reveal="{ direction: 'translate-y-10', delay: 'delay-200' }" class="text-center w-full hover:scale-105 transition-transform duration-300">
            <h3 class="text-4xl md:text-5xl font-black text-white mb-2">3x Lipat</h3>
            <p class="text-xs font-bold text-green-300 uppercase tracking-widest">Efisiensi Biaya</p>
          </div>
        </div>
      </div>
    </section>

    <section class="py-32 text-center bg-[#f8fdf9]">
      <div class="container mx-auto px-6 max-w-4xl">
         <div v-reveal="{ direction: 'translate-y-12' }" class="bg-white p-12 md:p-20 rounded-[40px] shadow-2xl shadow-green-900/5 border border-green-50 relative overflow-hidden group">
            <div class="absolute -top-32 -right-32 w-80 h-80 bg-green-200/40 rounded-full blur-3xl opacity-60 pointer-events-none group-hover:scale-125 transition-transform duration-700"></div>
            <div class="absolute -bottom-32 -left-32 w-80 h-80 bg-[#a4d233]/20 rounded-full blur-3xl opacity-60 pointer-events-none group-hover:scale-125 transition-transform duration-700"></div>
            
            <h2 class="text-4xl md:text-5xl font-black text-[#1c4532] mb-6 relative z-10">Integrasikan HELPIN Ke Koperasi Anda</h2>
            <p class="text-base md:text-lg text-gray-500 font-medium mb-10 max-w-2xl mx-auto relative z-10 leading-relaxed">
              Biarkan sistem cerdas kami menangani pencatatan dan analisis harian. Anda cukup duduk dan memantau perkembangan ekosistem agrobisnis secara presisi.
            </p>
            
            <button class="bg-[#1c4532] text-white px-12 py-5 rounded-full font-bold text-lg hover:bg-green-900 transition-all hover:scale-105 hover:shadow-green-900/40 shadow-xl shadow-green-900/20 inline-flex items-center gap-3 relative z-10">
              Mulai Sekarang Gratis
              <svg class="w-5 h-5 text-[#a4d233] animate-pulse" fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M11.3 1.046A1 1 0 0112 2v5h4a1 1 0 01.82 1.573l-7 10A1 1 0 018 18v-5H4a1 1 0 01-.82-1.573l7-10a1 1 0 011.12-.38z" clip-rule="evenodd"/></svg>
            </button>
         </div>
      </div>
    </section>

  </div>
</template>

<style scoped>
/* Import CSS Leaflet Modern */
@import url('https://unpkg.com/leaflet@1.9.4/dist/leaflet.css');

.perspective-1500 { perspective: 1500px; }
.transform-style-preserve-3d { transform-style: preserve-3d; }

/* Delay Utilities untuk Directive Reveal */
.delay-0 { transition-delay: 0ms; }
.delay-100 { transition-delay: 100ms; }
.delay-200 { transition-delay: 200ms; }
.delay-300 { transition-delay: 300ms; }

/* Custom Keyframes untuk Tailwind */
@keyframes blob {
  0% { transform: translate(0px, 0px) scale(1); }
  33% { transform: translate(30px, -50px) scale(1.1); }
  66% { transform: translate(-20px, 20px) scale(0.9); }
  100% { transform: translate(0px, 0px) scale(1); }
}

@keyframes float {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-10px); }
}

@keyframes shimmer {
  100% { transform: translateX(100%); }
}

.animate-blob {
  animation: blob 7s infinite;
}
.animate-float {
  animation: float 6s ease-in-out infinite;
}
.animation-delay-2000 {
  animation-delay: 2s;
}

/* Transisi Slider 3D */
.active-slide, .prev-slide, .next-slide {
  transition: transform 0.7s cubic-bezier(0.2, 0.8, 0.2, 1), opacity 0.7s ease, box-shadow 0.5s ease;
  will-change: transform, opacity;
}

.center-slide { transform: translateX(0) scale(1) rotateY(0deg); }
.rotate-y-left { transform: translateX(-60%) scale(0.85) rotateY(15deg); }
.rotate-y-right { transform: translateX(60%) scale(0.85) rotateY(-15deg); }

@media (min-width: 768px) {
  .rotate-y-left { transform: translateX(-85%) scale(0.85) rotateY(25deg); }
  .rotate-y-right { transform: translateX(85%) scale(0.85) rotateY(-25deg); }
}

/* Kustomisasi Leaflet Popup */
:deep(.leaflet-pane) { z-index: 10 !important; }
:deep(.leaflet-top), :deep(.leaflet-bottom) { z-index: 20 !important; }
:deep(.leaflet-popup-content-wrapper) {
  border-radius: 16px;
  box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);
  border: 1px solid #f3f4f6;
  padding: 0;
  overflow: hidden;
}
:deep(.leaflet-popup-content) { margin: 16px; }
:deep(.leaflet-popup-tip-container) { display: none; }

/* Animasi untuk Modal Dropdown */
.fade-down-enter-active,
.fade-down-leave-active {
  transition: opacity 0.25s ease, transform 0.25s cubic-bezier(0.175, 0.885, 0.32, 1.275);
}
.fade-down-enter-from,
.fade-down-leave-to {
  opacity: 0;
  transform: translateY(-10px);
}
</style>