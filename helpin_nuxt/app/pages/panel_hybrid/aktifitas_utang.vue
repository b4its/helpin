<template>
  <div class="flex h-screen w-full bg-[#f4f7f5] font-sans overflow-hidden relative">
    
    <SidebarHybrid :isOpen="isSidebarOpen" @close="isSidebarOpen = false" activeMenu="/panel_hybrid/aktifitas_utang" />

    <main class="flex-1 flex flex-col overflow-y-auto relative w-full no-scrollbar">
      
      <header class="bg-[#1a402d] text-white px-6 md:px-10 py-6 sticky top-0 z-20 shadow-md border-b-4 border-orange-500 flex flex-col md:flex-row justify-between items-start md:items-center gap-4 shrink-0">
        <div class="flex items-center gap-4">
          <button @click="isSidebarOpen = true" class="md:hidden p-2 -ml-2 text-white hover:bg-white/10 rounded-lg transition">
            <MenuIcon class="w-6 h-6" />
          </button>
          <div class="w-12 h-12 bg-white rounded-full flex items-center justify-center text-[#1a402d] shadow-inner">
            <CreditCardIcon class="w-6 h-6" />
          </div>
          <div>
            <h1 class="text-xl md:text-2xl font-black tracking-tight">Manajemen Utang Anggota</h1>
            <p class="text-xs md:text-sm text-orange-300 font-medium mt-0.5">Koperasi Agro Helpin Terpadu | E-Commerce B2B</p>
          </div>
        </div>
        
        <div class="flex items-center gap-3 w-full md:w-auto mt-4 md:mt-0">
          <button @click="openModal" class="flex-1 md:flex-none flex justify-center items-center gap-2 bg-orange-500 hover:bg-orange-400 text-white px-5 py-2.5 rounded-xl transition font-black shadow-[0_0_15px_rgba(249,115,22,0.3)]">
            <PlusIcon class="w-5 h-5" /> Catat Pembayaran
          </button>

        </div>
      </header>

      <div class="p-4 md:p-6 flex flex-col gap-6 w-full max-w-[100vw]">
        
        <section class="grid grid-cols-1 md:grid-cols-3 gap-4 md:gap-6">
          <div class="bg-white p-6 rounded-3xl border border-gray-100 shadow-sm flex flex-col relative overflow-hidden group hover:shadow-md transition">
            <div class="absolute -right-6 -top-6 w-24 h-24 bg-red-50 rounded-full opacity-50 group-hover:scale-150 transition-transform duration-500"></div>
            <div class="flex justify-between items-start mb-4">
              <div class="w-12 h-12 rounded-xl bg-red-100 text-red-600 flex items-center justify-center">
                <TrendingUpIcon class="w-6 h-6" />
              </div>
              <span class="px-2 py-1 bg-red-50 text-red-600 text-[10px] font-black rounded uppercase tracking-wider">Kewajiban</span>
            </div>
            <p class="text-xs font-bold text-gray-400 uppercase tracking-wider mb-1">Total Utang (Akumulasi)</p>
            <h3 class="text-2xl lg:text-3xl font-black text-gray-800">{{ formatRupiah(summary.totalUtang) }}</h3>
          </div>

          <div class="bg-white p-6 rounded-3xl border border-gray-100 shadow-sm flex flex-col relative overflow-hidden group hover:shadow-md transition">
            <div class="absolute -right-6 -top-6 w-24 h-24 bg-green-50 rounded-full opacity-50 group-hover:scale-150 transition-transform duration-500"></div>
            <div class="flex justify-between items-start mb-4">
              <div class="w-12 h-12 rounded-xl bg-green-100 text-green-500 flex items-center justify-center">
                <TrendingDownIcon class="w-6 h-6" />
              </div>
              <span class="px-2 py-1 bg-green-50 text-green-600 text-[10px] font-black rounded uppercase tracking-wider">Diselesaikan</span>
            </div>
            <p class="text-xs font-bold text-gray-400 uppercase tracking-wider mb-1">Total Dibayar</p>
            <h3 class="text-2xl lg:text-3xl font-black text-gray-800">{{ formatRupiah(summary.totalDibayar) }}</h3>
          </div>

          <div class="bg-orange-500 p-6 rounded-3xl border border-orange-600 shadow-xl flex flex-col relative overflow-hidden group text-white">
            <div class="absolute -right-6 -top-6 w-24 h-24 bg-white/10 rounded-full opacity-50 group-hover:scale-150 transition-transform duration-500"></div>
            <div class="flex justify-between items-start mb-4">
              <div class="w-12 h-12 rounded-xl bg-orange-600 text-white flex items-center justify-center border border-white/20">
                <AlertCircleIcon class="w-6 h-6" />
              </div>
              <span class="px-2 py-1 bg-white/20 text-white text-[10px] font-black rounded border border-white/30 uppercase tracking-wider">Outstanding</span>
            </div>
            <p class="text-xs font-bold text-orange-100 uppercase tracking-wider mb-1">Sisa Utang Berjalan</p>
            <h3 class="text-2xl lg:text-3xl font-black text-white">{{ formatRupiah(summary.sisaUtang) }}</h3>
          </div>
        </section>

        <section class="bg-white rounded-3xl shadow-sm border border-gray-100 flex flex-col overflow-hidden">
          <div class="p-6 border-b border-gray-100 flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 bg-gray-50/50">
            <div>
              <h2 class="text-lg font-black text-gray-800 flex items-center gap-2">
                <HistoryIcon class="w-5 h-5 text-[#1a402d]" />
                Buku Besar Utang & Transaksi
              </h2>
              <p class="text-xs font-medium text-gray-500 mt-1">Lacak jejak siklus utang dari pengajuan hingga pelunasan (Lengkap dengan Hash).</p>
            </div>
            <div class="flex gap-2 w-full sm:w-auto">
              <div class="relative flex-1 sm:min-w-[250px]">
                <SearchIcon class="w-4 h-4 absolute left-3 top-1/2 -translate-y-1/2 text-gray-400" />
                <input 
                  v-model="searchQuery" 
                  type="text" 
                  placeholder="Cari referensi / hash..." 
                  class="w-full pl-9 pr-4 py-2 bg-white border border-gray-200 rounded-xl text-sm focus:outline-none focus:border-orange-500 transition-all"
                >
              </div>
            </div>
          </div>

          <div class="overflow-x-auto no-scrollbar">
            <table class="w-full text-sm text-left min-w-[1000px]">
              <thead class="text-[11px] text-gray-500 uppercase tracking-widest border-b border-gray-200 bg-gray-50">
                <tr>
                  <th class="py-4 px-6 font-black w-32">Tanggal</th>
                  <th class="py-4 px-6 font-black">Ref & Hash</th>
                  <th class="py-4 px-6 font-black text-center w-28">Tipe</th>
                  <th class="py-4 px-6 font-black text-right w-36">Debit (Utang)</th>
                  <th class="py-4 px-6 font-black text-right w-36">Kredit (Bayar)</th>
                  <th class="py-4 px-6 font-black text-right w-36">Saldo Berjalan</th>
                  <th class="py-4 px-6 font-black text-center w-24">Aksi</th>
                </tr>
              </thead>
              <tbody class="divide-y divide-gray-100" v-if="!loading">
                <tr v-for="record in filteredRecords" :key="record.id" class="hover:bg-orange-50/20 transition-colors group">
                  <td class="py-4 px-6">
                    <span class="font-bold text-gray-800 block">{{ record.formattedDate }}</span>
                  </td>
                  <td class="py-4 px-6">
                    <span class="font-black text-gray-800 block">{{ record.invoice_no }}</span>
                    <div class="flex items-center gap-1 mt-0.5">
                      <HashIcon class="w-3 h-3 text-gray-400" />
                      <span class="text-[10px] font-mono text-gray-500 truncate max-w-[150px]">{{ record.tx_hash }}</span>
                    </div>
                  </td>
                  <td class="py-4 px-6 text-center">
                    <span :class="record.type === 'utang' ? 'bg-red-50 text-red-600 border-red-200' : 'bg-green-50 text-green-600 border-green-200'" class="px-2 py-1 text-[10px] font-black rounded uppercase tracking-wider border">
                      {{ record.type === 'utang' ? 'HUTANG' : 'CICILAN' }}
                    </span>
                  </td>
                  <td class="py-4 px-6 text-right font-black text-red-500">
                    {{ record.type === 'utang' ? formatRupiah(record.amount) : '-' }}
                  </td>
                  <td class="py-4 px-6 text-right font-black text-green-600">
                    {{ record.type === 'bayar' ? formatRupiah(record.amount) : '-' }}
                  </td>
                  <td class="py-4 px-6 text-right">
                    <span class="font-black text-gray-800">{{ formatRupiah(record.running_balance) }}</span>
                  </td>
                  <td class="py-4 px-6 text-center">
                    <button @click="openDetail(record)" class="text-xs font-bold text-orange-600 hover:text-orange-800 bg-orange-50 hover:bg-orange-100 px-3 py-1.5 rounded-lg transition">
                      Detail
                    </button>
                  </td>
                </tr>
                <tr v-if="filteredRecords.length === 0">
                  <td colspan="7" class="py-8 text-center text-gray-400">
                    <p class="font-bold">Tidak ada data ditemukan.</p>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </section>

      </div>
    </main>

    <div v-if="isDetailOpen" class="fixed inset-0 z-50 flex justify-end">
      <div class="absolute inset-0 bg-[#0c1a13]/60 backdrop-blur-sm" @click="closeDetail"></div>
      
      <div class="bg-white w-full max-w-md h-full shadow-2xl relative z-10 flex flex-col animate-in slide-in-from-right duration-300">
        <div class="p-6 border-b border-gray-100 bg-gray-50 flex justify-between items-center shrink-0">
          <div>
            <h2 class="text-lg font-black text-gray-800">Detail Alur Utang</h2>
            <p class="text-xs font-medium text-gray-500 mt-1">Pelacakan Historis & Ledger Hash</p>
          </div>
          <button @click="closeDetail" class="p-2 text-gray-400 hover:bg-gray-200 hover:text-red-500 rounded-xl transition">
            <XIcon class="w-5 h-5" />
          </button>
        </div>

        <div class="p-6 overflow-y-auto flex-1 bg-white" v-if="timelineData">
          
          <div class="bg-gray-50 p-4 rounded-2xl border border-gray-100 mb-8">
            <p class="text-[10px] font-black text-gray-400 uppercase tracking-widest mb-1">Referensi Induk</p>
            <h3 class="text-lg font-black text-gray-800">{{ timelineData.mainDebt.invoice_no }}</h3>
            <div class="mt-3 flex justify-between items-end">
              <div>
                <p class="text-xs font-medium text-gray-500">Status Pembayaran</p>
                <span :class="timelineData.isLunas ? 'text-green-600 bg-green-100' : 'text-orange-600 bg-orange-100'" class="inline-block mt-1 px-2 py-0.5 text-[10px] font-black rounded uppercase tracking-wider">
                  {{ timelineData.isLunas ? 'LUNAS SEPENUHNYA' : 'BELUM LUNAS' }}
                </span>
              </div>
              <div class="text-right">
                <p class="text-xs font-medium text-gray-500">Sisa Tanggungan</p>
                <p class="text-lg font-black text-red-500">{{ formatRupiah(timelineData.sisaHutang) }}</p>
              </div>
            </div>
          </div>

          <h4 class="text-sm font-black text-gray-800 mb-4 flex items-center gap-2">
            <GitMergeIcon class="w-4 h-4 text-orange-500" /> Timeline Transaksi
          </h4>

          <div class="relative border-l-2 border-gray-200 ml-3 pl-6 space-y-6">
            
            <div class="relative">
              <div class="absolute -left-[29px] top-1 w-3 h-3 rounded-full bg-red-500 ring-4 ring-white"></div>
              <p class="text-[10px] font-bold text-gray-400">{{ timelineData.mainDebt.formattedDate }}</p>
              <h5 class="text-sm font-black text-gray-800 mt-0.5">Pengambilan Utang Awal</h5>
              <p class="text-xs font-medium text-gray-600 mt-1">{{ timelineData.mainDebt.description }}</p>
              <div class="mt-2 bg-red-50 border border-red-100 p-2 rounded-lg">
                <p class="text-xs font-black text-red-600 flex justify-between">
                  <span>Nominal:</span> <span>{{ formatRupiah(timelineData.mainDebt.amount) }}</span>
                </p>
                <div class="mt-1.5 flex items-start gap-1">
                  <HashIcon class="w-3 h-3 text-gray-400 mt-0.5" />
                  <p class="text-[9px] font-mono text-gray-500 break-all leading-relaxed">{{ timelineData.mainDebt.tx_hash }}</p>
                </div>
              </div>
            </div>

            <div v-for="(payment, index) in timelineData.payments" :key="payment.id" class="relative">
              <div class="absolute -left-[29px] top-1 w-3 h-3 rounded-full bg-green-500 ring-4 ring-white"></div>
              <p class="text-[10px] font-bold text-gray-400">{{ payment.formattedDate }}</p>
              <h5 class="text-sm font-black text-gray-800 mt-0.5">Pembayaran ke-{{ index + 1 }}</h5>
              <p class="text-xs font-medium text-gray-600 mt-1">{{ payment.description }}</p>
              <div class="mt-2 bg-green-50 border border-green-100 p-2 rounded-lg">
                <p class="text-xs font-black text-green-600 flex justify-between">
                  <span>Dibayar:</span> <span>{{ formatRupiah(payment.amount) }}</span>
                </p>
                <div class="mt-1.5 flex items-start gap-1">
                  <HashIcon class="w-3 h-3 text-gray-400 mt-0.5" />
                  <p class="text-[9px] font-mono text-gray-500 break-all leading-relaxed">{{ payment.tx_hash }}</p>
                </div>
              </div>
            </div>

            <div class="relative" v-if="timelineData.isLunas">
              <div class="absolute -left-[31px] top-1 w-4 h-4 rounded-full bg-blue-500 ring-4 ring-white flex items-center justify-center">
                <CheckIcon class="w-2.5 h-2.5 text-white" />
              </div>
              <h5 class="text-sm font-black text-blue-600 mt-0.5">Utang Telah Lunas</h5>
              <p class="text-xs font-medium text-gray-500 mt-1">Seluruh rantai transaksi telah ditutup.</p>
            </div>

          </div>
        </div>
      </div>
    </div>

    <div v-if="isModalOpen" class="fixed inset-0 z-50 flex items-center justify-center p-4">
      <div class="absolute inset-0 bg-[#0c1a13]/80 backdrop-blur-sm" @click="closeModal"></div>
      <div class="bg-white rounded-3xl w-full max-w-lg shadow-2xl relative z-10 flex flex-col overflow-hidden border border-gray-100">
        <div class="p-6 flex justify-between items-center border-b border-gray-100 bg-gray-50">
          <h2 class="text-xl font-black text-gray-800">Catat Pembayaran Utang</h2>
          <button @click="closeModal"><XIcon class="w-6 h-6 text-gray-400"/></button>
        </div>
        <div class="p-6 space-y-4">
          <div>
            <label class="text-[11px] font-black text-gray-400 uppercase tracking-widest mb-2 block">Pilih Induk Utang</label>
            <select v-model="formPay.related_invoice" class="w-full bg-gray-50 border border-gray-200 text-gray-800 font-black rounded-xl px-4 py-3 focus:outline-none focus:border-orange-500">
              <option value="" disabled>Pilih referensi utang...</option>
              <option v-for="debt in debtLedger.filter(r => r.type === 'utang')" :key="debt.id" :value="debt.invoice_no">
                {{ debt.invoice_no }} - {{ debt.description }}
              </option>
            </select>
          </div>
          <div>
            <label class="text-[11px] font-black text-gray-400 uppercase tracking-widest mb-2 block">Nominal Pembayaran (Rp)</label>
            <input v-model="formPay.amount" type="number" class="w-full bg-gray-50 border border-gray-200 text-gray-800 font-black rounded-xl px-4 py-3 focus:outline-none focus:border-orange-500">
          </div>
        </div>
        <div class="p-6 border-t border-gray-100 bg-gray-50 flex justify-end gap-3">
          <button @click="closeModal" class="px-6 py-3 font-bold text-gray-500 bg-white border border-gray-200 rounded-xl">Batal</button>
          <button @click="submitPayment" class="px-6 py-3 font-bold text-white bg-orange-500 rounded-xl shadow-lg">Simpan Pembayaran</button>
        </div>
      </div>
    </div>

  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { 
  MenuIcon, CreditCardIcon, TrendingDownIcon, TrendingUpIcon, AlertCircleIcon,
  HistoryIcon, SearchIcon, DownloadIcon, PlusIcon, XIcon, HashIcon, GitMergeIcon, CheckIcon
} from 'lucide-vue-next'
import SidebarHybrid from '~/components/SidebarHybrid.vue'

