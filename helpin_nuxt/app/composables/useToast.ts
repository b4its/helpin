import { ref } from 'vue'

export interface ToastItem {
  id: number
  type: 'success' | 'error' | 'info' | 'warning'
  title: string
  message?: string
  timeout: number
}

// Singleton store (module-level) so every caller shares the same queue.
const toasts = ref<ToastItem[]>([])
let seq = 0

const remove = (id: number) => {
  const i = toasts.value.findIndex((t) => t.id === id)
  if (i !== -1) toasts.value.splice(i, 1)
}

const push = (
  type: ToastItem['type'],
  title: string,
  message?: string,
  timeout = 3500,
) => {
  const id = ++seq
  toasts.value.push({ id, type, title, message, timeout })
  if (timeout > 0) {
    setTimeout(() => remove(id), timeout)
  }
  return id
}

/**
 * Notifikasi toast global (gaya toastr). Dipakai di seluruh halaman & proses.
 *
 * Contoh:
 *   const toast = useToast()
 *   toast.success('Produk disimpan')
 *   toast.error('Gagal menyimpan', detailPesan)
 */
export const useToast = () => {
  return {
    toasts,
    remove,
    success: (title: string, message?: string, timeout?: number) => push('success', title, message, timeout),
    error: (title: string, message?: string, timeout?: number) => push('error', title, message, timeout ?? 5000),
    info: (title: string, message?: string, timeout?: number) => push('info', title, message, timeout),
    warning: (title: string, message?: string, timeout?: number) => push('warning', title, message, timeout),
  }
}
