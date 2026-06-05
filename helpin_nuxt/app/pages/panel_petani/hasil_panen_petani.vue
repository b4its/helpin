<template>
  <div class="flex h-screen w-full bg-[#f4f7f5] font-sans overflow-hidden relative">
    
<SidebarPetani :isOpen="isSidebarOpen" @close="isSidebarOpen = false" />

    <main class="flex-1 flex flex-col overflow-y-auto relative w-full">
      <header class="flex justify-between items-center px-6 md:px-10 py-4 md:py-6 border-b border-gray-200 bg-white shadow-sm z-10 sticky top-0">
        <div class="flex items-center gap-4">
          <button @click="isSidebarOpen = true" class="md:hidden p-2 -ml-2 text-gray-600 hover:bg-gray-100 rounded-lg transition">
            <MenuIcon class="w-6 h-6" />
          </button>
          <div>
            <h1 class="text-xl md:text-2xl font-extrabold text-gray-800 leading-tight">Hasil Panen</h1>
            <p class="text-xs md:text-sm text-gray-500 font-medium mt-0.5">Analitik dan catatan produksi pascapanen</p>
          </div>
        </div>
        <div>
          <div class="flex items-center gap-2 bg-green-100 px-3 py-1.5 md:px-4 md:py-2 rounded-lg border border-green-200">
            <div class="w-2 h-2 rounded-full bg-green-600 shrink-0 animate-pulse"></div>
            <span class="text-xs md:text-sm font-bold text-green-600 hidden sm:block">STATUS ONLINE</span>
            <span class="text-xs md:text-sm font-bold text-green-600 sm:hidden">ONLINE</span>
          </div>
        </div>
      </header>

      <div class="p-4 md:p-10 flex flex-col w-full max-w-[100vw] gap-6">
        
        <section class="grid grid-cols-1 lg:grid-cols-3 gap-6 w-full">
          <div class="bg-white rounded-2xl shadow-sm border border-gray-100 p-5 lg:col-span-2">
            <h3 class="text-sm font-bold text-gray-800 mb-4 flex items-center gap-2">
              <HistoryIcon class="w-4 h-4 text-green-700" />
              Tren Volume Panen (Ton)
            </h3>
            <div class="h-64 w-full relative">
              <Line v-if="chartDataLine" :data="chartDataLine" :options="chartOptionsLine" />
            </div>
          </div>

          <div class="bg-white rounded-2xl shadow-sm border border-gray-100 p-5">
            <h3 class="text-sm font-bold text-gray-800 mb-4 flex items-center gap-2">
              <LeafIcon class="w-4 h-4 text-green-700" />
              Distribusi Jenis Tanaman
            </h3>
            <div class="h-64 w-full relative flex justify-center">
              <Pie v-if="chartDataPie" :data="chartDataPie" :options="chartOptionsPie" />
            </div>
          </div>

          <div class="bg-white rounded-2xl shadow-sm border border-gray-100 p-5 lg:col-span-3">
            <h3 class="text-sm font-bold text-gray-800 mb-4 flex items-center gap-2">
              <ArchiveIcon class="w-4 h-4 text-green-700" />
              Akumulasi Kualitas Panen (Grade A, B, Afkir)
            </h3>
            <div class="h-48 w-full relative">
              <Bar v-if="chartDataBar" :data="chartDataBar" :options="chartOptionsBar" />
            </div>
          </div>
        </section>

        <section class="bg-white rounded-2xl shadow-sm border border-gray-100 p-4 md:p-6 w-full flex flex-col">
          <div class="flex items-center gap-2 mb-6">
            <div class="w-2 h-2 rounded-full bg-[#1a402d]"></div>
            <h2 class="text-base md:text-lg font-bold text-gray-800">Daftar Panen Keseluruhan</h2>
          </div>
          
          <div class="overflow-x-auto -mx-4 md:mx-0 px-4 md:px-0">
            <table class="w-full text-sm text-left min-w-[900px]">
              <thead class="text-xs text-green-800 uppercase bg-[#e1f0e5] font-bold">
                <tr>
                  <th scope="col" class="px-4 md:px-6 py-4 rounded-l-lg">ID Panen</th>
                  <th scope="col" class="px-4 md:px-6 py-4">Lahan & Tanaman</th>
                  <th scope="col" class="px-4 md:px-6 py-4">Tgl Panen</th>
                  <th scope="col" class="px-4 md:px-6 py-4">Total (Ton)</th>
                  <th scope="col" class="px-4 md:px-6 py-4">Detail Grade (A/B/Afkir)</th>
                  <th scope="col" class="px-4 md:px-6 py-4">Status</th>
                  <th scope="col" class="px-4 md:px-6 py-4 rounded-r-lg text-center">Aksi</th>
                </tr>
              </thead>
              <tbody>
                <tr v-if="panenList.length === 0">
                  <td colspan="7" class="text-center py-10 text-gray-500 font-medium">Belum ada pencatatan hasil panen.</td>
                </tr>
                <tr v-for="item in panenList" :key="item.id" class="border-b border-gray-50 last:border-0 hover:bg-gray-50 transition-colors">
                  <td class="px-4 md:px-6 py-4 font-bold text-gray-500">{{ item.id }}</td>
                  <td class="px-4 md:px-6 py-4">
                    <span class="font-bold text-gray-800 block">{{ item.cropName }}</span>
                    <span class="text-xs font-medium text-gray-500 flex items-center gap-1 mt-0.5">
                      <MapPinIcon class="w-3 h-3" /> {{ item.landName }}
                    </span>
                  </td>
                  <td class="px-4 md:px-6 py-4 font-semibold text-gray-600">{{ item.harvestDate }}</td>
                  <td class="px-4 md:px-6 py-4">
                    <span class="font-extrabold text-[#1a402d] text-base">{{ item.totalAmount }}</span>
                  </td>
                  <td class="px-4 md:px-6 py-4">
                    <div class="flex items-center gap-2 text-xs font-bold">
                      <span class="text-green-600 w-8" title="Grade A">{{ item.gradeA }}t</span>
                      <div class="flex-1 h-1.5 bg-gray-100 rounded-full overflow-hidden flex">
                        <div class="bg-green-500 h-full" :style="`width: ${(item.gradeA / item.totalAmount) * 100}%`"></div>
                        <div class="bg-yellow-400 h-full" :style="`width: ${(item.gradeB / item.totalAmount) * 100}%`"></div>
                        <div class="bg-red-500 h-full" :style="`width: ${(item.reject / item.totalAmount) * 100}%`"></div>
                      </div>
                      <span class="text-red-500 w-8 text-right" title="Afkir/Reject">{{ item.reject }}t</span>
                    </div>
                  </td>
                  <td class="px-4 md:px-6 py-4">
                    <span class="px-2.5 py-1 rounded-full text-xs font-bold" 
                      :class="{
                        'bg-blue-100 text-blue-700': item.status === 'Proses Sortir',
                        'bg-green-100 text-green-700': item.status === 'Masuk Gudang',
                        'bg-purple-100 text-purple-700': item.status === 'Terjual'
                      }">
                      {{ item.status }}
                    </span>
                  </td>
                  <td class="px-4 md:px-6 py-4 flex justify-center gap-2">
                    <button @click="openModal('view', item)" class="bg-[#3b82f6] hover:bg-blue-600 text-white p-2 rounded-md transition shadow-sm" title="Lihat Detail">
                      <MenuIcon class="w-4 h-4" />
                    </button>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </section>
      </div>
    </main>

    <div v-if="isFormModalOpen" class="fixed inset-0 z-50 flex items-center justify-center p-2 md:p-4">
      <div class="absolute inset-0 bg-black/60 backdrop-blur-sm" @click="closeModal"></div>
      
      <div class="bg-white rounded-2xl w-full max-w-2xl shadow-2xl relative z-10 flex flex-col max-h-[95vh] animate-in fade-in zoom-in duration-200">
        <div class="flex justify-between items-center p-4 md:p-6 border-b border-gray-100 shrink-0">
          <div class="flex items-center gap-3">
            <div class="p-2 bg-green-100 text-green-700 rounded-lg">
              <WheatIcon class="w-6 h-6" />
            </div>
            <h2 class="text-lg md:text-xl font-extrabold text-gray-800">
              {{ isViewMode ? 'Detail Hasil Panen' : 'Input Hasil Panen Baru' }}
            </h2>
          </div>
          <button @click="closeModal" class="p-2 text-gray-400 hover:bg-gray-100 hover:text-gray-700 rounded-lg transition shrink-0">
            <XIcon class="w-5 h-5 md:w-6 md:h-6" />
          </button>
        </div>

        <div class="p-4 md:p-6 overflow-y-auto">
          <form @submit.prevent="saveData" class="space-y-5">
            <div class="grid grid-cols-1 md:grid-cols-2 gap-5">
              <div>
                <label class="block mb-1.5 text-sm font-bold text-gray-700">Pilih Lahan</label>
                <select v-model="formData.landName" :disabled="isViewMode" required class="bg-gray-50 border border-gray-200 text-gray-900 text-sm rounded-lg focus:ring-[#1a402d] focus:border-[#1a402d] block w-full p-2.5 outline-none transition disabled:opacity-70 disabled:cursor-not-allowed">
                  <option value="" disabled>Pilih Lahan Tanam</option>
                  <option value="Blok A - Lahan Utara">Blok A - Lahan Utara</option>
                  <option value="Blok B - Lahan Kering">Blok B - Lahan Kering</option>
                  <option value="Blok C - Sawah Timur">Blok C - Sawah Timur</option>
                </select>
              </div>
              
              <div>
                <label class="block mb-1.5 text-sm font-bold text-gray-700">Nama Tanaman</label>
                <input v-model="formData.cropName" type="text" :disabled="isViewMode" required class="bg-gray-50 border border-gray-200 text-gray-900 text-sm rounded-lg focus:ring-[#1a402d] focus:border-[#1a402d] block w-full p-2.5 outline-none transition disabled:opacity-70 disabled:cursor-not-allowed" placeholder="Cth: Jagung Manis" />
              </div>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-5">
              <div>
                <label class="block mb-1.5 text-sm font-bold text-gray-700">Tanggal Panen</label>
                <input v-model="formData.harvestDate" type="date" :disabled="isViewMode" required class="bg-gray-50 border border-gray-200 text-gray-900 text-sm rounded-lg focus:ring-[#1a402d] focus:border-[#1a402d] block w-full p-2.5 outline-none transition disabled:opacity-70 disabled:cursor-not-allowed" />
              </div>
              <div>
                <label class="block mb-1.5 text-sm font-bold text-gray-700">Status Pascapanen</label>
                <select v-model="formData.status" :disabled="isViewMode" required class="bg-gray-50 border border-gray-200 text-gray-900 text-sm rounded-lg focus:ring-[#1a402d] focus:border-[#1a402d] block w-full p-2.5 outline-none transition disabled:opacity-70 disabled:cursor-not-allowed">
                  <option value="" disabled>Pilih Status</option>
                  <option value="Proses Sortir">Proses Sortir</option>
                  <option value="Masuk Gudang">Masuk Gudang</option>
                  <option value="Terjual">Terjual</option>
                </select>
              </div>
            </div>

            <div class="bg-gray-50 p-4 rounded-xl border border-gray-200">
              <div class="flex justify-between items-center mb-4">
                <h3 class="text-sm font-bold text-gray-800">Distribusi Kualitas (Tonase)</h3>
                <span class="text-xs font-bold px-2 py-1 bg-[#1a402d] text-white rounded">Total: {{ computedTotal }} Ton</span>
              </div>
              <div class="grid grid-cols-3 gap-4">
                <div>
                  <label class="block mb-1 text-xs font-bold text-green-700">Grade A</label>
                  <input v-model.number="formData.gradeA" type="number" step="0.1" min="0" :disabled="isViewMode" required class="bg-white border border-gray-200 text-gray-900 text-sm rounded-lg focus:ring-[#1a402d] focus:border-[#1a402d] block w-full p-2 outline-none transition disabled:opacity-70 disabled:cursor-not-allowed" placeholder="0" />
                </div>
                <div>
                  <label class="block mb-1 text-xs font-bold text-yellow-600">Grade B</label>
                  <input v-model.number="formData.gradeB" type="number" step="0.1" min="0" :disabled="isViewMode" required class="bg-white border border-gray-200 text-gray-900 text-sm rounded-lg focus:ring-[#1a402d] focus:border-[#1a402d] block w-full p-2 outline-none transition disabled:opacity-70 disabled:cursor-not-allowed" placeholder="0" />
                </div>
                <div>
                  <label class="block mb-1 text-xs font-bold text-red-600">Afkir / Reject</label>
                  <input v-model.number="formData.reject" type="number" step="0.1" min="0" :disabled="isViewMode" required class="bg-white border border-gray-200 text-gray-900 text-sm rounded-lg focus:ring-[#1a402d] focus:border-[#1a402d] block w-full p-2 outline-none transition disabled:opacity-70 disabled:cursor-not-allowed" placeholder="0" />
                </div>
              </div>
            </div>
            
            <div v-if="isViewMode" class="mt-4 p-4 bg-yellow-50 border border-yellow-200 rounded-lg text-sm text-yellow-800 font-medium flex items-start gap-2">
              <AlertTriangleIcon class="w-5 h-5 shrink-0" />
              <span>Data hasil panen ini bersifat permanen dan tidak dapat diedit untuk menjaga integritas rekam jejak.</span>
            </div>
          </form>
        </div>

        <div class="p-4 md:p-6 border-t border-gray-100 flex justify-end gap-3 bg-gray-50 rounded-b-2xl shrink-0">
          <button @click="closeModal" type="button" class="px-5 py-2.5 font-bold text-gray-600 hover:bg-gray-200 rounded-lg transition text-center">
            {{ isViewMode ? 'Tutup' : 'Batal' }}
          </button>
          <button v-if="!isViewMode" @click="saveData" type="button" class="px-5 py-2.5 font-bold text-white bg-[#1a402d] hover:bg-[#143222] rounded-lg transition shadow-md flex items-center justify-center gap-2">
            Simpan Data
          </button>
        </div>
      </div>
    </div>

    <div v-if="isDeleteModalOpen" class="fixed inset-0 z-50 flex items-center justify-center p-4">
      <div class="absolute inset-0 bg-black/60 backdrop-blur-sm" @click="closeDeleteModal"></div>
      
      <div class="bg-white rounded-2xl w-full max-w-sm shadow-2xl relative z-10 flex flex-col animate-in fade-in zoom-in duration-200 p-6 text-center">
        <div class="w-16 h-16 bg-red-100 rounded-full flex items-center justify-center mx-auto mb-4 text-red-500">
          <AlertTriangleIcon class="w-8 h-8" />
        </div>
        <h2 class="text-xl font-extrabold text-gray-800 mb-2">Hapus Catatan Panen?</h2>
        <p class="text-sm text-gray-500 mb-6">Yakin menghapus catatan panen <strong>{{ itemToDelete?.cropName }}</strong>? Data yang dihapus akan mempengaruhi riwayat inventori.</p>
        
        <div class="flex gap-3 justify-center">
          <button @click="closeDeleteModal" class="px-5 py-2.5 font-bold text-gray-600 bg-gray-100 hover:bg-gray-200 rounded-lg transition flex-1">
            Batal
          </button>
          <button @click="executeDelete" class="px-5 py-2.5 font-bold text-white bg-red-500 hover:bg-red-600 rounded-lg transition flex-1">
            Ya, Hapus
          </button>
        </div>
      </div>
    </div>

  </div>