// ==========================================
// API COMPOSABLE
// ==========================================
const { getReport, recordExpense } = useFinance()

// ==========================================
// STATE MANAGEMENT
// ==========================================
const isSidebarOpen = ref(false)
const searchQuery = ref('')
const loading = ref(true)
const debtLedger = ref([])

// ==========================================
// LIFECYCLE & DATA PROCESSING
// ==========================================
onMounted(async () => {
  await loadData()
})

const loadData = async () => {
  loading.value = true
  try {
    const report = await getReport()
    
    if (report && report.length > 0) {
      let currentDebt = 0
      debtLedger.value = report
        .filter(item => {
          // Filter for debt-related records (pengeluaran as utang, pemasukan as bayar)
          return item.category?.toLowerCase().includes('utang') || 
                 item.category?.toLowerCase().includes('cicilan') ||
                 item.category?.toLowerCase().includes('pinjaman') ||
                 item.description?.toLowerCase().includes('utang') ||
                 item.description?.toLowerCase().includes('cicilan')
        })
        .map(item => {
          const isDebt = (item.record_type || item.type) === 'pengeluaran'
          const type = isDebt ? 'utang' : 'bayar'
          
          if (type === 'utang') currentDebt += item.amount
          if (type === 'bayar') currentDebt -= item.amount
          
          const dateObj = new Date(item.recorded_at || item.date || Date.now())
          return {
            id: item.id || Date.now(),
            invoice_no: item.reference_id ? `INV-${item.reference_id.substring(0, 6).toUpperCase()}` : `TRX-${(item.id || '').substring(0, 6).toUpperCase()}`,
            related_invoice: item.reference_id ? `INV-${item.reference_id.substring(0, 6).toUpperCase()}` : null,
            date: item.recorded_at || item.date,
            description: item.description || '',
            type,
            amount: item.amount || 0,
            tx_hash: item.tx_hash || item.id || '0x' + Math.random().toString(16).substring(2, 14),
            formattedDate: dateObj.toLocaleDateString('id-ID', { day: '2-digit', month: 'short', year: 'numeric' }),
            running_balance: currentDebt
          }
        })

      // If no debt-specific records, show all financial records as a ledger
      if (debtLedger.value.length === 0) {
        currentDebt = 0
        debtLedger.value = report.map(item => {
          const isDebt = (item.record_type || item.type) === 'pengeluaran'
          const type = isDebt ? 'utang' : 'bayar'
          
          if (type === 'utang') currentDebt += item.amount
          if (type === 'bayar') currentDebt -= item.amount
          
          const dateObj = new Date(item.recorded_at || item.date || Date.now())
          return {
            id: item.id || Date.now(),
            invoice_no: item.reference_id ? `INV-${item.reference_id.substring(0, 6).toUpperCase()}` : `TRX-${(item.id || '').substring(0, 6).toUpperCase()}`,
            related_invoice: null,
            date: item.recorded_at || item.date,
            description: item.description || '',
            type,
            amount: item.amount || 0,
            tx_hash: item.tx_hash || item.id || '0x' + Math.random().toString(16).substring(2, 14),
            formattedDate: dateObj.toLocaleDateString('id-ID', { day: '2-digit', month: 'short', year: 'numeric' }),
            running_balance: Math.abs(currentDebt)
          }
        })
      }
    }
  } catch (e) {
    console.error('Failed to load debt data:', e)
    debtLedger.value = []
  } finally {
    loading.value = false
  }
}

