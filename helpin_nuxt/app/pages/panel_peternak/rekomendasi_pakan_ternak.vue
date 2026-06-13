<template>
  <div class="flex h-screen w-full bg-[#f4f7f5] font-sans overflow-hidden relative text-slate-900">

    <SidebarPeternak :isOpen="isSidebarOpen" @close="isSidebarOpen = false" />

    <main class="flex-1 flex flex-col overflow-y-auto relative w-full no-scrollbar">
      <header class="flex justify-between items-center px-6 md:px-10 py-6 border-b border-gray-200 bg-white shadow-sm z-10 sticky top-0">
        <div class="flex items-center gap-4">
          <button @click="isSidebarOpen = true" class="md:hidden p-2 text-gray-600 hover:bg-gray-100 rounded-lg transition"><MenuIcon class="w-6 h-6" /></button>
          <div>
            <h1 class="text-xl md:text-2xl font-extrabold text-gray-800 leading-tight">Rekomendasi Pakan</h1>
            <p class="text-xs md:text-sm text-gray-500 font-medium mt-0.5">Analisa kecocokan pakan per kandang (AI / ensemble)</p>
          </div>
        </div>
        <div class="flex items-center gap-2 bg-green-100 px-3 py-1.5 md:px-4 md:py-2 rounded-lg border border-green-200">
          <div class="w-2 h-2 rounded-full bg-green-600 shrink-0 animate-pulse"></div>
          <span class="text-xs md:text-sm font-bold text-green-600">ONLINE</span>
        </div>
      </header>

      <div class="p-4 md:p-10 grid grid-cols-1 lg:grid-cols-3 gap-6">
        <!-- Pilih kandang -->
        <section class="lg:col-span-1 bg-white rounded-[32px] shadow-sm border border-gray-100 p-5">
          <h2 class="text-sm font-black text-gray-700 uppercase tracking-widest mb-4">Pilih Kandang</h2>
          <div v-if="loading" class="py-10 flex justify-center"><div class="animate-spin rounded-full h-6 w-6 border-b-2 border-[#1a402d]"></div></div>
          <div v-else class="space-y-2">
            <button v-for="p in pens" :key="p.id" @click="selectPen(p)"
              :class="['w-full text-left p-4 rounded-2xl border transition-all', selected?.id === p.id ? 'border-[#1a402d] bg-green-50' : 'border-gray-100 hover:border-gray-200']">
              <p class="font-black text-gray-800 uppercase">{{ p.name }}</p>
              <p class="text-[11px] text-gray-400 font-bold">{{ p.pen_type }} · {{ p.occupancy }}/{{ p.capacity }} ekor · {{ p.condition }}</p>
            </button>
            <p v-if="pens.length === 0" class="text-center text-gray-300 font-bold py-8">Belum ada kandang</p>
          </div>
        </section>

        <!-- Hasil analisa -->
        <section class="lg:col-span-2 space-y-6">
          <div v-if="!selected" class="bg-white rounded-[32px] border border-gray-100 p-16 text-center text-gray-300 font-bold">Pilih kandang untuk analisa pakan.</div>

          <template v-else>
            <div class="bg-white rounded-[32px] shadow-sm border border-gray-100 p-6 flex items-center justify-between gap-4 flex-wrap">
              <div>
                <h2 class="text-2xl font-black text-gray-800 uppercase tracking-tighter">{{ selected.name }}</h2>
                <p class="text-sm text-gray-400 font-bold">{{ selected.pen_type }} · {{ selected.occupancy }}/{{ selected.capacity }} ekor</p>
              </div>
              <button @click="runAnalysis" :disabled="analyzing" class="bg-[#1a402d] text-white px-6 py-3 rounded-2xl font-black uppercase text-xs shadow-lg flex items-center gap-2 disabled:opacity-50">
                <SoupIcon class="w-4 h-4" /> {{ analyzing ? 'Menganalisa...' : 'Analisa Pakan' }}
              </button>
            </div>

            <div v-if="analyzing" class="bg-white rounded-[32px] border border-gray-100 p-16 text-center text-gray-400 font-bold">
              <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-[#1a402d] mx-auto mb-3"></div>Menganalisa kecocokan pakan...
            </div>
            <div v-else-if="error" class="bg-white rounded-[32px] border border-red-100 p-10 text-center">
              <p class="text-red-500 font-bold">{{ error }}</p>
            </div>
            <div v-else-if="result" class="space-y-6">
              <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
                <div class="bg-green-50 p-5 rounded-3xl border border-green-100"><p class="text-[10px] font-black text-green-500 uppercase">Grade</p><p class="text-2xl font-black text-green-700">{{ result.quality_projection?.grade }}</p></div>
                <div class="bg-blue-50 p-5 rounded-3xl border border-blue-100"><p class="text-[10px] font-black text-blue-500 uppercase">ADG</p><p class="text-2xl font-black text-blue-700">{{ result.quality_projection?.projected_adg_kg }}<small class="text-xs"> kg/hr</small></p></div>
                <div class="bg-gray-50 p-5 rounded-3xl border border-gray-100"><p class="text-[10px] font-black text-gray-400 uppercase">FCR</p><p class="text-2xl font-black text-slate-800">{{ result.quality_projection?.fcr }}</p></div>
                <div class="bg-orange-50 p-5 rounded-3xl border border-orange-100"><p class="text-[10px] font-black text-orange-500 uppercase">Target 90hr</p><p class="text-2xl font-black text-orange-700">{{ result.quality_projection?.target_weight_kg }}<small class="text-xs"> kg</small></p></div>
              </div>

              <div class="bg-white rounded-[32px] shadow-sm border border-gray-100 p-6">
                <h3 class="text-xs font-black text-gray-700 uppercase tracking-[0.2em] mb-4 flex items-center gap-2"><SoupIcon class="w-4 h-4 text-orange-500" /> Top Pakan Terbaik <span class="text-gray-300">({{ result.analysis_source }})</span></h3>
                <div v-if="(result.top_feeds||[]).length === 0" class="text-sm text-gray-400 italic">Belum ada kandidat pakan. Tambahkan inventori "Pakan" atau produk pakan dengan kandungan nutrisi.</div>
                <div v-else class="space-y-3">
                  <div v-for="(feed, idx) in result.top_feeds" :key="idx" class="flex items-center gap-4 p-4 rounded-3xl border" :class="idx === 0 ? 'border-green-300 bg-green-50/50' : 'border-gray-100'">
                    <div class="w-9 h-9 rounded-2xl flex items-center justify-center font-black text-white shrink-0" :class="idx === 0 ? 'bg-green-600' : 'bg-gray-300'">#{{ idx + 1 }}</div>
                    <div class="flex-1 min-w-0">
                      <p class="font-black text-gray-800 truncate">{{ feed.name }} <span class="text-[10px] text-gray-400 uppercase">({{ feed.source }})</span></p>
                      <p class="text-[11px] text-gray-500">Protein {{ feed.nutrition?.protein }}% · TDN {{ feed.nutrition?.energy_tdn }}% · ADG {{ feed.projected_adg_kg }} kg/hr</p>
                    </div>
                    <div class="text-right shrink-0"><p class="text-lg font-black" :class="idx === 0 ? 'text-green-700' : 'text-slate-700'">{{ feed.compatibility_percent }}%</p><p class="text-[10px] font-black text-gray-400">Grade {{ feed.grade }}</p></div>
                  </div>
                </div>
              </div>

              <div v-if="(result.recommendations||[]).length" class="bg-orange-50/40 border border-orange-100 rounded-3xl p-6">
                <h3 class="text-xs font-black text-orange-700 uppercase tracking-[0.2em] mb-3">Rekomendasi</h3>
                <ul class="space-y-2"><li v-for="(r,i) in result.recommendations" :key="i" class="text-sm font-medium text-slate-700 flex gap-2"><span class="text-orange-500 font-black">•</span> {{ r }}</li></ul>
              </div>
            </div>
            <div v-else class="bg-white rounded-[32px] border border-gray-100 p-16 text-center text-gray-300 font-bold">Klik "Analisa Pakan" untuk memulai.</div>
          </template>
        </section>
      </div>
    </main>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { MenuIcon, SoupIcon } from 'lucide-vue-next'

const isSidebarOpen = ref(false)
const { list: fetchPens, analyzeFeed, getFeedAnalysis } = usePen()

const pens = ref([])
const loading = ref(false)
const selected = ref(null)
const result = ref(null)
const analyzing = ref(false)
const error = ref('')

onMounted(async () => {
  loading.value = true
  try { pens.value = (await fetchPens()) || [] } catch (e) { console.error(e) } finally { loading.value = false }
})

const selectPen = async (p) => {
  selected.value = p
  result.value = null
  error.value = ''
  try {
    const hist = await getFeedAnalysis(p.id)
    if (hist && hist.length) result.value = hist[0].data || hist[0]
  } catch { /* ignore */ }
}

const runAnalysis = async () => {
  if (!selected.value) return
  analyzing.value = true
  error.value = ''
  try {
    result.value = await analyzeFeed(selected.value.id)
  } catch (e) {
    error.value = e?.data?.error?.message || e?.data?.message || 'Gagal menganalisa pakan.'
  } finally {
    analyzing.value = false
  }
}
</script>

<style scoped>
.no-scrollbar::-webkit-scrollbar { display: none; }
.no-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }
</style>