</template>

<script setup>
import { ref, reactive, computed, onMounted } from 'vue'
import { 
  LayoutDashboardIcon, ArchiveIcon, MapIcon, LeafIcon, WheatIcon, 
  HistoryIcon, LogOutIcon, MenuIcon, XIcon, PlusIcon, 
  Trash2Icon, AlertTriangleIcon, MapPinIcon
} from 'lucide-vue-next'

// --- CHART.JS IMPORTS ---
import { Line, Pie, Bar } from 'vue-chartjs'
import { 
  Chart as ChartJS, Title, Tooltip, Legend, BarElement, 
  CategoryScale, LinearScale, PointElement, LineElement, ArcElement, Colors
} from 'chart.js'

ChartJS.register(
  Title, Tooltip, Legend, BarElement, CategoryScale, 
  LinearScale, PointElement, LineElement, ArcElement, Colors
)

const isSidebarOpen = ref(false)

// ==========================================
// 1. DATA MODEL (CLASS)
// ==========================================
class HasilPanenModel {
  constructor(data) {
    this.id = data.id || `PNN-${Math.floor(Math.random() * 90000) + 10000}`;
    this.landName = data.landName;
    this.cropName = data.cropName;
    this.harvestDate = data.harvestDate;
    this.gradeA = parseFloat(data.gradeA) || 0;
    this.gradeB = parseFloat(data.gradeB) || 0;
    this.reject = parseFloat(data.reject) || 0;
    this.status = data.status;
    
    // Total amount dikalkulasi otomatis dari grade
    this.totalAmount = parseFloat((this.gradeA + this.gradeB + this.reject).toFixed(2));
  }

