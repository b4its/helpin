<template>
  <div class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/50 backdrop-blur-sm" @click="$emit('close')">
    <div class="bg-white rounded-2xl w-full max-w-2xl p-6 shadow-xl" @click.stop>
      
      <div class="flex justify-between items-center mb-6">
        <h2 class="text-xl font-bold text-[#19462D]">Akun Supplier</h2>
        <button @click="$emit('close')" class="text-gray-500 hover:text-red-500">
          <svg class="w-6 h-6" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24"><path d="M6 18L18 6M6 6l12 12"></path></svg>
        </button>
      </div>

      <div class="bg-gray-50 p-6 rounded-xl border border-gray-100">
        <h4 class="text-sm font-bold text-[#19462D] mb-4">TAMBAH SUPPLIER BARU</h4>
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
           </div>
        <button @click="$emit('close')" class="mt-6 w-full bg-[#19462D] text-white font-bold py-3 rounded-lg hover:bg-green-900 transition">
          SIMPAN AKUN
        </button>
      </div>

    </div>
  </div>
</template>

<script setup>
defineEmits(['close'])
</script>