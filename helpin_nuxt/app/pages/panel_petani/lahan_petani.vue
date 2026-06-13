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
            <h1 class="text-xl md:text-2xl font-extrabold text-gray-800 leading-tight">Manajemen Lahan</h1>
            <p class="text-xs md:text-sm text-gray-500 font-medium mt-0.5">Pemetaan area, komoditas, dan status operasional</p>
          </div>
        </div>
        <div class="flex items-center gap-2 bg-green-100 px-3 py-1.5 md:px-4 md:py-2 rounded-lg border border-green-200">
          <div class="w-2 h-2 rounded-full bg-green-600 shrink-0 animate-pulse"></div>
          <span class="text-xs md:text-sm font-bold text-green-600 hidden sm:block">STATUS ONLINE</span>
        </div>
      </header>

      <div class="p-4 md:p-10 flex flex-col w-full max-w-[100vw]">
        
        <!-- Warning: belum ada inventori bibit -->
        <div v-if="!loading && noSeedWarning" class="mb-5 bg-amber-50 border border-amber-200 rounded-xl p-4 flex items-start gap-3">
          <AlertTriangleIcon class="w-5 h-5 text-amber-500 shrink-0 mt-0.5" />
          <div class="text-sm text-amber-700">
            <span class="font-bold">Belum ada bibit di inventori.</span>
            Sebelum membuat lahan, tambahkan bibit di menu 
            <NuxtLink to="/panel_petani/inventori_petani" class="underline font-bold">Inventori</NuxtLink> 
            agar bisa menentukan komoditas lahan.
          </div>
        </div>

        <section class="bg-white rounded-2xl shadow-sm border border-gray-100 p-4 md:p-6 w-full flex flex-col flex-1">
          <div class="flex flex-col sm:flex-row justify-between items-start sm:items-center mb-6 gap-4">
            <div class="flex items-center gap-2">
              <div class="w-2 h-2 rounded-full bg-[#1a402d]"></div>
              <h2 class="text-base md:text-lg font-bold text-gray-800">Daftar Lahan</h2>
            </div>
            <button @click="openModal()" class="w-full sm:w-auto px-4 py-2.5 font-bold text-white bg-[#1a402d] hover:bg-[#143222] rounded-lg transition shadow-md flex items-center justify-center gap-2">
              <PlusIcon class="w-5 h-5" /> Tambahkan Lahan
            </button>
          </div>

          <div v-if="loading" class="flex justify-center items-center py-20">
            <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-[#1a402d]"></div>
          </div>
          
          <div v-else class="overflow-x-auto -mx-4 md:mx-0 px-4 md:px-0">
            <table class="w-full text-sm text-left min-w-[800px]">
              <thead class="text-xs text-green-800 uppercase bg-[#e1f0e5] font-bold">
                <tr>
                  <th class="px-4 md:px-6 py-4 rounded-l-lg">Nama / Blok</th>
                  <th class="px-4 md:px-6 py-4">Komoditas</th>
                  <th class="px-4 md:px-6 py-4">Luas</th>
                  <th class="px-4 md:px-6 py-4">Jenis Tanah</th>
                  <th class="px-4 md:px-6 py-4">Koordinat</th>
                  <th class="px-4 md:px-6 py-4">Estimasi Panen</th>
                  <th class="px-4 md:px-6 py-4">Status</th>
                  <th class="px-4 md:px-6 py-4 rounded-r-lg text-center">Aksi</th>
                </tr>
              </thead>
              <tbody>
                <tr v-if="lahanList.length === 0">
                  <td colspan="8" class="text-center py-12 text-gray-400">
                    <div class="flex flex-col items-center gap-2">
                      <MapIcon class="w-10 h-10 text-gray-200" />
                      <p class="font-medium">Belum ada data lahan</p>
                    </div>
                  </td>
                </tr>
                <tr v-for="item in lahanList" :key="item.id" class="border-b border-gray-50 last:border-0 hover:bg-gray-50 transition-colors">
                  <td class="px-4 md:px-6 py-4 font-bold text-gray-800">
                    {{ item.name }}
                    <span class="block text-xs font-medium text-gray-400 mt-0.5">{{ item.code }}</span>
                  </td>
                  <td class="px-4 md:px-6 py-4">
                    <span v-if="item.crop_type" class="px-2.5 py-1 bg-green-100 text-green-700 rounded-full text-xs font-bold flex items-center gap-1 w-fit">
                      <SproutIcon class="w-3 h-3" />
                      {{ item.crop_type }}
                    </span>
                    <span v-else class="text-xs text-gray-400 italic">Belum dipilih</span>
                  </td>
                  <td class="px-4 md:px-6 py-4 font-bold text-gray-800">
                    {{ item.area_hectare }} <span class="text-xs text-gray-500">Ha</span>
                  </td>
                  <td class="px-4 md:px-6 py-4 font-medium text-gray-600">{{ item.soil_type }}</td>
                  <td class="px-4 md:px-6 py-4 text-xs font-mono text-gray-500 max-w-[120px] truncate">
                    {{ item.location || '-' }}
                  </td>
                  <td class="px-4 md:px-6 py-4">
                    <div v-if="predictions[item.id]?.has_prediction" class="flex flex-col gap-1">
                      <div class="flex items-center gap-1.5">
                        <span class="px-2 py-0.5 rounded-full text-[10px] font-black"
                          :class="{
                            'bg-green-100 text-green-700': predictions[item.id].quality_grade === 'A',
                            'bg-blue-100 text-blue-700': predictions[item.id].quality_grade === 'B',
                            'bg-amber-100 text-amber-700': predictions[item.id].quality_grade === 'C'
                          }">
                          Grade {{ predictions[item.id].quality_grade }}
                        </span>
                        <span class="text-xs font-bold text-gray-700">
                          {{ formatWeight(predictions[item.id].predicted_weight_kg) }}
                        </span>
                      </div>
                      <span class="text-[10px] text-gray-400 flex items-center gap-1">
                        <CalendarIcon class="w-3 h-3" />
                        {{ predictions[item.id].predicted_harvest_date }}
                      </span>
                    </div>
                    <span v-else class="text-xs text-gray-300 italic">Belum ada</span>
                  </td>
                  <td class="px-4 md:px-6 py-4">
                    <span class="px-2.5 py-1 rounded-full text-xs font-bold"
                      :class="{
                        'bg-green-100 text-green-700': item.status === 'Aktif Ditanami',
                        'bg-yellow-100 text-yellow-700': item.status === 'Persiapan',
                        'bg-gray-100 text-gray-600': item.status === 'Masa Bera'
                      }">
                      {{ item.status }}
                    </span>
                  </td>
                  <td class="px-4 md:px-6 py-4">
                    <div class="flex justify-center gap-2">
                      <button @click="openTreatmentModal(item)" class="bg-[#1a402d] hover:bg-[#143222] text-white px-2.5 py-2 rounded-md transition shadow-sm flex items-center gap-1 text-xs font-bold whitespace-nowrap" title="Tambah Informasi Treatment">
                        <FlaskConicalIcon class="w-4 h-4" /> Info
                      </button>
                      <button @click="openModal(item)" class="bg-[#eab308] hover:bg-yellow-600 text-white p-2 rounded-md transition shadow-sm" title="Edit">
                        <EditIcon class="w-4 h-4" />
                      </button>
                      <button @click="confirmDelete(item)" class="bg-[#ef4444] hover:bg-red-600 text-white p-2 rounded-md transition shadow-sm" title="Hapus">
                        <Trash2Icon class="w-4 h-4" />
                      </button>
                    </div>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </section>
      </div>
    </main>

    <!-- Form Modal -->
    <div v-if="isFormModalOpen" class="fixed inset-0 z-50 flex items-center justify-center p-2 md:p-4">
      <div class="absolute inset-0 bg-black/60 backdrop-blur-sm" @click="closeModal"></div>
      
      <div class="bg-white rounded-2xl w-full max-w-2xl shadow-2xl relative z-10 flex flex-col max-h-[95vh] animate-modal">
        <div class="flex justify-between items-center p-5 border-b border-gray-100 shrink-0">
          <div class="flex items-center gap-3">
            <div class="p-2 bg-green-100 text-green-700 rounded-lg">
              <MapIcon class="w-5 h-5" />
            </div>
            <h2 class="text-lg font-extrabold text-gray-800">
              {{ isEditMode ? 'Edit Lahan' : 'Tambah Lahan Baru' }}
            </h2>
          </div>
          <button @click="closeModal" class="p-2 text-gray-400 hover:bg-gray-100 rounded-lg transition">
            <XIcon class="w-5 h-5" />
          </button>
        </div>

        <div class="p-5 overflow-y-auto">
          <form @submit.prevent="saveData" class="space-y-5">

            <!-- Basic info -->
            <div class="grid grid-cols-1 md:grid-cols-2 gap-5">
              <div>
                <label class="block mb-1.5 text-sm font-bold text-gray-700">Nama / Blok Lahan <span class="text-red-500">*</span></label>
                <input v-model="formData.name" type="text" required
                  class="bg-gray-50 border border-gray-200 text-gray-900 text-sm rounded-lg focus:ring-[#1a402d] focus:border-[#1a402d] block w-full p-2.5 outline-none transition"
                  placeholder="Cth: Blok A - Sawah Utara" />
              </div>
              <div>
                <label class="block mb-1.5 text-sm font-bold text-gray-700">Luas Area (Hektar) <span class="text-red-500">*</span></label>
                <input v-model.number="formData.area_hectare" type="number" step="0.01" min="0.01" required
                  class="bg-gray-50 border border-gray-200 text-gray-900 text-sm rounded-lg focus:ring-[#1a402d] focus:border-[#1a402d] block w-full p-2.5 outline-none transition"
                  placeholder="Cth: 2.5" />
              </div>
            </div>

            <div>
              <label class="block mb-1.5 text-sm font-bold text-gray-700">Jenis Tanah <span class="text-red-500">*</span></label>
              <select v-model="formData.soil_type" required
                class="bg-gray-50 border border-gray-200 text-gray-900 text-sm rounded-lg focus:ring-[#1a402d] focus:border-[#1a402d] block w-full p-2.5 outline-none transition">
                <option value="" disabled>Pilih Jenis Tanah</option>
                <option value="Tanah Lempung">Tanah Lempung</option>
                <option value="Tanah Pasir">Tanah Pasir</option>
                <option value="Tanah Gambut">Tanah Gambut</option>
                <option value="Tanah Humus">Tanah Humus</option>
              </select>
            </div>

            <!-- DIVIDER: Inventori -->
            <div class="border-t border-gray-100 pt-4">
              <h3 class="text-sm font-black text-gray-700 mb-1">Pilih dari Inventori Anda</h3>
              <p class="text-xs text-gray-400 mb-4">
                Pilih 1 bibit (menentukan komoditas lahan), pupuk, dan alat yang akan dipakai.
              </p>

              <!-- Bibit — pilih 1, wajib -->
              <div class="mb-4">
                <label class="block mb-2 text-sm font-bold text-gray-700 flex items-center gap-1.5">
                  <SproutIcon class="w-4 h-4 text-green-600" />
                  Bibit (Komoditas Lahan) <span class="text-red-500">*</span>
                </label>
                <div v-if="seedInventory.length === 0" class="text-xs text-gray-400 italic p-3 bg-gray-50 rounded-lg border border-gray-200">
                  Belum ada bibit. 
                  <NuxtLink to="/panel_petani/inventori_petani" class="text-[#1a402d] font-bold underline" @click.stop>Tambah bibit di Inventori</NuxtLink>
                </div>
                <div v-else class="grid grid-cols-1 sm:grid-cols-2 gap-2">
                  <label v-for="inv in seedInventory" :key="inv.id"
                    class="flex items-center gap-3 p-3 rounded-xl border-2 cursor-pointer transition-all"
                    :class="formData.selected_seed_id === inv.id
                      ? 'border-green-500 bg-green-50'
                      : 'border-gray-200 bg-gray-50 hover:border-gray-300'">
                    <input type="radio" :value="inv.id" v-model="formData.selected_seed_id"
                      class="w-4 h-4 text-green-600 focus:ring-green-500" />
                    <div class="flex-1 min-w-0">
                      <span class="text-sm font-bold text-gray-800 block truncate">{{ inv.name }}</span>
                      <span class="text-[10px] text-gray-400">{{ parseFloat(inv.quantity).toFixed(1) }} {{ inv.unit }}</span>
                    </div>
                    <SproutIcon v-if="formData.selected_seed_id === inv.id" class="w-4 h-4 text-green-600 shrink-0" />
                  </label>
                </div>
                <!-- Komoditas yang akan tersimpan -->
                <div v-if="selectedSeedName" class="mt-2 flex items-center gap-2 text-xs text-green-700 font-bold">
                  <CheckCircleIcon class="w-4 h-4" />
                  Komoditas lahan: <span class="px-2 py-0.5 bg-green-100 rounded-full">{{ selectedSeedName }}</span>
                </div>
              </div>

              <!-- Pupuk — pilih banyak -->
              <div class="mb-4">
                <label class="block mb-2 text-sm font-bold text-gray-700 flex items-center gap-1.5">
                  <FlaskConicalIcon class="w-4 h-4 text-blue-600" />
                  Pupuk yang Digunakan
                  <span class="text-[10px] font-normal text-gray-400">(opsional, bisa pilih banyak)</span>
                </label>
                <div v-if="fertilizerInventory.length === 0" class="text-xs text-gray-400 italic p-3 bg-gray-50 rounded-lg border border-gray-200">
                  Belum ada pupuk di inventori.
                </div>
                <div v-else class="space-y-1.5 max-h-36 overflow-y-auto border border-gray-200 rounded-xl p-3">
                  <label v-for="inv in fertilizerInventory" :key="inv.id"
                    class="flex items-center gap-3 p-2 rounded-lg hover:bg-blue-50 cursor-pointer transition">
                    <input type="checkbox" :value="inv.id" v-model="formData.selected_fertilizer_ids"
                      class="w-4 h-4 text-blue-600 rounded focus:ring-blue-500" />
                    <span class="flex-1 text-sm font-medium text-gray-700 truncate">{{ inv.name }}</span>
                    <span class="text-xs text-gray-400 shrink-0">{{ parseFloat(inv.quantity).toFixed(1) }} {{ inv.unit }}</span>
                  </label>
                </div>
              </div>

              <!-- Alat Pertanian — pilih banyak -->
              <div>
                <label class="block mb-2 text-sm font-bold text-gray-700 flex items-center gap-1.5">
                  <WrenchIcon class="w-4 h-4 text-orange-500" />
                  Alat Pertanian
                  <span class="text-[10px] font-normal text-gray-400">(opsional)</span>
                </label>
                <div v-if="toolInventory.length === 0" class="text-xs text-gray-400 italic p-3 bg-gray-50 rounded-lg border border-gray-200">
                  Belum ada alat pertanian di inventori.
                </div>
                <div v-else class="space-y-1.5 max-h-32 overflow-y-auto border border-gray-200 rounded-xl p-3">
                  <label v-for="inv in toolInventory" :key="inv.id"
                    class="flex items-center gap-3 p-2 rounded-lg hover:bg-orange-50 cursor-pointer transition">
                    <input type="checkbox" :value="inv.id" v-model="formData.selected_tool_ids"
                      class="w-4 h-4 text-orange-500 rounded focus:ring-orange-400" />
                    <span class="flex-1 text-sm font-medium text-gray-700 truncate">{{ inv.name }}</span>
                    <span class="text-xs text-gray-400 shrink-0">{{ parseFloat(inv.quantity).toFixed(1) }} {{ inv.unit }}</span>
                  </label>
                </div>
              </div>
            </div>

            <!-- Map / Koordinat -->
            <div class="border-t border-gray-100 pt-4">
              <label class="block mb-1.5 text-sm font-bold text-gray-700">Lokasi Lahan)</label>
              <p class="text-xs text-gray-400 mb-3">Klik peta untuk pin lokasi, atau isi manual</p>
              
              <div class="relative w-full h-44 bg-gray-100 rounded-xl overflow-hidden border border-gray-200 mb-3">
                <div ref="mapContainer" class="w-full h-full">
                  <div v-if="!mapLoaded" class="absolute inset-0 flex flex-col items-center justify-center gap-2 text-gray-400">
                    <MapPinIcon class="w-8 h-8" />
                    <p class="text-xs font-medium">Klik untuk set lokasi</p>
                  </div>
                </div>
                <div v-if="formData.latitude && formData.longitude"
                  class="absolute top-2 right-2 bg-white/90 backdrop-blur-sm px-2 py-1 rounded-md shadow text-[10px] font-mono text-gray-700">
                  {{ formData.latitude.toFixed(5) }}, {{ formData.longitude.toFixed(5) }}
                </div>
              </div>
            <div class="grid grid-cols-2 gap-3">
              <div>
                <label class="block mb-1 text-xs font-bold text-gray-500">Latitude</label>
                <input v-model.number="formData.latitude" type="number" step="0.000001" readonly
                  class="bg-gray-200 border border-gray-300 text-gray-600 cursor-not-allowed text-xs rounded-lg focus:outline-none block w-full p-2 font-mono transition"
                  placeholder="-6.xxxx" />
              </div>
              <div>
                <label class="block mb-1 text-xs font-bold text-gray-500">Longitude</label>
                <input v-model.number="formData.longitude" type="number" step="0.000001" readonly
                  class="bg-gray-200 border border-gray-300 text-gray-600 cursor-not-allowed text-xs rounded-lg focus:outline-none block w-full p-2 font-mono transition"
                  placeholder="106.xxxx" />
              </div>
            </div>
            </div>

          </form>
        </div>

        <div class="p-5 border-t border-gray-100 flex justify-end gap-3 bg-gray-50 rounded-b-2xl shrink-0">
          <button @click="closeModal" type="button" class="px-5 py-2.5 font-bold text-gray-600 hover:bg-gray-200 rounded-lg transition">Batal</button>
          <button @click="saveData" type="button" :disabled="saving"
            class="px-5 py-2.5 font-bold text-white bg-[#1a402d] hover:bg-[#143222] rounded-lg transition shadow-md flex items-center gap-2 disabled:opacity-50">
            <LoaderIcon v-if="saving" class="w-4 h-4 animate-spin" />
            {{ saving ? 'Menyimpan...' : 'Simpan Lahan' }}
          </button>
        </div>
      </div>
    </div>

    <!-- Treatment Modal (Tambah Informasi) -->
    <div v-if="isTreatmentModalOpen" class="fixed inset-0 z-50 flex items-center justify-center p-2 md:p-4">
      <div class="absolute inset-0 bg-black/60 backdrop-blur-sm" @click="closeTreatmentModal"></div>
      <div class="bg-white rounded-2xl w-full max-w-lg shadow-2xl relative z-10 flex flex-col max-h-[95vh] animate-modal">
        <div class="flex justify-between items-center p-5 border-b border-gray-100 shrink-0">
          <div class="flex items-center gap-3">
            <div class="p-2 bg-[#1a402d] text-white rounded-lg">
              <FlaskConicalIcon class="w-5 h-5" />
            </div>
            <div>
              <h2 class="text-lg font-extrabold text-gray-800">Tambah Informasi Treatment</h2>
              <p class="text-xs text-gray-400">{{ treatmentLand?.name }} — {{ treatmentLand?.crop_type || 'belum ada komoditas' }}</p>
            </div>
          </div>
          <button @click="closeTreatmentModal" class="p-2 text-gray-400 hover:bg-gray-100 rounded-lg transition">
            <XIcon class="w-5 h-5" />
          </button>
        </div>

        <div class="p-5 overflow-y-auto space-y-5">
          <!-- Estimasi saat ini -->
          <div v-if="predictions[treatmentLand?.id]?.has_prediction" class="bg-green-50 border border-green-200 rounded-xl p-3 flex items-center justify-between">
            <div>
              <p class="text-[10px] text-green-600 font-bold uppercase">Estimasi Saat Ini</p>
              <p class="text-sm font-black text-gray-800">
                {{ formatWeight(predictions[treatmentLand.id].predicted_weight_kg) }} ·
                Grade {{ predictions[treatmentLand.id].quality_grade }}
              </p>
            </div>
            <div class="text-right">
              <p class="text-[10px] text-gray-400">Perkiraan panen</p>
              <p class="text-xs font-bold text-gray-700">{{ predictions[treatmentLand.id].predicted_harvest_date }}</p>
            </div>
          </div>

          <!-- Pupuk + jumlah -->
          <div>
            <label class="block mb-2 text-sm font-bold text-gray-700 flex items-center gap-1.5">
              <FlaskConicalIcon class="w-4 h-4 text-blue-600" /> Pupuk yang Digunakan
            </label>
            <div v-if="fertilizerInventory.length === 0" class="text-xs text-gray-400 italic p-3 bg-gray-50 rounded-lg border border-gray-200">
              Belum ada pupuk di inventori.
            </div>
            <div v-else class="space-y-2 max-h-44 overflow-y-auto border border-gray-200 rounded-xl p-3">
              <div v-for="inv in fertilizerInventory" :key="inv.id"
                class="flex items-center gap-3 p-2 rounded-lg" :class="treatmentForm.fertilizers[inv.id] != null ? 'bg-blue-50' : ''">
                <input type="checkbox" :checked="treatmentForm.fertilizers[inv.id] != null"
                  @change="toggleFertilizer(inv)"
                  class="w-4 h-4 text-blue-600 rounded focus:ring-blue-500" />
                <div class="flex-1 min-w-0">
                  <span class="text-sm font-medium text-gray-700 block truncate">{{ inv.name }}</span>
                  <span class="text-[10px] text-gray-400">stok: {{ parseFloat(inv.quantity).toFixed(1) }} {{ inv.unit }}</span>
                </div>
                <input v-if="treatmentForm.fertilizers[inv.id] != null"
                  v-model.number="treatmentForm.fertilizers[inv.id]" type="number" min="0" step="0.1"
                  class="w-20 bg-white border border-gray-300 text-gray-900 text-xs rounded-lg p-1.5 outline-none focus:border-blue-500"
                  :placeholder="'Jml ' + inv.unit" />
              </div>
            </div>
          </div>

          <!-- Alat pertanian -->
          <div>
            <label class="block mb-2 text-sm font-bold text-gray-700 flex items-center gap-1.5">
              <WrenchIcon class="w-4 h-4 text-orange-500" /> Alat Pertanian yang Digunakan
            </label>
            <div v-if="toolInventory.length === 0" class="text-xs text-gray-400 italic p-3 bg-gray-50 rounded-lg border border-gray-200">
              Belum ada alat pertanian di inventori.
            </div>
            <div v-else class="space-y-1.5 max-h-32 overflow-y-auto border border-gray-200 rounded-xl p-3">
              <label v-for="inv in toolInventory" :key="inv.id"
                class="flex items-center gap-3 p-2 rounded-lg hover:bg-orange-50 cursor-pointer transition">
                <input type="checkbox" :value="inv.id" v-model="treatmentForm.tool_ids"
                  class="w-4 h-4 text-orange-500 rounded focus:ring-orange-400" />
                <span class="flex-1 text-sm font-medium text-gray-700 truncate">{{ inv.name }}</span>
                <span class="text-xs text-gray-400 shrink-0">{{ parseFloat(inv.quantity).toFixed(1) }} {{ inv.unit }}</span>
              </label>
            </div>
          </div>

          <!-- Catatan -->
          <div>
            <label class="block mb-1.5 text-sm font-bold text-gray-700">Catatan (opsional)</label>
            <textarea v-model="treatmentForm.notes" rows="2"
              class="bg-gray-50 border border-gray-200 text-gray-900 text-sm rounded-lg focus:ring-[#1a402d] focus:border-[#1a402d] block w-full p-2.5 outline-none transition"
              placeholder="Cth: pemupukan susulan tahap 1"></textarea>
          </div>
        </div>

        <div class="p-5 border-t border-gray-100 flex justify-end gap-3 bg-gray-50 rounded-b-2xl shrink-0">
          <button @click="closeTreatmentModal" type="button" class="px-5 py-2.5 font-bold text-gray-600 hover:bg-gray-200 rounded-lg transition">Batal</button>
          <button @click="saveTreatment" type="button" :disabled="savingTreatment"
            class="px-5 py-2.5 font-bold text-white bg-[#1a402d] hover:bg-[#143222] rounded-lg transition shadow-md flex items-center gap-2 disabled:opacity-50">
            <LoaderIcon v-if="savingTreatment" class="w-4 h-4 animate-spin" />
            {{ savingTreatment ? 'Menyimpan...' : 'Simpan & Prediksi' }}
          </button>
        </div>
      </div>
    </div>

    <!-- Delete Modal -->
    <div v-if="isDeleteModalOpen" class="fixed inset-0 z-50 flex items-center justify-center p-4">
      <div class="absolute inset-0 bg-black/60 backdrop-blur-sm" @click="closeDeleteModal"></div>
      <div class="bg-white rounded-2xl w-full max-w-sm shadow-2xl relative z-10 p-6 text-center animate-modal">
        <div class="w-14 h-14 bg-red-100 rounded-full flex items-center justify-center mx-auto mb-4 text-red-500">
          <AlertTriangleIcon class="w-7 h-7" />
        </div>
        <h2 class="text-lg font-extrabold text-gray-800 mb-2">Hapus Lahan?</h2>
        <p class="text-sm text-gray-500 mb-5">Hapus <strong>{{ itemToDelete?.name }}</strong>? Semua data terkait (tanaman, panen) juga akan terhapus.</p>
        <div class="flex gap-3">
          <button @click="closeDeleteModal" class="flex-1 py-2.5 font-bold text-gray-600 bg-gray-100 hover:bg-gray-200 rounded-lg transition">Batal</button>
          <button @click="executeDelete" :disabled="saving" class="flex-1 py-2.5 font-bold text-white bg-red-500 hover:bg-red-600 rounded-lg transition disabled:opacity-50">Hapus</button>
        </div>
      </div>
    </div>

  </div>