  static fromJSON(jsonApiData) {
    return new HasilPanenModel({
      id: jsonApiData.id_panen,
      landName: jsonApiData.nama_lahan,
      cropName: jsonApiData.nama_tanaman,
      harvestDate: jsonApiData.tanggal_panen,
      gradeA: jsonApiData.kualitas_a,
      gradeB: jsonApiData.kualitas_b,
      reject: jsonApiData.kualitas_afkir,
      status: jsonApiData.status_pascapanen
    });
  }
}

// ==========================================
// 2. SIMULASI DATA API (Dummy Data)
// ==========================================
const dummyApiJsonResponse = [
  { id_panen: 'PNN-88201', nama_lahan: 'Blok A - Lahan Utara', nama_tanaman: 'Jagung Manis', tanggal_panen: '2026-05-12', kualitas_a: 6.5, kualitas_b: 2.5, kualitas_afkir: 1.0, status_pascapanen: 'Proses Sortir' },
  { id_panen: 'PNN-88202', nama_lahan: 'Blok C - Sawah Timur', nama_tanaman: 'Padi Hibrida', tanggal_panen: '2026-05-01', kualitas_a: 18.0, kualitas_b: 5.0, kualitas_afkir: 2.0, status_pascapanen: 'Masuk Gudang' },
  { id_panen: 'PNN-88203', nama_lahan: 'Blok B - Lahan Kering', nama_tanaman: 'Kedelai', tanggal_panen: '2026-04-18', kualitas_a: 5.0, kualitas_b: 2.0, kualitas_afkir: 1.0, status_pascapanen: 'Terjual' },
  { id_panen: 'PNN-88204', nama_lahan: 'Blok A - Lahan Utara', nama_tanaman: 'Jagung Manis', tanggal_panen: '2026-04-10', kualitas_a: 7.0, kualitas_b: 1.5, kualitas_afkir: 0.5, status_pascapanen: 'Terjual' },
]

