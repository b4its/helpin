<template>
  <div class="min-h-screen bg-kopSurface flex flex-col font-sans">
    <header class="fixed w-full top-0 z-40 bg-white/80 backdrop-blur-md border-b border-kopLight/50 transition-all duration-300">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 h-16 flex items-center justify-between">
        <NuxtLink to="../">
            <div class="flex items-center cursor-pointer">
                <img src="/assets/helpin_light_logo.png" alt="Logo Helpin Services" class="h-10 w-auto" />
            </div>
        </NuxtLink>
        <nav class="hidden md:flex gap-8 font-medium text-sm text-gray-600">
          <a href="beranda" class="hover:text-kopPrimary transition-colors">Beranda</a>
          <a href="produk" class="hover:text-kopPrimary transition-colors">Produk</a>
          <a href="pesanan" class="text-kopPrimary font-semibold transition-colors">Pesanan</a>
        </nav>

        <div class="flex items-center gap-4">
          <button @click="openCart" class="p-2 text-gray-600 hover:text-kopPrimary transition-colors relative group">
            <Icon name="lucide:shopping-cart" class="text-xl group-hover:scale-110 transition-transform" />
            <span v-if="cartItems.length > 0" class="absolute top-0 right-0 bg-orange-500 text-white text-[10px] font-bold w-4 h-4 rounded-full flex items-center justify-center animate-pulse-once">
              {{ cartTotalItems }}
            </span>
          </button>
          
          <div class="hidden sm:flex items-center gap-2 pl-4 border-l border-gray-200">
            <div class="w-8 h-8 rounded-full bg-kopPrimary text-white flex items-center justify-center font-bold text-xs">
              AD
            </div>
            <span class="text-sm font-medium text-kopDark">Ahmad D.</span>
          </div>
        </div>
      </div>
    </header>

    <main class="flex-grow pt-24 pb-16">
      <div class="max-w-5xl mx-auto px-4 sm:px-6 lg:px-8">
        
        <div class="mb-8" v-motion-fade-visible-once>
          <h1 class="text-3xl font-bold text-kopDark tracking-tight">Daftar Pesanan Saya</h1>
          <p class="text-gray-500 mt-2">Pantau status transaksi dan pengiriman hasil bumi Anda di sini.</p>
        </div>

        <div class="bg-white rounded-t-2xl border-b border-gray-100 overflow-x-auto no-scrollbar shadow-sm">
          <div class="flex px-2 min-w-max">
            <button 
              v-for="tab in statusTabs" :key="tab.value"
              @click="activeTab = tab.value"
              :class="[
                'px-6 py-4 text-sm font-medium transition-all relative whitespace-nowrap',
                activeTab === tab.value ? 'text-kopPrimary border-b-2 border-kopPrimary' : 'text-gray-500 hover:text-gray-800 hover:bg-gray-50'
              ]"
            >
              {{ tab.label }}
            </button>
          </div>
        </div>

        <div class="space-y-6 mt-6">
          <div v-if="filteredOrders.length === 0" class="bg-white rounded-2xl border border-gray-100 p-16 text-center flex flex-col items-center shadow-sm" v-motion-fade>
            <Icon name="lucide:receipt" class="text-6xl text-gray-300 mb-4" />
            <h3 class="text-lg font-bold text-kopDark">Tidak ada pesanan</h3>
            <p class="text-gray-500 mt-2">Anda belum memiliki pesanan dengan status ini.</p>
            <a href="/produk" class="mt-6 bg-kopLight text-kopPrimary font-semibold px-6 py-2.5 rounded-full hover:bg-kopPrimary hover:text-white transition-colors">
              Mulai Belanja
            </a>
          </div>

          <div 
            v-else
            v-for="(order, index) in filteredOrders" :key="order.id"
            v-motion :initial="{ opacity: 0, y: 20 }" :enter="{ opacity: 1, y: 0, transition: { duration: 400, delay: index * 50 } }"
            class="bg-white rounded-2xl shadow-sm border border-gray-100 overflow-hidden hover:shadow-soft transition-shadow"
          >
            <div class="px-6 py-4 border-b border-gray-50 flex flex-col sm:flex-row sm:items-center justify-between gap-4 bg-gray-50/50">
              <div class="flex items-center gap-4">
                <Icon name="lucide:shopping-bag" class="text-kopPrimary text-xl" />
                <div>
                  <p class="text-xs text-gray-500 mb-0.5">Belanja • {{ formatDate(order.date) }}</p>
                  <p class="text-sm font-bold text-kopDark">{{ order.invoice }}</p>
                </div>
              </div>
              <div class="flex items-center gap-3">
                <span :class="['px-3 py-1 rounded-full text-xs font-bold uppercase tracking-wider', getStatusColor(order.status)]">
                  {{ formatStatusText(order.status) }}
                </span>
              </div>
            </div>

            <div class="p-6 cursor-pointer group" @click="openOrderDetail(order)">
              <div class="flex gap-4">
                <div class="w-20 h-20 rounded-xl bg-kopSurface flex-shrink-0 border border-gray-100 overflow-hidden">
                  <img :src="order.items[0].product.image" class="w-full h-full object-cover mix-blend-multiply group-hover:scale-105 transition-transform" />
                </div>
                <div class="flex-grow">
                  <h4 class="font-semibold text-kopDark group-hover:text-kopPrimary transition-colors">{{ order.items[0].product.name }}</h4>
                  <p class="text-sm text-gray-500 mt-1">{{ order.items[0].quantity }} x {{ formatRupiah(order.items[0].product.price) }}</p>
                  
                  <p v-if="order.items.length > 1" class="text-xs text-kopPrimary font-medium mt-2 bg-kopLight inline-block px-2 py-1 rounded">
                    + {{ order.items.length - 1 }} produk lainnya
                  </p>
                </div>
                
                <div class="hidden sm:flex flex-col items-end justify-center border-l border-gray-100 pl-6 min-w-[150px]">
                  <p class="text-xs text-gray-500 mb-1">Total Belanja</p>
                  <p class="text-lg font-bold text-kopDark">{{ formatRupiah(order.totalAmount) }}</p>
                </div>
              </div>
            </div>

            <div class="px-6 py-4 border-t border-gray-50 flex flex-col sm:flex-row items-center justify-between gap-4">
              <div class="sm:hidden w-full flex justify-between items-center mb-2">
                <span class="text-xs text-gray-500">Total Belanja:</span>
                <span class="font-bold text-kopDark">{{ formatRupiah(order.totalAmount) }}</span>
              </div>
              
              <button @click="openOrderDetail(order)" class="text-sm font-semibold text-kopPrimary hover:underline">
                Lihat Detail Transaksi
              </button>
              
              <div class="flex gap-2 w-full sm:w-auto">
                <button v-if="order.status === 'completed'" class="flex-1 sm:flex-none px-4 py-2 text-sm font-semibold border border-gray-200 text-gray-700 rounded-lg hover:bg-gray-50 transition-colors">
                  Beli Lagi
                </button>
                <button v-if="order.status === 'pending'" @click="openCheckoutModal(order)" class="flex-1 sm:flex-none px-6 py-2 text-sm font-semibold bg-kopPrimary text-white rounded-lg hover:bg-kopDark shadow-soft transition-colors">
                  Bayar Sekarang
                </button>
                <button v-if="order.status === 'shipping' || order.status === 'processing'" @click="openTrackingModal(order)" class="flex-1 sm:flex-none px-6 py-2 text-sm font-semibold bg-kopPrimary text-white rounded-lg hover:bg-kopDark shadow-soft transition-colors">
                  Lacak Pengiriman
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </main>

    <footer class="bg-kopDark text-white py-12 mt-auto">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 text-center md:text-left flex flex-col md:flex-row justify-between items-center gap-4">
        <div class="flex items-center gap-2">
          <Icon name="lucide:leaf" class="text-2xl text-kopLight" />
          <span class="text-xl font-bold tracking-tight">Helpin<span class="text-kopLight">Kop</span></span>
        </div>
        <p class="text-kopLight/60 text-sm">&copy; 2026 Helpin Koperasi. Tata kelola modern untuk petani dan peternak.</p>
      </div>
    </footer>

    <ClientOnly>
      <Teleport to="body">
        <Transition name="fade">
          <div v-if="isDetailModalOpen" class="fixed inset-0 z-[100] flex items-center justify-center p-4 sm:p-6 md:p-8">
            <div class="absolute inset-0 bg-kopDark/60 backdrop-blur-sm" @click="closeOrderDetail"></div>

            <div class="relative bg-white rounded-[2rem] shadow-2xl w-full max-w-6xl overflow-hidden flex flex-col z-10 max-h-[95vh]">
              <div class="px-8 py-5 border-b border-gray-100 flex items-center justify-between bg-white z-20">
                <div class="flex items-center gap-4">
                  <div>
                    <h3 class="font-bold text-kopDark text-xl">Detail Transaksi</h3>
                    <p class="text-sm text-gray-500 font-medium mt-0.5">{{ selectedOrder?.invoice }} • {{ formatDate(selectedOrder?.date || new Date()) }}</p>
                  </div>
                  <span :class="['hidden sm:inline-block px-3 py-1 rounded-full text-xs font-bold uppercase tracking-wider ml-2', getStatusColor(selectedOrder?.status || 'all')]">
                    {{ formatStatusText(selectedOrder?.status || 'all') }}
                  </span>
                </div>
                <button @click="closeOrderDetail" class="text-gray-400 hover:text-red-500 bg-gray-50 hover:bg-red-50 p-2.5 rounded-full transition-colors">
                  <Icon name="lucide:x" class="text-xl block" />
                </button>
              </div>

              <div class="p-6 md:p-8 overflow-y-auto flex-grow bg-kopSurface/30">
                <div class="flex flex-col lg:flex-row gap-8">
                  <div class="lg:w-5/12 flex-shrink-0">
                    <div class="bg-white rounded-2xl p-6 border border-gray-100 shadow-sm">
                      <h4 class="text-lg font-bold text-kopDark flex items-center gap-2 mb-5">
                        <Icon name="lucide:shopping-bag" class="text-kopPrimary" /> Pesanan Anda
                      </h4>
                      <div class="space-y-4 max-h-[500px] overflow-y-auto pr-2 no-scrollbar">
                        <div v-for="item in selectedOrder?.items" :key="item.product.id" class="bg-white rounded-xl p-3 border border-gray-100 shadow-sm flex gap-4 items-center transition-all hover:border-kopPrimary/30">
                          <div class="w-20 h-20 rounded-lg overflow-hidden bg-kopSurface flex-shrink-0 border border-gray-50">
                            <img :src="item.product.image" :alt="item.product.name" class="w-full h-full object-cover mix-blend-multiply" />
                          </div>
                          <div class="flex-grow">
                            <h5 class="font-bold text-kopDark text-sm leading-tight mb-1">{{ item.product.name }}</h5>
                            <p class="text-[11px] font-semibold text-gray-500 mb-1">{{ item.product.unit }}</p>
                            <div class="flex items-center justify-between mt-2">
                              <p class="text-xs text-kopPrimary font-bold">{{ formatRupiah(item.product.price) }} <span class="text-gray-400 font-medium">x {{ item.quantity }}</span></p>
                              <p class="text-sm font-bold text-kopPrimary">{{ formatRupiah(item.product.price * item.quantity) }}</p>
                            </div>
                          </div>
                        </div>
                      </div>
                    </div>
                  </div>

                  <div class="lg:w-7/12 flex flex-col gap-6">
                    <section class="bg-white p-6 rounded-2xl border border-gray-100 shadow-sm">
                      <div class="flex justify-between items-start mb-4">
                        <h4 class="font-bold text-kopDark text-base">Lokasi Pengiriman</h4>
                      </div>
                      <div class="flex gap-4 items-start">
                        <Icon name="lucide:map-pin" class="text-kopPrimary text-xl mt-1" />
                        <div>
                          <h5 class="font-bold text-kopDark text-sm">{{ selectedOrder?.shippingName }}</h5>
                          <p class="text-sm text-gray-500 mt-1 leading-relaxed">{{ selectedOrder?.shippingAddress }}</p>
                        </div>
                      </div>
                    </section>

                    <section class="bg-white p-6 rounded-2xl border border-gray-100 shadow-sm">
                      <h4 class="font-bold text-kopDark mb-4 text-base">Metode Pembayaran</h4>
                      <div class="bg-kopDark text-white rounded-xl p-4 flex items-center justify-between gap-3 shadow-soft">
                        <div class="flex items-center gap-3">
                          <Icon name="lucide:credit-card" class="text-2xl" />
                          <span class="font-bold text-sm">{{ selectedOrder?.paymentMethod || 'Belum dipilih' }}</span>
                        </div>
                        <Icon name="lucide:check-circle-2" class="text-kopLight text-xl" />
                      </div>
                    </section>

                    <section class="bg-white p-6 rounded-2xl border border-gray-100 shadow-sm">
                      <h4 class="font-bold text-kopDark mb-4 text-base">Total Biaya</h4>
                      <div class="bg-kopSurface/40 border border-gray-100 rounded-xl p-5">
                        <div class="space-y-3 mb-4 text-sm">
                          <div class="flex justify-between text-gray-500">
                            <span>Total Semua</span>
                            <span class="font-semibold text-kopDark">{{ formatRupiah(getSubtotal(selectedOrder)) }}</span>
                          </div>
                          <div class="flex justify-between text-gray-500">
                            <span>Biaya Pengiriman</span>
                            <span class="font-semibold text-kopDark">{{ formatRupiah(selectedOrder?.shippingFee || 0) }}</span>
                          </div>
                        </div>
                        <div class="border-t border-gray-200 pt-4 flex justify-between items-center">
                          <span class="font-bold text-kopDark text-lg">Total</span>
                          <span class="font-bold text-kopDark text-xl">{{ formatRupiah(selectedOrder?.totalAmount || 0) }}</span>
                        </div>
                      </div>
                    </section>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </Transition>
      </Teleport>
    </ClientOnly>

    <ClientOnly>
      <Teleport to="body">
        <Transition name="fade">
          <div v-if="isCheckoutModalOpen" class="fixed inset-0 z-[110] flex items-center justify-center p-4 sm:p-6 md:p-8">
            <div class="absolute inset-0 bg-kopDark/80 backdrop-blur-sm" @click="closeCheckoutModal"></div>
            
            <div class="relative bg-white rounded-3xl shadow-2xl w-full max-w-5xl overflow-hidden z-10 flex flex-col max-h-[95vh]">
              <div class="px-6 py-4 border-b border-gray-100 flex items-center justify-between z-10 bg-white">
                <div>
                  <h3 class="font-bold text-kopDark text-lg flex items-center gap-2">
                    <Icon name="lucide:credit-card" class="text-kopPrimary" /> Selesaikan Pembayaran
                  </h3>
                  <p class="text-xs text-gray-500 mt-1">Konfirmasi lokasi dan pilih metode pembayaran untuk order {{ selectedCheckoutOrder?.invoice }}</p>
                </div>
                <button @click="closeCheckoutModal" class="text-gray-400 hover:text-red-500 bg-gray-50 p-2 rounded-full transition-colors">
                  <Icon name="lucide:x" class="text-xl block" />
                </button>
              </div>

              <div class="overflow-y-auto flex-grow bg-kopSurface/20 p-6">
                <div class="flex flex-col lg:flex-row gap-6">
                  
                  <div class="lg:w-1/2 space-y-4">
                    <div class="bg-white p-5 rounded-2xl border border-gray-100 shadow-sm">
                      <h4 class="font-bold text-kopDark mb-3 text-sm flex items-center gap-2">
                        <Icon name="lucide:map-pin" class="text-kopPrimary" /> Tentukan Lokasi Pengiriman
                      </h4>
                      <p class="text-xs text-gray-500 mb-3">Geser pin atau klik di peta untuk menentukan kordinat alamat.</p>
                      
                      <div id="checkoutMapContainer" class="w-full h-64 bg-gray-200 rounded-xl relative z-0 overflow-hidden border border-gray-200">
                        <div v-if="isFetchingAddress" class="absolute inset-0 bg-white/60 backdrop-blur-sm z-[999] flex flex-col items-center justify-center text-kopPrimary">
                           <Icon name="lucide:loader-2" class="text-4xl animate-spin mb-2" />
                           <span class="font-bold text-sm">Mencari alamat...</span>
                        </div>
                      </div>

                      <div class="mt-4 bg-gray-50 p-3 rounded-lg border border-gray-200">
                        <p class="text-sm text-gray-700 font-medium">{{ checkoutAddress || 'Pilih lokasi di peta' }}</p>
                      </div>

                      <textarea 
                        v-model="checkoutAddressDetail"
                        placeholder="Detail Patokan (contoh: Pagar Hitam, Dekat Warung)"
                        class="w-full mt-3 bg-white border border-gray-300 rounded-xl p-3 text-sm focus:outline-none focus:border-kopPrimary focus:ring-1 focus:ring-kopPrimary"
                        rows="2"
                      ></textarea>
                    </div>
                  </div>

                  <div class="lg:w-1/2 flex flex-col gap-4">
                    <div class="bg-white p-5 rounded-2xl border border-gray-100 shadow-sm flex-grow">
                      <h4 class="font-bold text-kopDark mb-4 text-sm flex items-center gap-2">
                        <Icon name="lucide:wallet" class="text-kopPrimary" /> Pilih Metode Pembayaran
                      </h4>
                      
                      <div class="space-y-4">
                        <div v-for="category in paymentCategoriesList" :key="category.name">
                          <p class="text-xs font-bold text-gray-400 uppercase tracking-wider mb-2">{{ category.name }}</p>
                          <div class="grid grid-cols-2 gap-3">
                            <button 
                              v-for="method in category.methods" :key="method"
                              @click="checkoutPaymentMethod = method"
                              :class="['px-3 py-3 border rounded-xl text-xs font-semibold transition-all text-left flex justify-between items-center', checkoutPaymentMethod === method ? 'border-kopPrimary bg-kopLight/20 text-kopPrimary ring-1 ring-kopPrimary' : 'border-gray-200 text-gray-600 hover:border-kopPrimary/50']"
                            >
                              {{ method }}
                              <Icon v-if="checkoutPaymentMethod === method" name="lucide:check-circle-2" class="text-kopPrimary" />
                            </button>
                          </div>
                        </div>
                      </div>
                    </div>
                    
                    <button 
                      @click="processCheckout"
                      class="w-full bg-kopDark hover:bg-[#153B2A] text-white font-bold py-4 rounded-2xl transition-all shadow-md active:scale-[0.99] disabled:opacity-50 disabled:cursor-not-allowed"
                      :disabled="!checkoutPaymentMethod || !checkoutAddress"
                    >
                      Bayar Sekarang — {{ formatRupiah(selectedCheckoutOrder?.totalAmount || 0) }}
                    </button>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </Transition>
      </Teleport>
    </ClientOnly>

    <ClientOnly>
      <Teleport to="body">
        <Transition name="fade">
          <div v-if="isTrackingModalOpen" class="fixed inset-0 z-[120] flex items-center justify-center p-4 sm:p-6 md:p-8">
            <div class="absolute inset-0 bg-kopDark/60 backdrop-blur-sm" @click="closeTrackingModal"></div>
            
            <div class="relative bg-white rounded-3xl shadow-2xl w-full max-w-md overflow-hidden z-10 flex flex-col">
              <div class="px-6 py-5 border-b border-gray-100 flex items-center justify-between bg-kopSurface z-10">
                <div>
                  <h3 class="font-bold text-kopDark text-lg flex items-center gap-2">
                    <Icon name="lucide:truck" class="text-kopPrimary" /> Status Pengiriman
                  </h3>
                  <p class="text-xs text-gray-500 font-medium mt-1 font-mono">Resi: KOP-{{ selectedTrackingOrder?.id.split('-')[1] }}-{{ new Date().getFullYear() }}</p>
                </div>
                <button @click="closeTrackingModal" class="text-gray-400 hover:text-red-500 bg-white p-2 rounded-full transition-colors border border-gray-100">
                  <Icon name="lucide:x" class="text-xl block" />
                </button>
              </div>

              <div class="p-6 md:p-8 bg-white overflow-y-auto">
                <div class="relative pl-4 border-l-2 border-kopLight space-y-8">
                  <div class="relative">
                    <div class="absolute -left-[23px] top-0 w-6 h-6 rounded-full bg-kopPrimary text-white flex items-center justify-center shadow-[0_0_0_4px_white]">
                      <Icon name="lucide:check" class="text-xs" />
                    </div>
                    <div class="pl-4">
                      <h4 class="font-bold text-kopDark text-sm">Pesanan Dibuat</h4>
                      <p class="text-xs text-gray-500 mt-1">{{ formatDate(selectedTrackingOrder?.date || new Date()) }} 10:00</p>
                      <p class="text-sm text-gray-600 mt-2">Pesanan telah berhasil dibuat dan dibayar.</p>
                    </div>
                  </div>

                  <div class="relative">
                    <div :class="['absolute -left-[23px] top-0 w-6 h-6 rounded-full flex items-center justify-center shadow-[0_0_0_4px_white]', getTimelineStatusLevel(selectedTrackingOrder?.status) >= 2 ? 'bg-kopPrimary text-white' : 'bg-gray-200 text-gray-400']">
                      <Icon v-if="getTimelineStatusLevel(selectedTrackingOrder?.status) >= 2" name="lucide:check" class="text-xs" />
                      <div v-else class="w-2 h-2 rounded-full bg-gray-400"></div>
                    </div>
                    <div class="pl-4">
                      <h4 :class="['font-bold text-sm', getTimelineStatusLevel(selectedTrackingOrder?.status) >= 2 ? 'text-kopDark' : 'text-gray-400']">Pesanan Diproses</h4>
                      <p v-if="getTimelineStatusLevel(selectedTrackingOrder?.status) >= 2" class="text-xs text-gray-500 mt-1">{{ formatDate(selectedTrackingOrder?.date || new Date()) }} 14:30</p>
                      <p :class="['text-sm mt-2', getTimelineStatusLevel(selectedTrackingOrder?.status) >= 2 ? 'text-gray-600' : 'text-gray-400']">Pesanan sedang dikemas oleh pihak Koperasi.</p>
                    </div>
                  </div>

                  <div class="relative">
                    <div :class="['absolute -left-[23px] top-0 w-6 h-6 rounded-full flex items-center justify-center shadow-[0_0_0_4px_white]', getTimelineStatusLevel(selectedTrackingOrder?.status) >= 3 ? 'bg-kopPrimary text-white' : 'bg-gray-200 text-gray-400']">
                      <Icon v-if="getTimelineStatusLevel(selectedTrackingOrder?.status) >= 3" name="lucide:check" class="text-xs" />
                      <div v-else class="w-2 h-2 rounded-full bg-gray-400"></div>
                    </div>
                    <div class="pl-4">
                      <h4 :class="['font-bold text-sm', getTimelineStatusLevel(selectedTrackingOrder?.status) >= 3 ? 'text-kopDark' : 'text-gray-400']">Dalam Perjalanan</h4>
                      <p v-if="getTimelineStatusLevel(selectedTrackingOrder?.status) >= 3" class="text-xs text-gray-500 mt-1">{{ formatDate(selectedTrackingOrder?.date || new Date()) }} 16:45</p>
                      <p :class="['text-sm mt-2', getTimelineStatusLevel(selectedTrackingOrder?.status) >= 3 ? 'text-gray-600' : 'text-gray-400']">Kurir sedang dalam perjalanan menuju lokasi Anda.</p>
                    </div>
                  </div>

                  <div class="relative">
                    <div :class="['absolute -left-[23px] top-0 w-6 h-6 rounded-full flex items-center justify-center shadow-[0_0_0_4px_white]', getTimelineStatusLevel(selectedTrackingOrder?.status) >= 4 ? 'bg-kopPrimary text-white' : 'bg-gray-200 text-gray-400']">
                      <Icon v-if="getTimelineStatusLevel(selectedTrackingOrder?.status) >= 4" name="lucide:check" class="text-xs" />
                      <div v-else class="w-2 h-2 rounded-full bg-gray-400"></div>
                    </div>
                    <div class="pl-4">
                      <h4 :class="['font-bold text-sm', getTimelineStatusLevel(selectedTrackingOrder?.status) >= 4 ? 'text-kopDark' : 'text-gray-400']">Pesanan Selesai</h4>
                      <p :class="['text-sm mt-2', getTimelineStatusLevel(selectedTrackingOrder?.status) >= 4 ? 'text-gray-600' : 'text-gray-400']">Pesanan telah diterima dengan baik.</p>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </Transition>
      </Teleport>
    </ClientOnly>

    <ClientOnly>
      <Teleport to="body">
        <Transition name="fade">
          <div v-if="isCartOpen" class="fixed inset-0 z-[110] bg-kopDark/50 backdrop-blur-sm" @click="closeCart"></div>
        </Transition>
        
        <Transition name="slide-right">
          <div v-if="isCartOpen" class="fixed top-0 right-0 h-full w-full sm:w-[400px] bg-white shadow-2xl z-[120] flex flex-col">
            <div class="px-6 py-5 border-b border-gray-100 flex items-center justify-between bg-kopSurface">
              <h2 class="text-xl font-bold text-kopDark flex items-center gap-2">
                <Icon name="lucide:shopping-bag" class="text-kopPrimary" /> Keranjang Saya
              </h2>
              <button @click="closeCart" class="text-gray-400 hover:text-red-500 hover:bg-red-50 p-2 rounded-full transition-colors">
                <Icon name="lucide:x" class="text-xl block" />
              </button>
            </div>

            <div class="flex-grow overflow-y-auto p-6">
              <div v-if="cartItems.length === 0" class="h-full flex flex-col items-center justify-center text-center opacity-60">
                <Icon name="lucide:shopping-cart" class="text-6xl text-gray-300 mb-4" />
                <p class="text-gray-500 font-medium">Keranjang masih kosong.</p>
                <p class="text-sm text-gray-400">Mari mulai belanja hasil bumi terbaik.</p>
              </div>

              <div v-else class="space-y-6">
                <div v-for="item in cartItems" :key="item.product.id" class="flex gap-4 group">
                  <div class="w-20 h-20 rounded-xl bg-kopSurface flex-shrink-0 overflow-hidden border border-gray-100">
                    <img :src="item.product.image" class="w-full h-full object-cover mix-blend-multiply" />
                  </div>
                  <div class="flex-grow flex flex-col justify-between">
                    <div class="flex justify-between items-start">
                      <div>
                        <h4 class="font-semibold text-kopDark text-sm leading-tight">{{ item.product.name }}</h4>
                        <p class="text-xs text-gray-400 mt-0.5">{{ formatRupiah(item.product.price) }} / {{ item.product.unit }}</p>
                      </div>
                      <button @click="removeFromCart(item.product.id)" class="text-gray-300 hover:text-red-500 transition-colors px-1">
                        <Icon name="lucide:trash-2" class="text-lg" />
                      </button>
                    </div>
                    <div class="flex items-center justify-between mt-2">
                      <div class="flex items-center bg-gray-50 rounded-lg border border-gray-100">
                        <button @click="updateCartQty(item.product.id, -1)" class="w-7 h-7 flex items-center justify-center text-gray-500 hover:text-kopPrimary disabled:opacity-30" :disabled="item.quantity <= 1">-</button>
                        <span class="text-xs font-semibold w-6 text-center">{{ item.quantity }}</span>
                        <button @click="updateCartQty(item.product.id, 1)" class="w-7 h-7 flex items-center justify-center text-gray-500 hover:text-kopPrimary">+</button>
                      </div>
                      <span class="font-bold text-kopPrimary text-sm">{{ formatRupiah(item.product.price * item.quantity) }}</span>
                    </div>
                  </div>
                </div>
              </div>
            </div>

            <div v-if="cartItems.length > 0" class="border-t border-gray-100 p-6 bg-white shadow-[0_-10px_20px_-10px_rgba(0,0,0,0.05)]">
              <div class="flex items-center justify-between mb-4 text-sm">
                <span class="text-gray-500">Total Belanja ({{ cartTotalItems }} barang)</span>
                <span class="font-bold text-xl text-kopDark">{{ formatRupiah(cartTotalPrice) }}</span>
              </div>
              <a href="/checkout" class="block w-full bg-kopDark hover:bg-kopPrimary text-white font-bold py-4 rounded-xl transition-colors shadow-lg active:scale-[0.98] text-center">
                Lanjut ke Pembayaran
              </a>
            </div>
          </div>
        </Transition>
      </Teleport>
    </ClientOnly>

  </div>