const summary = computed(() => {
  let totalUtang = 0
  let totalDibayar = 0
  debtLedger.value.forEach(record => {
    if (record.type === 'utang') totalUtang += record.amount
    if (record.type === 'bayar') totalDibayar += record.amount
  })
  return { totalUtang, totalDibayar, sisaUtang: totalUtang - totalDibayar }
})

const filteredRecords = computed(() => {
  const reversed = [...debtLedger.value].reverse()
  if (!searchQuery.value) return reversed
  const q = searchQuery.value.toLowerCase()
  return reversed.filter(item => 
    item.invoice_no.toLowerCase().includes(q) || 
    (item.tx_hash && item.tx_hash.toLowerCase().includes(q))
  )
})

const formatRupiah = (value) => {
  return new Intl.NumberFormat('id-ID', { style: 'currency', currency: 'IDR', minimumFractionDigits: 0 }).format(value || 0)
}

// ==========================================
// LOGIKA SLIDE-OVER DETAIL ALUR UTANG
// ==========================================
const isDetailOpen = ref(false)
const selectedRecord = ref(null)

const openDetail = (record) => {
  selectedRecord.value = record
  isDetailOpen.value = true
}

const closeDetail = () => {
  isDetailOpen.value = false
  setTimeout(() => selectedRecord.value = null, 300)
}

