<template>
  <div class="flex h-screen w-full bg-[#f4f7f5] font-sans overflow-hidden relative text-slate-900">

    <SidebarPeternak :isOpen="isSidebarOpen" @close="isSidebarOpen = false" />

    <main class="flex-1 flex flex-col overflow-y-auto relative w-full no-scrollbar">
      <header class="flex justify-between items-center px-6 md:px-10 py-6 border-b border-gray-200 bg-white shadow-sm z-10 sticky top-0">
        <div class="flex items-center gap-4">
          <button @click="isSidebarOpen = true" class="md:hidden p-2 text-gray-600 hover:bg-gray-100 rounded-lg transition"><MenuIcon class="w-6 h-6" /></button>
          <div>
            <h1 class="text-xl md:text-2xl font-extrabold text-gray-800 leading-tight">Kondisi Kesehatan Ternak</h1>
            <p class="text-xs md:text-sm text-gray-500 font-medium mt-0.5">Diagnosa biometrik otomatis (AI online / ensemble offline)</p>
          </div>
        </div>
        <div class="flex items-center gap-2 bg-green-100 px-3 py-1.5 md:px-4 md:py-2 rounded-lg border border-green-200">
          <div class="w-2 h-2 rounded-full bg-green-600 shrink-0 animate-pulse"></div>
          <span class="text-xs md:text-sm font-bold text-green-600">ONLINE</span>
        </div>
      </header>

      <div class="p-4 md:p-10 grid grid-cols-1 lg:grid-cols-3 gap-6">

        <!-- Daftar ternak -->
        <section class="lg:col-span-1 bg-white rounded-[32px] shadow-sm border border-gray-100 p-5 flex flex-col">
          <h2 class="text-sm font-black text-gray-700 uppercase tracking-widest mb-4">Pilih Ternak</h2>
          <div v-if="loading" class="py-10 flex justify-center"><div class="animate-spin rounded-full h-6 w-6 border-b-2 border-[#1a402d]"></div></div>
          <div v-else class="space-y-2 overflow-y-auto no-scrollbar max-h-[70vh]">
            <button v-for="t in livestockList" :key="t.id" @click="selectTernak(t)"
              :class="['w-full text-left p-4 rounded-2xl border transition-all flex items-center justify-between', selected?.id === t.id ? 'border-[#1a402d] bg-green-50' : 'border-gray-100 hover:border-gray-200']">
              <div>
                <p class="font-black text-gray-800 uppercase">{{ t.tagId }}</p>
                <p class="text-[11px] text-gray-400 font-bold">{{ t.category }} · {{ t.breed }}</p>
              </div>
              <span :class="['px-2.5 py-1 rounded-full text-[10px] font-black', statusClass(t.healthStatus)]">{{ t.healthScore }}</span>
            </button>
            <p v-if="livestockList.length === 0" class="text-center text-gray-300 font-bold py-8">Belum ada ternak</p>
          </div>
        </section>

        <!-- Detail & generate -->
        <section class="lg:col-span-2 space-y-6">
          <div v-if="!selected" class="bg-white rounded-[32px] border border-gray-100 p-16 text-center text-gray-300 font-bold">
            Pilih ternak di samping untuk melihat & generate kondisi kesehatan.
          </div>

          <template v-else>
            <div class="bg-white rounded-[32px] shadow-sm border border-gray-100 p-6 flex items-center justify-between gap-4 flex-wrap">
              <div>
                <h2 class="text-2xl font-black text-gray-800 uppercase tracking-tighter">{{ selected.tagId }}</h2>
                <p class="text-sm text-gray-400 font-bold">{{ selected.category }} · {{ selected.breed }} · {{ selected.ageMonths ? selected.ageMonths + ' bln' : 'usia -' }} · {{ selected.weight }} kg</p>
              </div>
              <button @click="generate" :disabled="generating" class="bg-[#1a402d] text-white px-6 py-3 rounded-2xl font-black uppercase text-xs shadow-lg flex items-center gap-2 disabled:opacity-50">
                <ActivityIcon class="w-4 h-4" /> {{ generating ? 'Menganalisa...' : 'Generate Kondisi' }}
              </button>
            </div>

            <div v-if="result" class="bg-white rounded-[32px] shadow-sm border border-gray-100 p-8 space-y-6">
              <div class="flex items-center gap-2">
                <span class="px-3 py-1 rounded-full text-[10px] font-black uppercase tracking-widest" :class="statusClass(result.status)">{{ result.status }}</span>
                <span class="text-xs text-gray-400 font-mono">{{ result.analysis_source }}</span>
                <span v-if="result.auto_generated" class="text-[10px] text-blue-500 font-black uppercase">auto-generated</span>
              </div>

              <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
                <div class="bg-gray-50 p-5 rounded-3xl border border-gray-100 text-center">
                  <p class="text-[10px] font-black text-gray-400 uppercase">Skor</p>
                  <p class="text-3xl font-black text-[#1a402d]">{{ result.health_score }}</p>
                </div>
                <div class="bg-red-50/50 p-5 rounded-3xl border border-red-100 text-center">
                  <p class="text-[10px] font-black text-red-400 uppercase">Detak Jantung</p>
                  <p class="text-2xl font-black text-red-700">{{ result.vitals?.heart_rate }}<small class="text-xs"> bpm</small></p>
                </div>
                <div class="bg-orange-50/50 p-5 rounded-3xl border border-orange-100 text-center">
                  <p class="text-[10px] font-black text-orange-400 uppercase">Suhu</p>
                  <p class="text-2xl font-black text-orange-700">{{ result.vitals?.body_temp }}<small class="text-xs"> °C</small></p>
                </div>
                <div class="bg-blue-50/50 p-5 rounded-3xl border border-blue-100 text-center">
                  <p class="text-[10px] font-black text-blue-400 uppercase">Respirasi</p>
                  <p class="text-2xl font-black text-blue-700">{{ result.vitals?.respiratory_rate }}<small class="text-xs"> /mnt</small></p>
                </div>
              </div>

              <div class="flex items-center gap-3">
                <span class="text-[10px] font-black text-gray-400 uppercase">Kondisi Tubuh:</span>
                <span class="font-black text-gray-800">{{ result.body_condition }}</span>
              </div>

              <div v-if="(result.possible_conditions||[]).length" class="bg-red-50/40 border border-red-100 rounded-3xl p-5">
                <p class="text-[10px] font-black text-red-500 uppercase mb-2">Kemungkinan Kondisi</p>
                <div class="flex flex-wrap gap-2">
                  <span v-for="(c,i) in result.possible_conditions" :key="i" class="px-3 py-1 bg-white border border-red-200 text-red-600 rounded-full text-xs font-bold">{{ c }}</span>
                </div>
              </div>

              <div class="bg-green-50/40 border border-green-100 rounded-3xl p-5">
                <p class="text-[10px] font-black text-green-600 uppercase mb-2">Rekomendasi</p>
                <ul class="space-y-2">
                  <li v-for="(r,i) in result.recommendations" :key="i" class="text-sm font-medium text-slate-700 flex gap-2"><span class="text-green-600 font-black">•</span> {{ r }}</li>
                </ul>
              </div>
            </div>

            <!-- Riwayat -->
            <div class="bg-white rounded-[32px] shadow-sm border border-gray-100 p-6">
              <h3 class="text-sm font-black text-gray-700 uppercase tracking-widest mb-4">Riwayat Kesehatan</h3>
              <div v-if="history.length === 0" class="text-center text-gray-300 font-bold py-6">Belum ada riwayat</div>
              <div v-else class="space-y-2">
                <div v-for="h in history" :key="h.id" class="flex items-center justify-between p-3 rounded-2xl border border-gray-100">
                  <div class="text-sm">
                    <span class="font-black text-gray-700">Skor {{ h.health_score }}</span>
                    <span class="text-gray-400 font-medium"> · HR {{ h.heart_rate || '-' }} · {{ h.body_temp || '-' }}°C · RR {{ h.respiratory_rate || '-' }}</span>
                  </div>
                  <span class="text-[11px] text-gray-400 font-bold">{{ formatDate(h.recorded_at) }}</span>
                </div>
              </div>
            </div>
          </template>
        </section>
      </div>
    </main>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { MenuIcon, ActivityIcon } from 'lucide-vue-next'