</template>

<script setup lang="ts">
import { ref, computed, nextTick, onMounted } from 'vue'
import { useHead } from '#imports'

useHead({ link: [{ rel: 'stylesheet', href: 'https://unpkg.com/leaflet@1.9.4/dist/leaflet.css' }] })

// ==========================================
// API COMPOSABLE
// ==========================================
const { listOrders: fetchOrders, getCart: fetchCart, addToCart: apiAddToCart, updateCartQty: apiUpdateCartQty, removeFromCart: apiRemoveFromCart, checkout: apiCheckout } = useProducts()

// ==========================================
// 1. ARSITEKTUR DATA (Strict TypeScript)
// ==========================================
type OrderStatus = 'all' | 'pending' | 'processing' | 'shipping' | 'completed' | 'cancelled'
interface IOrderProduct { id: number; name: string; price: number; unit: string; image: string; }
interface IOrderItem { product: IOrderProduct; quantity: number; }
interface IShippingOption { name: string; estimate: string; }

interface IOrder {
  id: string; invoice: string; date: Date; status: OrderStatus;
  items: IOrderItem[]; totalAmount: number;
  shippingName: string; shippingAddress: string; lat: number; lng: number;
  shippingOption: IShippingOption; shippingFee: number; paymentMethod: string; discount: number;
}

