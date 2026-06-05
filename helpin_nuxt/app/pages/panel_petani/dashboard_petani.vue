<template>
  <div class="flex h-screen w-full bg-[#f4f7f5] font-sans overflow-hidden relative">
    
    <div 
      v-if="isSidebarOpen" 
      @click="isSidebarOpen = false" 
      class="fixed inset-0 bg-black/50 z-30 md:hidden transition-opacity backdrop-blur-sm"
    ></div>

    <aside 
      :class="[
        'w-[280px] bg-[#1a402d] text-white flex flex-col justify-between shrink-0 absolute inset-y-0 left-0 z-40 transform transition-transform duration-300 ease-in-out md:relative md:translate-x-0',
        isSidebarOpen ? 'translate-x-0' : '-translate-x-full'
      ]"
    >
      <div class="overflow-y-auto overflow-x-hidden h-full no-scrollbar flex flex-col justify-between">
        <div>
          <div class="px-6 md:px-8 py-6 md:py-8 flex justify-between items-center">
            <div class="flex items-baseline gap-1">
              <h1 class="text-3xl font-extrabold tracking-wider">HELPIN</h1>
              <span class="text-[10px] uppercase tracking-widest font-semibold text-gray-300">Services</span>
            </div>
            <button @click="isSidebarOpen = false" class="md:hidden text-white hover:text-red-400">
              <XIcon class="w-6 h-6" />
            </button>
          </div>

          <div class="px-6 md:px-8 mb-6 md:mb-8 border-b border-[#23533b] pb-6 bg-[#173a28]">
            <div class="flex items-center gap-2 mb-2">
              <div class="w-2 h-2 rounded-full bg-green-500"></div>
              <span class="text-xs font-semibold text-green-400">PENGGUNA AKTIF</span>
            </div>
            <h2 class="text-xl font-bold mb-2">Petani Suki</h2>
            <span class="inline-block px-3 py-1 text-[10px] font-bold border border-green-600 text-green-400 rounded">
              PETANI
            </span>
          </div>

          <nav class="flex flex-col gap-2 pl-4">
            <a href="dashboard_petani" class="active-menu relative flex items-center gap-4 px-4 py-4 bg-[#f4f7f5] text-[#1a402d] rounded-l-full font-bold shadow-[-5px_0_10px_rgba(0,0,0,0.05)]">
              <LayoutDashboardIcon class="w-5 h-5" />
              Dashboard
            </a>
            <a href="inventori_petani" class="flex items-center gap-4 px-4 py-4 text-gray-200 hover:bg-[#23533b] rounded-l-full transition-colors font-medium">
              <ArchiveIcon class="w-5 h-5" />
              Inventori
            </a>
            <a href="lahan_petani" class="flex items-center gap-4 px-4 py-4 text-gray-200 hover:bg-[#23533b] rounded-l-full transition-colors font-medium">
              <MapIcon class="w-5 h-5" />
              Lahan
            </a>
            <a href="hasil_panen_petani" class="flex items-center gap-4 px-4 py-4 text-gray-200 hover:bg-[#23533b] rounded-l-full transition-colors font-medium">
              <WheatIcon class="w-5 h-5" />
              Hasil Panen
            </a>
            <a href="#" class="flex items-center gap-4 px-4 py-4 text-gray-200 hover:bg-[#23533b] rounded-l-full transition-colors font-medium">
              <HistoryIcon class="w-5 h-5" />
              Riwayat Panen
            </a>
          </nav>
        </div>

        <div class="p-6">
          <div class="bg-[#143222] rounded-xl p-4 flex items-center justify-between">
            <div class="min-w-0"> <div class="flex items-center gap-2 mb-1">
                <div class="w-1.5 h-1.5 rounded-full bg-green-500 shrink-0"></div>
                <span class="text-[10px] text-gray-400 font-semibold uppercase truncate">Logged in as</span>
              </div>
              <p class="text-sm font-bold truncate pr-2">Admin Suki SUPER</p>
            </div>
            <button class="text-red-400 hover:text-red-300 transition shrink-0">
              <LogOutIcon class="w-5 h-5" />
            </button>
          </div>
        </div>
      </div>
    </aside>

    <main class="flex-1 flex flex-col overflow-y-auto relative w-full">
      <header class="flex justify-between items-center px-6 md:px-10 py-4 md:py-6 border-b border-gray-200 bg-white shadow-sm z-10 sticky top-0">
        <div class="flex items-center gap-4">
          <button @click="isSidebarOpen = true" class="md:hidden p-2 -ml-2 text-gray-600 hover:bg-gray-100 rounded-lg transition">
            <MenuIcon class="w-6 h-6" />
          </button>
          <div>
            <h1 class="text-xl md:text-2xl font-extrabold text-gray-800 leading-tight">Dashboard</h1>
            <p class="text-xs md:text-sm text-gray-500 font-medium mt-0.5">Dashboard dari sistem</p>
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

      <div class="p-4 md:p-10 flex flex-col gap-6 md:gap-8 w-full max-w-[100vw]">
        
        <section class="bg-white rounded-2xl shadow-sm border border-gray-100 p-4 md:p-6 w-full">
          <div class="flex flex-col sm:flex-row justify-between items-start sm:items-center mb-6 gap-4">
            <div class="flex items-center gap-2">
              <div class="w-2 h-2 rounded-full bg-[#1a402d]"></div>
              <h2 class="text-base md:text-lg font-bold text-gray-800">Hasil Panen</h2>
            </div>
            <select class="bg-gray-50 border border-gray-200 text-gray-700 text-sm rounded-lg focus:ring-[#1a402d] focus:border-[#1a402d] block w-full sm:w-auto p-2.5 outline-none font-medium cursor-pointer">
              <option>Bulanan</option>
              <option>Tahunan</option>
            </select>
          </div>
          <div class="relative w-full h-[250px] md:h-[300px]">
            <ClientOnly>
              <Bar :data="chartData" :options="chartOptions" />
            </ClientOnly>
          </div>
        </section>

        <section class="bg-white rounded-2xl shadow-sm border border-gray-100 p-4 md:p-6 w-full">
          <div class="flex items-center gap-2 mb-6">
            <div class="w-2 h-2 rounded-full bg-[#1a402d]"></div>
            <h2 class="text-base md:text-lg font-bold text-gray-800">Prediksi Panen Berikutnya</h2>
          </div>
          
          <div class="overflow-x-auto -mx-4 md:mx-0 px-4 md:px-0">
            <table class="w-full text-sm text-left min-w-[700px]">
              <thead class="text-xs text-green-800 uppercase bg-[#e1f0e5] font-bold">
                <tr>
                  <th scope="col" class="px-4 md:px-6 py-4 rounded-l-lg">Tanggal Panen</th>
                  <th scope="col" class="px-4 md:px-6 py-4">Tanaman</th>
                  <th scope="col" class="px-4 md:px-6 py-4">Jumlah</th>
                  <th scope="col" class="px-4 md:px-6 py-4">Informasi</th>
                  <th scope="col" class="px-4 md:px-6 py-4 rounded-r-lg">Aksi</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="item in tableData" :key="item.id" class="border-b border-gray-50 last:border-0 hover:bg-gray-50 transition-colors">
                  <td class="px-4 md:px-6 py-4 font-bold text-gray-800 whitespace-nowrap">{{ item.date }}</td>
                  <td class="px-4 md:px-6 py-4 font-bold text-gray-800">{{ item.crop }}</td>
                  <td class="px-4 md:px-6 py-4 font-bold text-gray-800">{{ item.amount }} TON</td>
                  <td class="px-4 md:px-6 py-4 font-semibold text-red-500 whitespace-nowrap md:whitespace-normal">{{ item.info }}</td>
                  <td class="px-4 md:px-6 py-4">
                    <button @click="openModal(item)" class="bg-[#0ea5e9] hover:bg-blue-600 text-white p-2 rounded-md transition shadow-sm flex-shrink-0">
                      <EyeIcon class="w-4 h-4" />
                    </button>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </section>
      </div>
    </main>

    <div v-if="isModalOpen" class="fixed inset-0 z-50 flex items-center justify-center p-2 md:p-4">
      <div class="absolute inset-0 bg-black/60 backdrop-blur-sm" @click="closeModal"></div>
      
      <div class="bg-white rounded-2xl w-full max-w-4xl shadow-2xl relative z-10 flex flex-col max-h-[95vh] md:max-h-[90vh] animate-in fade-in zoom-in duration-200 overflow-hidden">
        
        <div class="flex justify-between items-center p-4 md:p-6 border-b border-gray-100 shrink-0">
          <div class="flex items-center gap-3">
            <div class="p-2 bg-green-100 text-green-700 rounded-lg hidden sm:block">
              <LeafIcon class="w-6 h-6" />
            </div>
            <div>
              <h2 class="text-lg md:text-xl font-extrabold text-gray-800">Detail Estimasi Panen</h2>
              <p class="text-xs md:text-sm text-gray-500 font-medium">ID Data: {{ selectedData?.id }}</p>
            </div>
          </div>
          <button @click="closeModal" class="p-2 text-gray-400 hover:bg-gray-100 hover:text-gray-700 rounded-lg transition shrink-0">
            <XIcon class="w-5 h-5 md:w-6 md:h-6" />
          </button>
        </div>

        <div class="p-4 md:p-6 overflow-y-auto">
          <div class="grid grid-cols-1 lg:grid-cols-2 gap-6 md:gap-8">
            
            <div class="space-y-6">
              <div class="bg-gray-50 p-4 md:p-5 rounded-xl border border-gray-100">
                <h3 class="text-xs md:text-sm font-bold text-gray-400 uppercase tracking-wider mb-4">Informasi Tanaman & Lahan</h3>
                <div class="space-y-3 md:space-y-4 text-sm md:text-base">
                  <div class="flex flex-col sm:flex-row sm:justify-between border-b border-gray-200 pb-2">
                    <span class="text-gray-600 font-medium mb-1 sm:mb-0">Nama Tanaman</span>
                    <span class="font-bold text-gray-900">{{ selectedData?.crop }}</span>
                  </div>
                  <div class="flex flex-col sm:flex-row sm:justify-between border-b border-gray-200 pb-2">
                    <span class="text-gray-600 font-medium mb-1 sm:mb-0">Kategori / Varietas</span>
                    <span class="font-bold text-gray-900">{{ selectedData?.details.varietas }}</span>
                  </div>
                  <div class="flex flex-col sm:flex-row sm:justify-between border-b border-gray-200 pb-2">
                    <span class="text-gray-600 font-medium mb-1 sm:mb-0">Lahan Tanam</span>
                    <span class="font-bold text-gray-900">{{ selectedData?.details.lahan }}</span>
                  </div>
                  <div class="flex flex-col sm:flex-row sm:justify-between border-b border-gray-200 pb-2">
                    <span class="text-gray-600 font-medium mb-1 sm:mb-0">Luas Area Tanam</span>
                    <span class="font-bold text-gray-900">{{ selectedData?.details.luas }} Hektar</span>
                  </div>
                  <div class="flex flex-col sm:flex-row sm:justify-between border-b border-gray-200 pb-2">
                    <span class="text-gray-600 font-medium mb-1 sm:mb-0">Tanggal Tanam</span>
                    <span class="font-bold text-gray-900">{{ selectedData?.details.tgl_tanam }}</span>
                  </div>
                  <div class="flex flex-col sm:flex-row sm:justify-between pb-1 md:pb-2">
                    <span class="text-gray-600 font-medium mb-1 sm:mb-0">Estimasi Total Panen</span>
                    <span class="font-extrabold text-[#1a402d] text-lg">{{ selectedData?.amount }} TON</span>
                  </div>
                </div>
              </div>
            </div>

            <div class="flex flex-col border border-gray-100 rounded-xl p-4 md:p-5 shadow-sm">
              <h3 class="text-xs md:text-sm font-bold text-gray-400 uppercase tracking-wider mb-2">Prediksi Kualitas Panen</h3>
              <p class="text-[11px] md:text-xs text-gray-500 mb-4">Distribusi grade panen berdasarkan analisis riwayat dan kondisi lahan saat ini.</p>
              
              <div class="flex-1 min-h-[200px] md:min-h-[250px] relative flex justify-center items-center">
                <ClientOnly>
                  <Pie v-if="modalChartData" :data="modalChartData" :options="pieChartOptions" />
                </ClientOnly>
              </div>
            </div>

          </div>
        </div>

        <div class="p-4 md:p-6 border-t border-gray-100 flex flex-col-reverse sm:flex-row justify-end gap-3 bg-gray-50 rounded-b-2xl shrink-0">
          <button @click="closeModal" class="w-full sm:w-auto px-5 py-2.5 font-bold text-gray-600 bg-white sm:bg-transparent border border-gray-300 sm:border-transparent hover:bg-gray-200 rounded-lg transition text-center">
            Tutup
          </button>
          <button class="w-full sm:w-auto px-5 py-2.5 font-bold text-white bg-[#1a402d] hover:bg-[#143222] rounded-lg transition shadow-md flex items-center justify-center gap-2">
            <ArchiveIcon class="w-4 h-4" /> Cetak Laporan
          </button>
        </div>
      </div>
    </div>

  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { LayoutDashboardIcon, ArchiveIcon, MapIcon, LeafIcon, WheatIcon, HistoryIcon, LogOutIcon, EyeIcon, XIcon, MenuIcon } from 'lucide-vue-next'