const isSidebarOpen = ref(false)
const { list: fetchLivestock, generateHealth, getHealthHistory } = useLivestock()

const livestockList = ref([])
const loading = ref(false)
const selected = ref(null)
const result = ref(null)
const history = ref([])
const generating = ref(false)

const mapItem = (i) => ({
  id: i.id, tagId: i.tag_id, category: i.category, breed: i.breed,
  weight: parseFloat(i.weight), ageMonths: i.age_months,
  healthStatus: i.health_status || 'Sehat', healthScore: i.health_score ?? 100
})

const statusClass = (s) => {
  if (s === 'Sakit' || s === 'Buruk') return 'bg-red-50 text-red-600'
  if (s === 'Perlu Perhatian' || s === 'Observasi') return 'bg-orange-50 text-orange-600'
  return 'bg-green-50 text-green-600'
}

onMounted(async () => {
  loading.value = true
  try { livestockList.value = ((await fetchLivestock()) || []).map(mapItem) }
  catch (e) { console.error(e) }
  finally { loading.value = false }
})

const selectTernak = async (t) => {
  selected.value = t
  result.value = null
  history.value = []
  try { history.value = (await getHealthHistory(t.id)) || [] } catch { history.value = [] }
}

const generate = async () => {
  if (!selected.value) return
  generating.value = true
  try {
    result.value = await generateHealth(selected.value.id)
    // Update skor di list
    const idx = livestockList.value.findIndex(i => i.id === selected.value.id)
    if (idx !== -1) {
      livestockList.value[idx].healthScore = result.value.health_score
      livestockList.value[idx].healthStatus = result.value.status
    }
    history.value = (await getHealthHistory(selected.value.id)) || []
  } catch (e) {
    console.error(e); useToast().error('Gagal generate kondisi kesehatan', e?.data?.message)
  } finally { generating.value = false }
}

const formatDate = (d) => d ? new Date(d).toLocaleString('id-ID', { day: '2-digit', month: 'short', hour: '2-digit', minute: '2-digit' }) : '-'
</script>

<style scoped>
.no-scrollbar::-webkit-scrollbar { display: none; }
.no-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }
</style>