// ==========================================
// 2. REACTIVE STATE
// ==========================================
const activeTab = ref<OrderStatus>('all')
const loading = ref(false)
const statusTabs = [
  { label: 'Semua', value: 'all' }, { label: 'Belum Bayar', value: 'pending' },
  { label: 'Dikemas', value: 'processing' }, { label: 'Dikirim', value: 'shipping' },
  { label: 'Selesai', value: 'completed' },
]

const paymentCategoriesList = [
  { name: 'Transfer Bank', methods: ['Bank Mandiri', 'BCA', 'BRI', 'BSI'] },
  { name: 'E-Wallet', methods: ['Gopay', 'OVO', 'DANA', 'QRIS'] },
  { name: 'Kredit/Debit', methods: ['Visa / Mastercard'] }
]

const orderData = ref<IOrder[]>([])
const cartItems = ref<IOrderItem[]>([])

// Map API order to UI model
const mapOrder = (item: any): IOrder => ({
  id: item.id,
  invoice: item.invoice || `INV/${item.id?.substring(0, 8).toUpperCase()}`,
  date: new Date(item.created_at || item.date || Date.now()),
  status: mapApiStatus(item.status),
  items: (item.items || item.order_items || []).map((oi: any) => ({
    product: {
      id: oi.product_id || oi.product?.id,
      name: oi.product_name || oi.product?.name || oi.name || 'Produk',
      price: oi.price_at_purchase || oi.price || oi.product?.price || 0,
      unit: oi.unit || oi.product?.unit || '1 unit',
      image: oi.image_url || oi.product?.image || 'https://images.unsplash.com/photo-1464226184884-fa280b87c399?auto=format&fit=crop&w=300&q=80'
    },
    quantity: oi.quantity || 1
  })),
  totalAmount: item.total_amount || item.totalAmount || 0,
  shippingName: item.shipping_name || 'User',
  shippingAddress: item.shipping_address || '',
  lat: -0.4718,
  lng: 117.1536,
  shippingOption: { name: 'Kurir Koperasi', estimate: '1-2 Hari' },
  shippingFee: 15000,
  paymentMethod: item.payment_method || '',
  discount: 0
})