import { Bar, Pie } from 'vue-chartjs'
import { Chart as ChartJS, Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale, ArcElement } from 'chart.js'

ChartJS.register(Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale, ArcElement)

// State untuk memantau apakah Sidebar di HP terbuka
const isSidebarOpen = ref(false)

// ==========================================
// 1. DATA MODEL (CLASS / DTO)
// ==========================================
class PrediksiPanenModel {
  constructor(data) {
    this.id = data.id;
    this.date = data.date;
    this.crop = data.crop;
    this.amount = data.amount;
    this.info = data.info;
    this.details = {
      varietas: data.varietas,
      lahan: data.lahan,
      luas: data.luas,
      tgl_tanam: data.tgl_tanam,
    };
    this.gradePrediction = data.gradePrediction;
  }

  static fromJSON(jsonApiData) {
    return new PrediksiPanenModel({
      id: jsonApiData.id_transaksi,
      date: jsonApiData.tanggal_panen,
      crop: jsonApiData.nama_tanaman,
      amount: jsonApiData.estimasi_tonase,
      info: jsonApiData.status_pesan,
      varietas: jsonApiData.jenis_varietas,
      lahan: jsonApiData.lokasi_lahan,
      luas: jsonApiData.luas_hektar,
      tgl_tanam: jsonApiData.tanggal_tanam,
      gradePrediction: [
        jsonApiData.prediksi_grade_a, 
        jsonApiData.prediksi_grade_b, 
        jsonApiData.prediksi_reject
      ]
    });
  }
}