</template>

<script setup>
import { ref, reactive, computed, onMounted, nextTick } from 'vue'
import { 
  MapIcon, MapPinIcon, MenuIcon, XIcon, PlusIcon, EditIcon, 
  Trash2Icon, AlertTriangleIcon, LoaderIcon, SproutIcon, WrenchIcon,
  FlaskConicalIcon, CheckCircleIcon, CalendarIcon
} from 'lucide-vue-next'

const isSidebarOpen = ref(false)
const loading = ref(false)
const saving = ref(false)
const mapLoaded = ref(false)
const mapContainer = ref(null)
let mapInstance = null
let markerInstance = null

const { list: fetchLands, create: createLand, update: updateLand, remove: removeLand, listAvailableInventory, getPrediction, addTreatment } = useLand()

const lahanList = ref([])
const availableInventory = ref([])  // inventori petani: Pupuk, Bibit, Alat Pertanian
const predictions = reactive({})    // { [landId]: { has_prediction, predicted_weight_kg, quality_grade, predicted_harvest_date } }

// Kelompokkan inventori berdasarkan kategori
const seedInventory       = computed(() => availableInventory.value.filter(i => i.category === 'Bibit'))
const fertilizerInventory = computed(() => availableInventory.value.filter(i => i.category === 'Pupuk'))
const toolInventory       = computed(() => availableInventory.value.filter(i => i.category === 'Alat Pertanian'))