// ==========================================
// 3. STATE MANAGEMENT
// ==========================================
const panenList = ref([])

onMounted(() => {
  panenList.value = dummyApiJsonResponse.map(data => HasilPanenModel.fromJSON(data))
})

// ==========================================
// 4. CHART COMPUTED DATA & OPTIONS
// ==========================================

// -- LINE CHART (TREN WAKTU) --
const chartDataLine = computed(() => {
  const sortedData = [...panenList.value].sort((a, b) => new Date(a.harvestDate) - new Date(b.harvestDate));
  return {
    labels: sortedData.map(item => item.harvestDate),
    datasets: [{
      label: 'Total Panen (Ton)',
      data: sortedData.map(item => item.totalAmount),
      borderColor: '#1a402d',
      backgroundColor: 'rgba(26, 64, 45, 0.2)',
      tension: 0.4,
      fill: true
    }]
  }
})

const chartOptionsLine = {
  responsive: true,
  maintainAspectRatio: false,
  plugins: { legend: { position: 'bottom' } }
}

// -- PIE CHART (DISTRIBUSI TANAMAN) --
const chartDataPie = computed(() => {
  const cropTotals = {};
  panenList.value.forEach(item => {
    cropTotals[item.cropName] = (cropTotals[item.cropName] || 0) + item.totalAmount;
  });
  return {
    labels: Object.keys(cropTotals),
    datasets: [{
      data: Object.values(cropTotals),
      backgroundColor: ['#22c55e', '#eab308', '#3b82f6', '#f97316', '#a855f7'],
      borderWidth: 1
    }]
  }
})