// ==========================================
// 2. SIMULASI JSON API (DATABASE RESPONSE)
// ==========================================
const dummyApiJsonResponse = [
  { 
    id_transaksi: 'TRX-9921', 
    tanggal_panen: '12-03-2026', 
    nama_tanaman: 'Jagung Manis', 
    estimasi_tonase: 10, 
    status_pesan: '3 Hari lagi anda akan melakukan panen',
    jenis_varietas: 'Bonanza F1', 
    lokasi_lahan: 'Blok A - Lahan Utara', 
    luas_hektar: 2.5, 
    tanggal_tanam: '12-12-2025',
    prediksi_grade_a: 6.5, prediksi_grade_b: 2.5, prediksi_reject: 1.0
  },
  { 
    id_transaksi: 'TRX-9922', 
    tanggal_panen: '15-03-2026', 
    nama_tanaman: 'Padi hibrida', 
    estimasi_tonase: 25, 
    status_pesan: '6 Hari lagi anda akan melakukan panen',
    jenis_varietas: 'Mapan P-05', 
    lokasi_lahan: 'Blok C - Sawah Timur', 
    luas_hektar: 4.0, 
    tanggal_tanam: '05-12-2025',
    prediksi_grade_a: 18.0, prediksi_grade_b: 5.0, prediksi_reject: 2.0
  },
  { 
    id_transaksi: 'TRX-9923', 
    tanggal_panen: '20-03-2026', 
    nama_tanaman: 'Kedelai', 
    estimasi_tonase: 8, 
    status_pesan: 'Sedang dalam masa pemupukan',
    jenis_varietas: 'Grobogan', 
    lokasi_lahan: 'Blok B - Lahan Kering', 
    luas_hektar: 3.2, 
    tanggal_tanam: '01-01-2026',
    prediksi_grade_a: 5.0, prediksi_grade_b: 2.0, prediksi_reject: 1.0
  }
];