const mapApiStatus = (status: string): OrderStatus => {
  const map: Record<string, OrderStatus> = {
    'Menunggu Pembayaran': 'pending',
    'Diproses': 'processing',
    'Dikirim': 'shipping',
    'Selesai': 'completed',
    'Dibatalkan': 'cancelled'
  }
  return map[status] || (status as OrderStatus) || 'pending'
}

onMounted(async () => {
  loading.value = true
  try {
    const [ordersRes, cartRes] = await Promise.allSettled([
      fetchOrders(),
      fetchCart()
    ])
    
    if (ordersRes.status === 'fulfilled' && ordersRes.value) {
      orderData.value = ordersRes.value.map(mapOrder)
    }
    if (cartRes.status === 'fulfilled' && cartRes.value) {
      cartItems.value = (cartRes.value || []).map((item: any) => ({
        product: {
          id: item.product_id || item.id,
          name: item.product_name || item.name || 'Produk',
          price: item.price || 0,
          unit: item.unit || '1 unit',
          image: item.image_url || item.image || 'https://images.unsplash.com/photo-1464226184884-fa280b87c399?auto=format&fit=crop&w=300&q=80'
        },
        quantity: item.quantity || 1
      }))
    }
  } catch (e) {
    console.error('Failed to load orders:', e)
  } finally {
    loading.value = false
  }
})