const chartOptionsPie = {
  responsive: true,
  maintainAspectRatio: false,
  plugins: { legend: { position: 'right' } }
}

// -- BAR CHART (KUALITAS GRADE A/B/AFKIR) --
const chartDataBar = computed(() => {
  let sumA = 0, sumB = 0, sumReject = 0;
  panenList.value.forEach(item => {
    sumA += item.gradeA;
    sumB += item.gradeB;
    sumReject += item.reject;
  });
  return {
    labels: ['Grade A', 'Grade B', 'Afkir / Reject'],
    datasets: [{
      label: 'Tonase Keseluruhan',
      data: [sumA, sumB, sumReject],
      backgroundColor: ['#22c55e', '#eab308', '#ef4444'],
      borderRadius: 6
    }]
  }
})

const chartOptionsBar = {
  responsive: true,
  maintainAspectRatio: false,
  plugins: { legend: { display: false } },
  scales: { y: { beginAtZero: true } }
}

// ==========================================
// 5. VIEW & CREATE LOGIC
// ==========================================
const isFormModalOpen = ref(false)
const isViewMode = ref(false)

const formData = reactive({
  landName: '', cropName: '', harvestDate: '', 
  gradeA: null, gradeB: null, reject: null, status: ''
})

const computedTotal = computed(() => {
  const total = (parseFloat(formData.gradeA) || 0) + (parseFloat(formData.gradeB) || 0) + (parseFloat(formData.reject) || 0)
  return total.toFixed(2)
})

