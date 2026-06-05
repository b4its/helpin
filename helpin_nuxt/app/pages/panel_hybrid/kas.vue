<template>
  <div class="flex h-screen w-full bg-[#f4f7f5] font-sans overflow-hidden relative">
    
    <SidebarHybrid :isOpen="isSidebarOpen" @close="isSidebarOpen = false" activeMenu="/panel_hybrid/kas" />

    <main class="flex-1 flex flex-col overflow-y-auto relative w-full no-scrollbar">
      
      <!-- HEADER -->
      <header class="bg-[#1a402d] text-white px-6 md:px-10 py-6 sticky top-0 z-20 shadow-md border-b-4 border-green-500 flex flex-col md:flex-row justify-between items-start md:items-center gap-4 shrink-0">
        <div class="flex items-center gap-4">
          <button @click="isSidebarOpen = true" class="md:hidden p-2 -ml-2 text-white hover:bg-white/10 rounded-lg transition">
            <MenuIcon class="w-6 h-6" />
          </button>
          <div class="w-12 h-12 bg-white rounded-full flex items-center justify-center text-[#1a402d] shadow-inner">
            <WalletIcon class="w-6 h-6" />
          </div>
          <div>
            <h1 class="text-xl md:text-2xl font-black tracking-tight">Buku Kas & Mutasi</h1>
            <p class="text-xs md:text-sm text-green-300 font-medium mt-0.5">Koperasi Agro Helpin Terpadu | Periode: Juni 2026</p>
          </div>
        </div>
        
        <!-- ACTION BUTTONS: Ditambahkan Tombol Catat Mutasi -->
        <div class="flex items-center gap-3 w-full md:w-auto mt-4 md:mt-0">
          <button @click="openModal" class="flex-1 md:flex-none flex justify-center items-center gap-2 bg-green-500 hover:bg-green-400 text-[#0c1a13] px-5 py-2.5 rounded-xl transition font-black shadow-[0_0_15px_rgba(34,197,94,0.3)]">
            <PlusIcon class="w-5 h-5" /> Catat Mutasi
          </button>
          <button class="flex items-center justify-center gap-2 bg-white/10 hover:bg-white/20 px-4 py-2.5 rounded-xl border border-white/20 transition text-sm font-bold">
            <DownloadIcon class="w-4 h-4" /> <span class="hidden sm:inline">Export Laporan</span>
          </button>
        </div>
      </header>

      <div class="p-4 md:p-6 flex flex-col gap-6 w-full max-w-[100vw]">
        
        <!-- KPI CARDS -->
        <section class="grid grid-cols-1 md:grid-cols-3 gap-4 md:gap-6">
          <div class="bg-white p-6 rounded-3xl border border-gray-100 shadow-sm flex flex-col relative overflow-hidden group hover:shadow-md transition">
            <div class="absolute -right-6 -top-6 w-24 h-24 bg-green-50 rounded-full opacity-50 group-hover:scale-150 transition-transform duration-500"></div>
            <div class="flex justify-between items-start mb-4">
              <div class="w-12 h-12 rounded-xl bg-green-100 text-green-600 flex items-center justify-center">
                <ArrowDownToLineIcon class="w-6 h-6" />
              </div>
              <span class="px-2 py-1 bg-green-50 text-green-600 text-[10px] font-black rounded uppercase tracking-wider">+12.5% MTM</span>
            </div>
            <p class="text-xs font-bold text-gray-400 uppercase tracking-wider mb-1">Total Pemasukan (Debit)</p>
            <h3 class="text-2xl lg:text-3xl font-black text-gray-800">{{ formatRupiah(summary.totalDebit) }}</h3>
          </div>

          <div class="bg-white p-6 rounded-3xl border border-gray-100 shadow-sm flex flex-col relative overflow-hidden group hover:shadow-md transition">
            <div class="absolute -right-6 -top-6 w-24 h-24 bg-red-50 rounded-full opacity-50 group-hover:scale-150 transition-transform duration-500"></div>
            <div class="flex justify-between items-start mb-4">
              <div class="w-12 h-12 rounded-xl bg-red-100 text-red-500 flex items-center justify-center">
                <ArrowUpFromLineIcon class="w-6 h-6" />
              </div>
              <span class="px-2 py-1 bg-red-50 text-red-500 text-[10px] font-black rounded uppercase tracking-wider">-4.2% MTM</span>
            </div>
            <p class="text-xs font-bold text-gray-400 uppercase tracking-wider mb-1">Total Pengeluaran (Kredit)</p>
            <h3 class="text-2xl lg:text-3xl font-black text-gray-800">{{ formatRupiah(summary.totalKredit) }}</h3>
          </div>

          <div class="bg-[#1a402d] p-6 rounded-3xl border border-[#143222] shadow-xl flex flex-col relative overflow-hidden group text-white">
            <div class="absolute -right-6 -top-6 w-24 h-24 bg-white/5 rounded-full opacity-50 group-hover:scale-150 transition-transform duration-500"></div>
            <div class="flex justify-between items-start mb-4">
              <div class="w-12 h-12 rounded-xl bg-[#23533b] text-green-400 flex items-center justify-center border border-white/10">
                <WalletIcon class="w-6 h-6" />
              </div>
              <span class="px-2 py-1 bg-green-500/20 text-green-300 text-[10px] font-black rounded border border-green-500/30 uppercase tracking-wider">Saldo Aktual</span>
            </div>
            <p class="text-xs font-bold text-green-400/80 uppercase tracking-wider mb-1">Total Saldo Kas</p>
            <h3 class="text-2xl lg:text-3xl font-black text-white">{{ formatRupiah(summary.saldoAkhir) }}</h3>
          </div>
        </section>

        <!-- CHARTS SECTION -->
        <section class="grid grid-cols-1 lg:grid-cols-3 gap-6">
          <div class="bg-white rounded-3xl shadow-sm border border-gray-100 p-6 lg:col-span-2">
            <div class="flex justify-between items-center mb-6">
              <div>
                <h2 class="text-lg font-black text-gray-800">Tren Arus Kas Bulanan</h2>
                <p class="text-xs font-medium text-gray-400 mt-1">Perbandingan Debit vs Kredit (Semester I - 2026)</p>
              </div>
              <select class="bg-gray-50 border border-gray-200 text-gray-700 text-xs font-bold rounded-lg px-3 py-2 outline-none cursor-pointer">
                <option>6 Bulan Terakhir</option>
                <option>Tahun Ini</option>
              </select>
            </div>
            <div class="relative w-full h-[300px]">
              <Bar :data="cashflowChartData" :options="cashflowOptions" />
            </div>
          </div>

          <div class="bg-white rounded-3xl shadow-sm border border-gray-100 p-6 flex flex-col">
            <h2 class="text-lg font-black text-gray-800 w-full text-left mb-2">Alokasi Pengeluaran</h2>
            <p class="text-xs font-medium text-gray-400 w-full text-left mb-6">Distribusi beban operasional & pembelian</p>
            
            <div class="relative flex-1 min-h-[220px] w-full flex items-center justify-center">
              <Doughnut :data="expensePieData" :options="pieOptions" />
              <div class="absolute inset-0 flex flex-col items-center justify-center pointer-events-none">
                <span class="text-xl font-black text-gray-800">{{ formatRupiah(summary.totalKredit / 1000000) }}Jt</span>
                <p class="text-[10px] text-gray-400 font-black uppercase tracking-widest mt-1">Total</p>
              </div>
            </div>
          </div>
        </section>

        <!-- MUTASI LEDGER TABLE -->
        <section class="bg-white rounded-3xl shadow-sm border border-gray-100 flex flex-col overflow-hidden">
          <div class="p-6 border-b border-gray-100 flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 bg-gray-50/50">
            <div>
              <h2 class="text-lg font-black text-gray-800 flex items-center gap-2">
                <HistoryIcon class="w-5 h-5 text-[#1a402d]" />
                Riwayat Mutasi Transaksi
              </h2>
              <p class="text-xs font-medium text-gray-500 mt-1">Pencatatan persisten arus masuk dan keluar kas koperasi.</p>
            </div>
            <div class="flex gap-2 w-full sm:w-auto">
              <div class="relative flex-1 sm:min-w-[250px]">
                <SearchIcon class="w-4 h-4 absolute left-3 top-1/2 -translate-y-1/2 text-gray-400" />
                <input 
                  v-model="searchTx" 
                  type="text" 
                  placeholder="Cari referensi / keterangan..." 
                  class="w-full pl-9 pr-4 py-2 bg-white border border-gray-200 rounded-xl text-sm focus:outline-none focus:border-green-500 transition-all"
                >
              </div>
            </div>
          </div>

          <div class="overflow-x-auto no-scrollbar">
            <table class="w-full text-sm text-left min-w-[900px]">
              <thead class="text-[11px] text-gray-500 uppercase tracking-widest border-b border-gray-200 bg-gray-50">
                <tr>
                  <th class="py-4 px-6 font-black w-32">Tanggal</th>
                  <th class="py-4 px-6 font-black">Ref / Keterangan</th>
                  <th class="py-4 px-6 font-black text-center w-32">Kategori</th>
                  <th class="py-4 px-6 font-black text-right w-40">Debit (Masuk)</th>
                  <th class="py-4 px-6 font-black text-right w-40">Kredit (Keluar)</th>
                  <th class="py-4 px-6 font-black text-right w-40">Saldo</th>
                </tr>
              </thead>
              <tbody class="divide-y divide-gray-100">
                <tr v-for="tx in filteredTransactions" :key="tx.id" class="hover:bg-green-50/20 transition-colors group">
                  <td class="py-4 px-6">
                    <span class="font-bold text-gray-800 block">{{ tx.date }}</span>
                    <span class="text-[10px] font-mono text-gray-400 mt-0.5 block">{{ tx.id }}</span>
                  </td>
                  <td class="py-4 px-6">
                    <span class="font-black text-gray-800 block">{{ tx.description }}</span>
                    <span class="text-xs font-medium text-gray-500">{{ tx.pic }}</span>
                  </td>
                  <td class="py-4 px-6 text-center">
                    <span class="px-2 py-1 bg-gray-100 text-gray-600 text-[10px] font-black rounded uppercase tracking-wider">
                      {{ tx.category }}
                    </span>
                  </td>
                  <td class="py-4 px-6 text-right font-black text-green-600">
                    {{ tx.type === 'debit' ? formatRupiah(tx.amount) : '-' }}
                  </td>
                  <td class="py-4 px-6 text-right font-black text-red-500">
                    {{ tx.type === 'kredit' ? formatRupiah(tx.amount) : '-' }}
                  </td>
                  <td class="py-4 px-6 text-right">
                    <span class="font-black text-[#1a402d]">{{ formatRupiah(tx.balance) }}</span>
                  </td>
                </tr>
                <tr v-if="filteredTransactions.length === 0">
                  <td colspan="6" class="py-8 text-center text-gray-400">
                    <ArchiveIcon class="w-12 h-12 mx-auto mb-3 opacity-20" />
                    <p class="font-bold">Tidak ada data mutasi yang ditemukan.</p>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </section>

      </div>
    </main>

    <!-- ========================================== -->
    <!-- MODAL CATAT MUTASI -->
    <!-- ========================================== -->
    <div v-if="isModalOpen" class="fixed inset-0 z-50 flex items-center justify-center p-4">
      <!-- Backdrop -->
      <div class="absolute inset-0 bg-[#0c1a13]/80 backdrop-blur-sm animate-in fade-in" @click="closeModal"></div>
      
      <!-- Modal Content -->
      <div class="bg-white rounded-3xl w-full max-w-xl shadow-2xl relative z-10 flex flex-col max-h-[90vh] animate-in slide-in-from-bottom duration-300 overflow-hidden border border-gray-100">
        <div class="flex justify-between items-center p-6 border-b border-gray-100 bg-gray-50/50 shrink-0">
          <h2 class="text-xl font-black text-gray-800 flex items-center gap-2">
            <PlusIcon class="w-6 h-6 text-[#1a402d]" /> Input Mutasi Baru
          </h2>
          <button @click="closeModal" class="p-2 text-gray-400 hover:bg-gray-200 hover:text-red-500 rounded-xl transition">
            <XIcon class="w-6 h-6" />
          </button>
        </div>

        <div class="p-6 overflow-y-auto space-y-6">
          
          <!-- Pilihan Tipe Mutasi -->
          <div>
            <label class="text-[11px] font-black text-gray-400 uppercase tracking-widest mb-3 block">Jenis Mutasi</label>
            <div class="grid grid-cols-2 gap-4">
              <button 
                @click="formMutasi.type = 'debit'" 
                :class="formMutasi.type === 'debit' ? 'border-green-500 bg-green-50 text-green-700 shadow-md' : 'border-gray-200 bg-white text-gray-400 hover:border-green-200'"
                class="border-2 rounded-2xl p-4 flex flex-col items-center justify-center gap-2 transition-all font-black"
              >
                <ArrowDownToLineIcon class="w-6 h-6" /> Debit (Kas Masuk)
              </button>
              <button 
                @click="formMutasi.type = 'kredit'" 
                :class="formMutasi.type === 'kredit' ? 'border-red-500 bg-red-50 text-red-600 shadow-md' : 'border-gray-200 bg-white text-gray-400 hover:border-red-200'"
                class="border-2 rounded-2xl p-4 flex flex-col items-center justify-center gap-2 transition-all font-black"
              >
                <ArrowUpFromLineIcon class="w-6 h-6" /> Kredit (Kas Keluar)
              </button>
            </div>
          </div>

          <!-- Input Nominal & Kategori -->
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <label class="text-[11px] font-black text-gray-400 uppercase tracking-widest mb-2 block">Nominal (Rp)</label>
              <input v-model="formMutasi.amount" type="number" placeholder="Contoh: 1500000" class="w-full bg-gray-50 border border-gray-200 text-gray-800 font-black rounded-xl px-4 py-3 focus:outline-none focus:border-green-500 focus:ring-1 focus:ring-green-500 transition-all">
            </div>
            <div>
              <label class="text-[11px] font-black text-gray-400 uppercase tracking-widest mb-2 block">Kategori</label>
              <select v-model="formMutasi.category" class="w-full bg-gray-50 border border-gray-200 text-gray-700 font-bold rounded-xl px-4 py-3 focus:outline-none focus:border-green-500 transition-all cursor-pointer">
                <option value="" disabled>Pilih Kategori...</option>
                <option v-for="cat in availableCategories" :key="cat" :value="cat">{{ cat }}</option>
              </select>
            </div>
          </div>

          <!-- Input Deskripsi & Tanggal -->
          <div>
            <label class="text-[11px] font-black text-gray-400 uppercase tracking-widest mb-2 block">Keterangan / Referensi</label>
            <textarea v-model="formMutasi.description" rows="2" placeholder="Catatan transaksi..." class="w-full bg-gray-50 border border-gray-200 text-gray-700 font-medium rounded-xl px-4 py-3 focus:outline-none focus:border-green-500 transition-all"></textarea>
          </div>
          
          <div>
            <label class="text-[11px] font-black text-gray-400 uppercase tracking-widest mb-2 block">Tanggal Pencatatan</label>
            <input v-model="formMutasi.date" type="date" class="w-full bg-gray-50 border border-gray-200 text-gray-700 font-bold rounded-xl px-4 py-3 focus:outline-none focus:border-green-500 transition-all">
          </div>

        </div>

        <div class="p-6 border-t border-gray-100 bg-gray-50 flex justify-end gap-3 shrink-0">
          <button @click="closeModal" class="px-6 py-3 font-bold text-gray-500 bg-white border border-gray-200 hover:bg-gray-100 rounded-xl transition">Batal</button>
          <button @click="saveTransaction" class="px-6 py-3 font-bold text-white bg-[#1a402d] hover:bg-[#143222] rounded-xl transition flex items-center gap-2 shadow-lg">
            <SaveIcon class="w-4 h-4" /> Simpan Entri
          </button>
        </div>
      </div>
    </div>
    <!-- ========================================== -->

  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { 
  MenuIcon, WalletIcon, ArrowDownToLineIcon, ArrowUpFromLineIcon, 
  HistoryIcon, SearchIcon, DownloadIcon, ArchiveIcon, PlusIcon, XIcon, SaveIcon
} from 'lucide-vue-next'
import { Bar, Doughnut } from 'vue-chartjs'
import { 
  Chart as ChartJS, Title, Tooltip, Legend, BarElement, LineElement, PointElement,
  CategoryScale, LinearScale, ArcElement
} from 'chart.js'