// ==========================================
// 3. REACTIVE STATE & INTEGRATION
// ==========================================
const tableData = ref([]);

onMounted(() => {
  tableData.value = dummyApiJsonResponse.map(data => PrediksiPanenModel.fromJSON(data));
});

// Konfigurasi Chart Bar
const chartData = ref({
  labels: ['Jan', 'Feb', 'Mar', 'Apr', 'Mei', 'Jun', 'Jul', 'Agst', 'Sep', 'Oct', 'Nov', 'Des'],
  datasets: [{
    label: 'Hasil Panen',
    backgroundColor: '#265139',
    borderRadius: 4,
    borderSkipped: false,
    maxBarThickness: 32, // PERUBAHAN PENTING: maxBarThickness agar batang bisa mengecil di HP
    data: [50, 62, 68, 75, 58, 68, 62, 70, 62, 50, 70, 62]
  }]
})

const chartOptions = ref({
  responsive: true,
  maintainAspectRatio: false,
  plugins: { legend: { display: false }, tooltip: { backgroundColor: '#1a402d', padding: 10, cornerRadius: 8, displayColors: false } },
  scales: {
    y: { beginAtZero: true, max: 90, ticks: { stepSize: 10, callback: (value) => value + ' ton', color: '#9ca3af', font: { size: 11, style: 'italic' } }, grid: { display: false }, border: { display: false } },
    x: { grid: { display: false }, ticks: { color: '#4b5563', font: { weight: 'bold', size: 12 } }, border: { color: '#d1d5db', width: 2 } }
  }
})

