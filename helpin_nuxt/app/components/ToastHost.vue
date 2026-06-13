<template>
  <Teleport to="body">
    <div class="fixed top-5 right-5 z-[9999] flex flex-col gap-3 w-[330px] max-w-[calc(100vw-2.5rem)] pointer-events-none">
      <TransitionGroup name="toast">
        <div
          v-for="t in toasts"
          :key="t.id"
          :class="[
            'pointer-events-auto flex items-start gap-3 rounded-xl shadow-lg border p-4 bg-white',
            borderClass(t.type),
          ]"
        >
          <div :class="['w-9 h-9 rounded-lg flex items-center justify-center shrink-0', iconBg(t.type)]">
            <component :is="iconFor(t.type)" class="w-5 h-5 text-white" />
          </div>
          <div class="flex-1 min-w-0 pt-0.5">
            <p class="text-sm font-black text-gray-800 leading-tight">{{ t.title }}</p>
            <p v-if="t.message" class="text-xs text-gray-500 font-medium mt-0.5 break-words">{{ t.message }}</p>
          </div>
          <button @click="remove(t.id)" class="text-gray-300 hover:text-gray-500 transition shrink-0">
            <XIcon class="w-4 h-4" />
          </button>
        </div>
      </TransitionGroup>
    </div>
  </Teleport>
</template>

<script setup>
import { CheckCircleIcon, XCircleIcon, InfoIcon, AlertTriangleIcon, XIcon } from 'lucide-vue-next'

const { toasts, remove } = useToast()

const iconFor = (type) => ({
  success: CheckCircleIcon,
  error: XCircleIcon,
  info: InfoIcon,
  warning: AlertTriangleIcon,
}[type] || InfoIcon)

const iconBg = (type) => ({
  success: 'bg-green-500',
  error: 'bg-red-500',
  info: 'bg-blue-500',
  warning: 'bg-amber-500',
}[type] || 'bg-gray-500')

const borderClass = (type) => ({
  success: 'border-green-100',
  error: 'border-red-100',
  info: 'border-blue-100',
  warning: 'border-amber-100',
}[type] || 'border-gray-100')
</script>

<style scoped>
.toast-enter-active,
.toast-leave-active {
  transition: all 0.3s cubic-bezier(0.16, 1, 0.3, 1);
}
.toast-enter-from {
  opacity: 0;
  transform: translateX(40px);
}
.toast-leave-to {
  opacity: 0;
  transform: translateX(40px);
}
.toast-move {
  transition: transform 0.3s ease;
}
</style>
