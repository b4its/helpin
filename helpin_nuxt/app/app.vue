<template>
  <div>
    <NuxtPage />
    <ToastHost />
  </div>
</template>

<script setup>
import ToastHost from '~/components/ToastHost.vue'
const { initAuth } = useAuth()

onMounted(async () => {
  await initAuth()
})
</script>

<style>
/* Reset dasar dan font setup */
html {
  scroll-behavior: smooth;
  font-family: 'Inter', sans-serif;
}
body {
  @apply bg-[#f8fdf9] text-gray-800;
}
</style>