import SidebarHybrid from '~/components/SidebarHybrid.vue'

ChartJS.register(Title, Tooltip, Legend, BarElement, LineElement, PointElement, CategoryScale, LinearScale, ArcElement)

const isSidebarOpen = ref(false)
const searchTx = ref('')

// ==========================================
// STATE & LOGIKA MODAL MUTASI
// ==========================================
const isModalOpen = ref(false)

// Format tanggal hari ini (YYYY-MM-DD)
const getTodayDate = () => {
  const today = new Date();
  return today.toISOString().split('T')[0];
}

const formMutasi = ref({
  type: 'debit',
  amount: null,
  category: '',
  description: '',
  date: getTodayDate()
})

const openModal = () => {
  isModalOpen.value = true
}

const closeModal = () => {
  isModalOpen.value = false
  setTimeout(() => {
    formMutasi.value = {
      type: 'debit',
      amount: null,
      category: '',
      description: '',
      date: getTodayDate()
    }
  }, 300)
}

// Opsi dropdown dinamis berdasarkan Debit/Kredit
const availableCategories = computed(() => {
  if (formMutasi.value.type === 'debit') {
    return ['Penjualan POS', 'Penjualan Besar', 'Modal/Suntikan', 'Pendapatan Lain']
  } else {
    return ['Pembelian Stok', 'Operasional', 'Gaji & Honor', 'Bagi Hasil', 'Pengeluaran Lain']
  }
})