// ==========================================
// 3. LOGIKA KERANJANG (CART DRAWER)
// ==========================================
const isCartOpen = ref(false)
const openCart = () => { isCartOpen.value = true; toggleBodyScroll(true) }
const closeCart = () => { isCartOpen.value = false; toggleBodyScroll(false) }

const removeFromCart = (id: number) => { cartItems.value = cartItems.value.filter(i => i.product.id !== id) }
const updateCartQty = (id: number, change: number) => {
  const item = cartItems.value.find(i => i.product.id === id)
  if (item && item.quantity + change > 0) item.quantity += change
}
const cartTotalItems = computed(() => cartItems.value.reduce((total, item) => total + item.quantity, 0))
const cartTotalPrice = computed(() => cartItems.value.reduce((total, item) => total + (item.product.price * item.quantity), 0))

// ==========================================
// 4. LOGIKA MODAL DETAIL (UMUM)
// ==========================================
const isDetailModalOpen = ref(false)
const selectedOrder = ref<IOrder | null>(null)

const toggleBodyScroll = (lock: boolean) => {
  if (typeof document !== 'undefined') document.body.style.overflow = lock ? 'hidden' : 'auto'
}

const openOrderDetail = (order: IOrder) => {
  selectedOrder.value = order
  isDetailModalOpen.value = true
  toggleBodyScroll(true)
}
const closeOrderDetail = () => {
  isDetailModalOpen.value = false
  toggleBodyScroll(false)
  setTimeout(() => { selectedOrder.value = null }, 300)
}
const getSubtotal = (order: IOrder | null) => {
  if (!order) return 0
  return order.items.reduce((total, item) => total + (item.product.price * item.quantity), 0)
}