const noSeedWarning = computed(() => !loading.value && seedInventory.value.length === 0)

// Nama bibit yang dipilih = crop_type lahan
const selectedSeedName = computed(() => {
  if (!formData.selected_seed_id) return null
  const seed = seedInventory.value.find(i => i.id === formData.selected_seed_id)
  return seed?.name || null
})

onMounted(async () => {
  loading.value = true
  try {
    const [lands, inventory] = await Promise.all([
      fetchLands(),
      listAvailableInventory().catch(() => [])
    ])
    lahanList.value = lands || []
    availableInventory.value = inventory || []
    loadPredictions()
  } catch (e) {
    console.error(e)
    lahanList.value = []
  } finally {
    loading.value = false
  }
})

// Ambil estimasi panen untuk semua lahan
const loadPredictions = async () => {
  await Promise.all(
    lahanList.value.map(async (land) => {
      try {
        predictions[land.id] = await getPrediction(land.id)
      } catch {
        predictions[land.id] = { has_prediction: false }
      }
    })
  )
}

const formatWeight = (kg) => {
  const n = Number(kg) || 0
  if (n >= 1000) return (n / 1000).toFixed(1).replace('.0', '') + ' ton'
  return Math.round(n) + ' kg'
}

// Map
const initMap = async () => {
  await nextTick()
  if (!mapContainer.value || typeof window === 'undefined') return
  if (!window.L) {
    const link = document.createElement('link')
    link.rel = 'stylesheet'
    link.href = 'https://unpkg.com/leaflet@1.9.4/dist/leaflet.css'
    document.head.appendChild(link)
    await new Promise(resolve => {
      const script = document.createElement('script')
      script.src = 'https://unpkg.com/leaflet@1.9.4/dist/leaflet.js'
      script.onload = resolve
      document.head.appendChild(script)
    })
  }
  const L = window.L
  const lat = formData.latitude || -6.2088
  const lng = formData.longitude || 106.8456
  mapInstance = L.map(mapContainer.value).setView([lat, lng], 13)
  L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', { attribution: '© OpenStreetMap' }).addTo(mapInstance)
  if (formData.latitude && formData.longitude) {
    markerInstance = L.marker([lat, lng]).addTo(mapInstance)
  }
  mapInstance.on('click', (e) => {
    formData.latitude = parseFloat(e.latlng.lat.toFixed(6))
    formData.longitude = parseFloat(e.latlng.lng.toFixed(6))
    if (markerInstance) markerInstance.setLatLng([e.latlng.lat, e.latlng.lng])
    else markerInstance = L.marker([e.latlng.lat, e.latlng.lng]).addTo(mapInstance)
  })
  mapLoaded.value = true
  setTimeout(() => mapInstance.invalidateSize(), 300)
}