// Eksekusi Simpan Data ke Array (Programmatis)
const saveTransaction = () => {
  if (!formMutasi.value.amount || !formMutasi.value.category || !formMutasi.value.description) {
    alert("Harap lengkapi Nominal, Kategori, dan Keterangan!")
    return
  }

  const numericAmount = Number(formMutasi.value.amount)
  
  // Mengambil saldo terakhir untuk diakumulasikan
  const lastBalance = transactions.value.length > 0 
    ? transactions.value[transactions.value.length - 1].balance 
    : 0

  const newBalance = formMutasi.value.type === 'debit'
    ? lastBalance + numericAmount
    : lastBalance - numericAmount

  // Format tanggal ke ID-Format ("06 Jun 2026")
  const rawDate = new Date(formMutasi.value.date)
  const formattedDate = rawDate.toLocaleDateString('id-ID', { day: '2-digit', month: 'short', year: 'numeric' })

  // Construct Data Baru
  const newTx = {
    id: `TRX-${Math.floor(Math.random() * 900) + 700}`, // Mockup ID
    date: formattedDate,
    description: formMutasi.value.description,
    pic: 'Admin Suki SUPER',
    category: formMutasi.value.category,
    type: formMutasi.value.type,
    amount: numericAmount,
    balance: newBalance
  }

  // Masukkan ke State
  transactions.value.push(newTx)
  
  // Tutup Modal
  closeModal()
}

