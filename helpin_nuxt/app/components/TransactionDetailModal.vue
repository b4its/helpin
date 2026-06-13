<template>
  <div v-if="transaction" class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50 backdrop-blur-sm" @click.self="$emit('close')">
    <div class="bg-white rounded-3xl w-full max-w-lg shadow-2xl overflow-hidden animate-fade max-h-[92vh] flex flex-col">
      <div class="flex justify-between items-center p-6 border-b border-gray-100 shrink-0">
        <div>
          <h2 class="text-lg font-black text-[#19462D]">Detail Transaksi</h2>
          <p class="text-xs text-gray-400 font-mono mt-0.5">ID: {{ transaction.id }}</p>
        </div>
        <button @click="$emit('close')" class="text-gray-400 hover:text-red-500"><XIcon class="w-6 h-6" /></button>
      </div>

      <div class="p-6 overflow-y-auto flex-1">
        <div class="flex justify-between text-xs text-gray-500 font-medium mb-4">
          <span>Waktu</span>
          <span class="font-bold text-gray-700">{{ formatDateTime(transaction.created_at) }}</span>
        </div>

        <h3 class="text-[11px] font-black text-gray-400 uppercase tracking-widest mb-2 border-b border-gray-100 pb-2">Rincian Item</h3>
        <div class="space-y-2 mb-5">
          <div v-for="(it, i) in items" :key="i" class="flex justify-between items-center text-sm">
            <div class="flex-1 pr-3 min-w-0">
              <p class="font-bold text-gray-800 truncate">{{ it.name }}</p>
              <p class="text-xs text-gray-500">{{ formatNumber(it.quantity) }} x {{ formatRupiah(it.price) }}</p>
            </div>
            <p class="font-black text-gray-800 shrink-0">{{ formatRupiah(it.price * it.quantity) }}</p>
          </div>
          <p v-if="items.length === 0" class="text-sm text-gray-400 italic">Tidak ada rincian item.</p>
        </div>

        <div class="bg-gray-50 rounded-xl p-4 space-y-2 border border-gray-100">
          <div class="flex justify-between text-sm"><span class="text-gray-500 font-bold">Subtotal</span><span class="font-bold text-gray-800">{{ formatRupiah(transaction.subtotal) }}</span></div>
          <div class="flex justify-between text-sm"><span class="text-gray-500 font-bold">Pajak</span><span class="font-bold text-gray-800">{{ formatRupiah(transaction.tax) }}</span></div>
          <div class="flex justify-between items-end pt-2 mt-2 border-t border-gray-200">
            <span class="text-sm font-black text-gray-800 uppercase tracking-wide">Total</span>
            <span class="text-xl font-black text-[#19462D]">{{ formatRupiah(transaction.total) }}</span>
          </div>
          <div class="flex justify-between text-sm pt-1"><span class="text-gray-500 font-bold">Uang Dibayar</span><span class="font-bold text-gray-800">{{ formatRupiah(transaction.amount_tendered) }}</span></div>
          <div class="flex justify-between text-sm"><span class="text-gray-500 font-bold">Kembalian</span><span class="font-black text-green-600">{{ formatRupiah(transaction.change_amount) }}</span></div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue'
import { XIcon } from 'lucide-vue-next'

const props = defineProps({
  transaction: { type: Object, default: null },
  products: { type: Array, default: () => [] },
})
defineEmits(['close'])

const { formatRupiah, formatNumber, formatDateTime } = useFormat()

const nameMap = computed(() => {
  const m = {}
  for (const p of props.products) m[p.id] = p.name
  return m
})

const items = computed(() => {
  const raw = props.transaction?.items
  const arr = Array.isArray(raw) ? raw : (raw ? Object.values(raw) : [])
  return arr.map((it) => ({
    name: it.name || nameMap.value[it.product_id] || 'Produk',
    quantity: Number(it.quantity) || 0,
    price: Number(it.price) || 0,
  }))
})
</script>

<style scoped>
.animate-fade { animation: fadeIn 0.3s cubic-bezier(0.16, 1, 0.3, 1); }
@keyframes fadeIn { from { opacity: 0; transform: translateY(10px); } to { opacity: 1; transform: translateY(0); } }
</style>