const destroyMap = () => {
  if (mapInstance) { mapInstance.remove(); mapInstance = null; markerInstance = null; mapLoaded.value = false }
}

// CRUD
const isFormModalOpen = ref(false)
const isEditMode = ref(false)
const editingId = ref(null)

const formData = reactive({
  name: '',
  area_hectare: null,
  soil_type: '',
  selected_seed_id: null,        // 1 bibit → crop_type
  selected_fertilizer_ids: [],   // banyak pupuk
  selected_tool_ids: [],         // banyak alat
  latitude: null,
  longitude: null
})

const openModal = async (item = null) => {
  if (item) {
    isEditMode.value = true
    editingId.value = item.id
    formData.name = item.name
    formData.area_hectare = item.area_hectare
    formData.soil_type = item.soil_type
    formData.selected_seed_id = null
    formData.selected_fertilizer_ids = []
    formData.selected_tool_ids = []
    if (item.location?.includes(',')) {
      const [lat, lng] = item.location.split(',').map(s => parseFloat(s.trim()))
      formData.latitude = lat || null
      formData.longitude = lng || null
    } else {
      formData.latitude = null; formData.longitude = null
    }
  } else {
    isEditMode.value = false
    editingId.value = null
    formData.name = ''; formData.area_hectare = null; formData.soil_type = ''
    formData.selected_seed_id = null
    formData.selected_fertilizer_ids = []
    formData.selected_tool_ids = []
    formData.latitude = null; formData.longitude = null
  }
  isFormModalOpen.value = true
  await nextTick()
  setTimeout(() => initMap(), 100)
}