// ==========================================
// DATA TRANSAKSI
// ==========================================
const transactions = ref([
  { id: 'TRX-601', date: '01 Jun 2026', description: 'Saldo Awal Bulan', pic: 'Sistem', category: 'Modal', type: 'debit', amount: 120000000, balance: 120000000 },
  { id: 'TRX-602', date: '02 Jun 2026', description: 'Penjualan POS Kasir #001', pic: 'Admin Suki SUPER', category: 'Penjualan', type: 'debit', amount: 659940, balance: 120659940 },
  { id: 'TRX-603', date: '03 Jun 2026', description: 'Pembelian Stok Pupuk (50 Sak)', pic: 'Divisi Gudang', category: 'Pembelian Stok', type: 'kredit', amount: 3500000, balance: 117159940 },
  { id: 'TRX-604', date: '04 Jun 2026', description: 'Pembayaran Listrik & Air Koperasi', pic: 'Keuangan', category: 'Operasional', type: 'kredit', amount: 850000, balance: 116309940 },
  { id: 'TRX-605', date: '05 Jun 2026', description: 'Penjualan Partai (Hasil Panen Padi)', pic: 'Admin Suki SUPER', category: 'Penjualan Besar', type: 'debit', amount: 15400000, balance: 131709940 },
  { id: 'TRX-606', date: '05 Jun 2026', description: 'Distribusi Bagi Hasil Petani', pic: 'Keuangan', category: 'Bagi Hasil', type: 'kredit', amount: 7209940, balance: 124500000 },
])