// ==========================================
// 5. LOGIKA MODAL BAYAR SEKARANG (CHECKOUT)
// ==========================================
const isCheckoutModalOpen = ref(false)
const selectedCheckoutOrder = ref<IOrder | null>(null)
let checkoutMapInstance: any = null
let checkoutMarkerInstance: any = null

const checkoutPaymentMethod = ref<string>('')
const checkoutAddress = ref<string>('')
const checkoutAddressDetail = ref<string>('')
const isFetchingAddress = ref(false)

const openCheckoutModal = async (order: IOrder) => {
  selectedCheckoutOrder.value = order
  checkoutAddress.value = order.shippingAddress
  checkoutPaymentMethod.value = ''
  checkoutAddressDetail.value = ''
  isCheckoutModalOpen.value = true
  toggleBodyScroll(true)

  await nextTick()
  if (typeof window !== 'undefined') {
    try {
      const L = (await import('leaflet')).default
      if (checkoutMapInstance) checkoutMapInstance.remove()
      
      const { lat, lng } = order
      checkoutMapInstance = L.map('checkoutMapContainer').setView([lat, lng], 15)

      L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
        attribution: '&copy; OpenStreetMap'
      }).addTo(checkoutMapInstance)

      const customIcon = L.icon({
        iconUrl: 'https://unpkg.com/leaflet@1.9.4/dist/images/marker-icon.png',
        shadowUrl: 'https://unpkg.com/leaflet@1.9.4/dist/images/marker-shadow.png',
        iconSize: [25, 41], iconAnchor: [12, 41]
      })

      checkoutMarkerInstance = L.marker([lat, lng], { icon: customIcon, draggable: true }).addTo(checkoutMapInstance)

      const fetchReverseGeo = async (lat: number, lng: number) => {
        isFetchingAddress.value = true
        try {
          const res = await fetch(`https://nominatim.openstreetmap.org/reverse?format=json&lat=${lat}&lon=${lng}&zoom=18&addressdetails=1`)
          const data = await res.json()
          if (data && data.display_name) checkoutAddress.value = data.display_name
        } catch (e) { checkoutAddress.value = 'Gagal memuat alamat.' }
        finally { isFetchingAddress.value = false }
      }

      checkoutMapInstance.on('click', async (e: any) => {
        const { lat, lng } = e.latlng
        checkoutMarkerInstance.setLatLng([lat, lng])
        await fetchReverseGeo(lat, lng)
      })

      checkoutMarkerInstance.on('dragend', async () => {
        const pos = checkoutMarkerInstance.getLatLng()
        await fetchReverseGeo(pos.lat, pos.lng)
      })

      setTimeout(() => checkoutMapInstance.invalidateSize(), 300)
    } catch (e) { console.error(e) }
  }
}