const openModal = (mode, item = null) => {
  if (mode === 'view' && item) {
    isViewMode.value = true
    formData.landName = item.landName
    formData.cropName = item.cropName
    formData.harvestDate = item.harvestDate
    formData.gradeA = item.gradeA
    formData.gradeB = item.gradeB
    formData.reject = item.reject
    formData.status = item.status
  } else {
    isViewMode.value = false
    formData.landName = ''
    formData.cropName = ''
    formData.harvestDate = new Date().toISOString().split('T')[0]
    formData.gradeA = null
    formData.gradeB = null
    formData.reject = null
    formData.status = ''
  }
  isFormModalOpen.value = true
}

const closeModal = () => {
  isFormModalOpen.value = false
}

const saveData = () => {
  if (isViewMode.value) return; // Proteksi tambahan
  
  if (!formData.landName || !formData.cropName || formData.gradeA === null || !formData.status) {
    alert("Mohon lengkapi field yang wajib diisi!")
    return
  }

  const newItem = new HasilPanenModel({
    landName: formData.landName,
    cropName: formData.cropName,
    harvestDate: formData.harvestDate,
    gradeA: formData.gradeA,
    gradeB: formData.gradeB,
    reject: formData.reject,
    status: formData.status
  })
  
  panenList.value.unshift(newItem)
  closeModal()
}

</script>

<style scoped>
a {
  transition: all 0.2s ease-in-out;
}

.no-scrollbar::-webkit-scrollbar { display: none; }
.no-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }

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

.animate-in { animation: animateIn 0.3s cubic-bezier(0.16, 1, 0.3, 1) forwards; }
@keyframes animateIn {
  from { opacity: 0; transform: scale(0.95) translateY(10px); }
  to { opacity: 1; transform: scale(1) translateY(0); }
}
</style>