const sortedTransactions = computed(() => {
  return [...transactions.value].reverse()
})

const filteredTransactions = computed(() => {
  if (!searchTx.value) return sortedTransactions.value
  const lowerSearch = searchTx.value.toLowerCase()
  return sortedTransactions.value.filter(tx => 
    tx.description.toLowerCase().includes(lowerSearch) ||
    tx.id.toLowerCase().includes(lowerSearch) ||
    tx.category.toLowerCase().includes(lowerSearch)
  )
})

const summary = computed(() => {
  let totalDebit = 0
  let totalKredit = 0
  let saldoAkhir = 0

  transactions.value.forEach(tx => {
    if (tx.type === 'debit' && tx.category !== 'Modal') totalDebit += tx.amount
    if (tx.type === 'kredit') totalKredit += tx.amount
    saldoAkhir = tx.balance
  })

  return { totalDebit, totalKredit, saldoAkhir }
})

const formatRupiah = (value) => {
  return new Intl.NumberFormat('id-ID', { style: 'currency', currency: 'IDR', minimumFractionDigits: 0 }).format(value)
}

// ==========================================
// KONFIGURASI CHART.JS
// ==========================================
const cashflowChartData = ref({
  labels: ['Jan', 'Feb', 'Mar', 'Apr', 'Mei', 'Jun'],
  datasets: [
    {
      type: 'line',
      label: 'Saldo Akhir',
      data: [105000000, 108000000, 115000000, 112000000, 120000000, 124500000],
      borderColor: '#3b82f6',
      backgroundColor: '#3b82f6',
      borderWidth: 2,
      tension: 0.4,
      yAxisID: 'y1',
    },
    {
      type: 'bar',
      label: 'Pemasukan',
      backgroundColor: '#16a34a',
      borderRadius: 4,
      data: [45000000, 52000000, 68000000, 48000000, 75000000, 16059940],
      yAxisID: 'y',
    },
    {
      type: 'bar',
      label: 'Pengeluaran',
      backgroundColor: '#ef4444',
      borderRadius: 4,
      data: [42000000, 45000000, 61000000, 51000000, 67000000, 11559940],
      yAxisID: 'y',
    }
  ]
})