const closeCheckoutModal = () => {
  isCheckoutModalOpen.value = false
  toggleBodyScroll(false)
  if (checkoutMapInstance) {
    checkoutMapInstance.remove()
    checkoutMapInstance = null
    checkoutMarkerInstance = null
  }
}

const processCheckout = async () => {
  if (selectedCheckoutOrder.value) {
    loading.value = true
    try {
      const fullAddress = checkoutAddressDetail.value ? `${checkoutAddress.value} (${checkoutAddressDetail.value})` : checkoutAddress.value
      await apiCheckout(fullAddress)
      
      const targetId = selectedCheckoutOrder.value.id
      const targetOrder = orderData.value.find(o => o.id === targetId)
      if (targetOrder) {
        targetOrder.status = 'processing'
        targetOrder.paymentMethod = checkoutPaymentMethod.value
        targetOrder.shippingAddress = fullAddress
      }
    } catch (e) {
      console.error('Failed to process checkout:', e)
      useToast().error('Gagal memproses pembayaran', e?.data?.error?.message || e?.data?.message)
    } finally {
      loading.value = false
    }
  }
  closeCheckoutModal()
}

// ==========================================
// 6. LOGIKA MODAL LACAK PENGIRIMAN (TRACKING)
// ==========================================
const isTrackingModalOpen = ref(false)
const selectedTrackingOrder = ref<IOrder | null>(null)