// Mengkomputasi "Hulu ke Hilir" berdasarkan record yang dipilih
const timelineData = computed(() => {
  if (!selectedRecord.value) return null

  // Cari Induk Utang (Hulu)
  const targetInvoice = selectedRecord.value.type === 'utang' 
    ? selectedRecord.value.invoice_no 
    : selectedRecord.value.related_invoice

  const mainDebt = debtLedger.value.find(r => r.invoice_no === targetInvoice && r.type === 'utang')
  if (!mainDebt) {
    // If no parent debt found, show the selected record as the main debt
    return {
      mainDebt: selectedRecord.value,
      payments: [],
      sisaHutang: selectedRecord.value.amount,
      isLunas: false
    }
  }

  // Cari semua cicilan yang menginduk ke utang ini (Hilir)
  const payments = debtLedger.value.filter(r => r.related_invoice === targetInvoice && r.type === 'bayar')

  const totalDibayar = payments.reduce((sum, p) => sum + p.amount, 0)
  const sisaHutang = mainDebt.amount - totalDibayar
  const isLunas = sisaHutang <= 0

  return {
    mainDebt,
    payments,
    sisaHutang,
    isLunas
  }
})

// ==========================================
// MODAL & FORM PEMBAYARAN
// ==========================================
const isModalOpen = ref(false)
const formPay = ref({ amount: null, related_invoice: '', date: new Date().toISOString().split('T')[0] })