// ==========================================
// 4. MODAL & PIE CHART LOGIC
// ==========================================
const isModalOpen = ref(false)
const selectedData = ref(null)
const modalChartData = ref(null)

const pieChartOptions = ref({
  responsive: true,
  maintainAspectRatio: false,
  plugins: {
    legend: { position: 'bottom', labels: { font: { family: 'sans-serif', weight: 'bold' }, color: '#374151', padding: 15 } },
    tooltip: { callbacks: { label: function(context) { return ` ${context.label}: ${context.raw} Ton`; } } }
  }
})

const openModal = (item) => {
  selectedData.value = item;
  modalChartData.value = {
    labels: ['Grade A (Super)', 'Grade B (Standar)', 'Afkir / Reject'],
    datasets: [{
      backgroundColor: ['#16a34a', '#eab308', '#ef4444'],
      hoverBackgroundColor: ['#15803d', '#ca8a04', '#dc2626'],
      borderWidth: 2, borderColor: '#ffffff', data: item.gradePrediction
    }]
  }
  isModalOpen.value = true;
}

const closeModal = () => {
  isModalOpen.value = false;
  setTimeout(() => { selectedData.value = null; modalChartData.value = null; }, 200);
}
</script>

<style scoped>
a {
  transition: all 0.2s ease-in-out;
}

/* Sembunyikan no-scrollbar untuk sidebar */
.no-scrollbar::-webkit-scrollbar { display: none; }
.no-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }

/* Inverted Radius untuk Desktop (Hanya dirender sempurna kalau parentnya relative di layout besar) */
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