const openTrackingModal = (order: IOrder) => {
  selectedTrackingOrder.value = order
  isTrackingModalOpen.value = true
  toggleBodyScroll(true)
}

const closeTrackingModal = () => {
  isTrackingModalOpen.value = false
  toggleBodyScroll(false)
  setTimeout(() => { selectedTrackingOrder.value = null }, 300)
}

const getTimelineStatusLevel = (status?: OrderStatus) => {
  if (!status) return 0
  switch (status) {
    case 'pending': return 1
    case 'processing': return 2
    case 'shipping': return 3
    case 'completed': return 4
    default: return 0
  }
}

// ==========================================
// 7. FILTER & FORMATTERS
// ==========================================
const filteredOrders = computed(() => {
  let result = orderData.value
  if (activeTab.value !== 'all') {
    result = result.filter(order => order.status === activeTab.value)
  }
  return result.sort((a, b) => b.date.getTime() - a.date.getTime())
})

const formatRupiah = (price: number) => new Intl.NumberFormat('id-ID', { style: 'currency', currency: 'IDR', minimumFractionDigits: 0 }).format(price).replace(/\s/g, ' ')
const formatDate = (date: Date) => new Intl.DateTimeFormat('id-ID', { day: 'numeric', month: 'short', year: 'numeric' }).format(date)
const formatStatusText = (status: OrderStatus) => ({ all: 'Semua', pending: 'Belum Bayar', processing: 'Dikemas', shipping: 'Dikirim', completed: 'Selesai', cancelled: 'Dibatalkan' }[status] || status)
const getStatusColor = (status: OrderStatus) => ({ all: '', pending: 'bg-orange-100 text-orange-600 border border-orange-200', processing: 'bg-blue-100 text-blue-600 border border-blue-200', shipping: 'bg-purple-100 text-purple-600 border border-purple-200', completed: 'bg-kopLight text-kopPrimary border border-kopPrimary/30', cancelled: 'bg-red-100 text-red-600 border border-red-200' }[status] || 'bg-gray-100 text-gray-600')
</script>

<style>
.no-scrollbar::-webkit-scrollbar { display: none; }
.no-scrollbar { -ms-overflow-style: none; scrollbar-width: none; }
.fade-enter-active, .fade-leave-active { transition: opacity 0.3s ease; }
.fade-enter-from, .fade-leave-to { opacity: 0; }
.fade-enter-active .relative.bg-white, .fade-leave-active .relative.bg-white { transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1); }
.fade-enter-from .relative.bg-white, .fade-leave-to .relative.bg-white { opacity: 0; transform: scale(0.95) translateY(10px); }

/* Animasi Slide Modal Keranjang */
.slide-right-enter-active, .slide-right-leave-active { transition: transform 0.4s cubic-bezier(0.4, 0, 0.2, 1); }
.slide-right-enter-from, .slide-right-leave-to { transform: translateX(100%); }

.leaflet-container { z-index: 1 !important; }
</style>