const closeModal = () => { destroyMap(); isFormModalOpen.value = false }

const saveData = async () => {
  if (!formData.name || !formData.area_hectare || !formData.soil_type) {
    useToast().warning('Data belum lengkap', 'Lengkapi nama, luas, dan jenis tanah')
    return
  }

  saving.value = true
  try {
    const location = (formData.latitude && formData.longitude)
      ? `${formData.latitude}, ${formData.longitude}` : null

    // Kumpulkan semua inventory_ids yang dipilih
    const inventory_ids = [
      ...(formData.selected_seed_id ? [formData.selected_seed_id] : []),
      ...formData.selected_fertilizer_ids,
      ...formData.selected_tool_ids
    ]

    const payload = {
      name: formData.name,
      area_hectare: formData.area_hectare,
      soil_type: formData.soil_type,
      crop_type: selectedSeedName.value || undefined,  // dari bibit yang dipilih
      location,
      inventory_ids: inventory_ids.length > 0 ? inventory_ids : undefined
      // status tidak dikirim → backend default "Persiapan"
    }

    if (isEditMode.value) {
      const updated = await updateLand(editingId.value, payload)
      const idx = lahanList.value.findIndex(i => i.id === editingId.value)
      if (idx !== -1) lahanList.value[idx] = updated || { ...lahanList.value[idx], ...payload }
      if (editingId.value) refreshPrediction(editingId.value)
    } else {
      const created = await createLand(payload)
      const newLand = created || { ...payload, id: crypto.randomUUID(), status: 'Persiapan' }
      lahanList.value.unshift(newLand)
      if (newLand.id) refreshPrediction(newLand.id)
    }
    closeModal()
  } catch (e) {
    console.error(e)
    useToast().error('Gagal menyimpan lahan', e?.data?.error?.message || e?.data?.message)
  } finally {
    saving.value = false
  }
}