const openModal = () => isModalOpen.value = true
const closeModal = () => isModalOpen.value = false

const submitPayment = async () => {
  if (!formPay.value.amount) return
  
  loading.value = true
  try {
    const payload = {
      record_type: 'pemasukan',
      amount: Number(formPay.value.amount),
      category: 'Cicilan Utang',
      description: `Pembayaran untuk ${formPay.value.related_invoice || 'Utang'}`
    }
    await recordExpense(payload)
    
    const newRecord = {
      id: Date.now().toString(),
      invoice_no: `PAY-${Math.floor(Math.random() * 1000)}`,
      related_invoice: formPay.value.related_invoice,
      date: formPay.value.date,
      description: payload.description,
      type: 'bayar',
      amount: Number(formPay.value.amount),
      tx_hash: '0x' + Math.random().toString(16).substring(2, 14),
      formattedDate: new Date(formPay.value.date).toLocaleDateString('id-ID', { day: '2-digit', month: 'short', year: 'numeric' }),
      running_balance: summary.value.sisaUtang - Number(formPay.value.amount)
    }
    debtLedger.value.push(newRecord)
    closeModal()
    formPay.value = { amount: null, related_invoice: '', date: new Date().toISOString().split('T')[0] }
  } catch (e) {
    console.error('Failed to record payment:', e)
    alert('Gagal mencatat pembayaran. Silakan coba lagi.')
  } finally {
    loading.value = false
  }
}
</script>

<style scoped>
.no-scrollbar::-webkit-scrollbar { display: none; }
.no-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }
.animate-in { animation: slideInRight 0.3s cubic-bezier(0.16, 1, 0.3, 1) forwards; }
@keyframes slideInRight {
  from { opacity: 0; transform: translateX(100%); }
  to { opacity: 1; transform: translateX(0); }
}
</style>