const cashflowOptions = {
  responsive: true,
  maintainAspectRatio: false,
  interaction: { mode: 'index', intersect: false },
  plugins: {
    legend: { position: 'top', labels: { usePointStyle: true, boxWidth: 8, font: { weight: 'bold' } } },
    tooltip: {
      callbacks: { label: (context) => formatRupiah(context.raw) }
    }
  },
  scales: {
    x: { grid: { display: false } },
    y: { 
      type: 'linear', display: true, position: 'left',
      grid: { borderDash: [5, 5], color: '#f1f5f9' },
      ticks: { callback: (value) => value / 1000000 + 'Jt', font: { weight: 'bold' } }
    },
    y1: {
      type: 'linear', display: false, position: 'right',
      grid: { drawOnChartArea: false }
    }
  }
}

const expensePieData = ref({
  labels: ['Pembelian Stok', 'Operasional', 'Gaji & Honor', 'Bagi Hasil'],
  datasets: [{
    data: [45, 15, 10, 30],
    backgroundColor: ['#1a402d', '#f59e0b', '#3b82f6', '#10b981'],
    borderWidth: 0,
    hoverOffset: 8
  }]
})

const pieOptions = {
  responsive: true,
  maintainAspectRatio: false,
  cutout: '70%',
  plugins: {
    legend: { position: 'bottom', labels: { usePointStyle: true, padding: 20, font: { weight: 'bold', size: 11 } } }
  }
}
</script>

<style scoped>
.no-scrollbar::-webkit-scrollbar { display: none; }
.no-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }

.animate-in { animation: fadeIn 0.4s cubic-bezier(0.16, 1, 0.3, 1) forwards; }
@keyframes fadeIn {
  from { opacity: 0; transform: translateY(15px); }
  to { opacity: 1; transform: translateY(0); }
}
</style>