// Treatment (Tambah Informasi)
const isTreatmentModalOpen = ref(false)
const savingTreatment = ref(false)
const treatmentLand = ref(null)
const treatmentForm = reactive({
  fertilizers: {}, // { [inventoryId]: quantity }
  tool_ids: [],
  notes: ''
})

const refreshPrediction = async (landId) => {
  try {
    predictions[landId] = await getPrediction(landId)
  } catch {
    predictions[landId] = { has_prediction: false }
  }
}

const openTreatmentModal = (item) => {
  treatmentLand.value = item
  treatmentForm.fertilizers = {}
  treatmentForm.tool_ids = []
  treatmentForm.notes = ''
  isTreatmentModalOpen.value = true
}
const closeTreatmentModal = () => { isTreatmentModalOpen.value = false; treatmentLand.value = null }

const toggleFertilizer = (inv) => {
  if (treatmentForm.fertilizers[inv.id] != null) {
    delete treatmentForm.fertilizers[inv.id]
  } else {
    treatmentForm.fertilizers[inv.id] = 0
  }
}

const saveTreatment = async () => {
  if (!treatmentLand.value) return
  savingTreatment.value = true
  try {
    const fertilizers = Object.entries(treatmentForm.fertilizers).map(([id, qty]) => {
      const inv = fertilizerInventory.value.find(i => i.id === id)
      return { inventory_id: id, name: inv?.name || 'Pupuk', quantity: Number(qty) || 0, unit: inv?.unit || null }
    })
    const tools = treatmentForm.tool_ids.map(id => {
      const inv = toolInventory.value.find(i => i.id === id)
      return { inventory_id: id, name: inv?.name || 'Alat' }
    })

    const res = await addTreatment(treatmentLand.value.id, {
      fertilizers,
      tools,
      notes: treatmentForm.notes || null
    })

    // Perbarui estimasi dari hasil prediksi
    if (res?.prediction) {
      predictions[treatmentLand.value.id] = {
        has_prediction: true,
        predicted_weight_kg: res.prediction.predicted_weight_kg,
        quality_grade: res.prediction.quality_grade,
        predicted_harvest_date: res.prediction.predicted_harvest_date
      }
    } else {
      await refreshPrediction(treatmentLand.value.id)
    }
    closeTreatmentModal()
  } catch (e) {
    console.error(e)
    useToast().error('Gagal menyimpan treatment', 'Pastikan layanan tersedia & coba lagi')
  } finally {
    savingTreatment.value = false
  }
}

// Delete
const isDeleteModalOpen = ref(false)
const itemToDelete = ref(null)
const confirmDelete = (item) => { itemToDelete.value = item; isDeleteModalOpen.value = true }
const closeDeleteModal = () => { isDeleteModalOpen.value = false; setTimeout(() => { itemToDelete.value = null }, 200) }
const executeDelete = async () => {
  if (!itemToDelete.value) return
  saving.value = true
  try {
    await removeLand(itemToDelete.value.id)
    lahanList.value = lahanList.value.filter(i => i.id !== itemToDelete.value.id)
    closeDeleteModal()
  } catch (e) {
    useToast().error('Gagal menghapus lahan', e?.data?.message)
  } finally {
    saving.value = false
  }
}
</script>

<style scoped>
.animate-modal { animation: modalIn 0.25s cubic-bezier(0.16, 1, 0.3, 1) forwards; }
@keyframes modalIn {
  from { opacity: 0; transform: scale(0.96) translateY(10px); }
  to   { opacity: 1; transform: scale(1) translateY(0); }
}
</style>
