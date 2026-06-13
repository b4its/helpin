/**
 * Pusat format angka & mata uang untuk seluruh panel.
 *
 * Aturan:
 * - Semua nilai uang (bayar, uang masuk/keluar, total biaya, harga, saldo, dll)
 *   ditampilkan sebagai Rupiah (IDR) dengan pemisah ribuan titik.
 * - Backend menyimpan uang sebagai bilangan bulat (BIGINT) rupiah penuh,
 *   jadi tampilan default tanpa angka desimal.
 * - Angka non-uang (stok, qty, berat, suhu, persentase) diformat sesuai porsinya
 *   sehingga nilai desimal tidak terlihat aneh (mis. 12.0 -> "12", 12.50 -> "12,5").
 */
export const useFormat = () => {
  const toNumber = (v: unknown): number => {
    if (typeof v === 'number') return isFinite(v) ? v : 0
    if (typeof v === 'string') {
      const n = parseFloat(v.replace(/[^0-9.-]/g, ''))
      return isFinite(n) ? n : 0
    }
    return 0
  }

  /** Format mata uang IDR, mis. 1500000 -> "Rp 1.500.000". */
  const formatRupiah = (value: unknown): string => {
    const n = Math.round(toNumber(value))
    return new Intl.NumberFormat('id-ID', {
      style: 'currency',
      currency: 'IDR',
      minimumFractionDigits: 0,
      maximumFractionDigits: 0,
    }).format(n)
  }

  /** Sama seperti formatRupiah namun tanpa simbol "Rp" (untuk input/edit). */
  const formatNominal = (value: unknown): string => {
    const n = Math.round(toNumber(value))
    return new Intl.NumberFormat('id-ID').format(n)
  }

  /**
   * Format angka umum. Menampilkan hingga `maxDecimals` angka di belakang koma,
   * namun memangkas nol yang tidak perlu (12.0 -> "12", 12.50 -> "12,5").
   */
  const formatNumber = (value: unknown, maxDecimals = 2): string => {
    const n = toNumber(value)
    return new Intl.NumberFormat('id-ID', {
      minimumFractionDigits: 0,
      maximumFractionDigits: maxDecimals,
    }).format(n)
  }

  /** Angka + satuan, mis. formatUnit(67, "Pcs") -> "67 Pcs". */
  const formatUnit = (value: unknown, unit?: string | null, maxDecimals = 2): string => {
    const num = formatNumber(value, maxDecimals)
    return unit ? `${num} ${unit}` : num
  }

  /** Persentase, mis. 12.5 -> "12,5%". */
  const formatPercent = (value: unknown, maxDecimals = 1): string => {
    return `${formatNumber(value, maxDecimals)}%`
  }

  /**
   * Membersihkan input bermata uang/angka dari pengguna menjadi number murni
   * yang siap dikirim ke backend (mis. "Rp 1.500.000" -> 1500000).
   */
  const parseNumber = (value: unknown): number => toNumber(value)

  /** Format tanggal singkat Indonesia, mis. "13 Jun 2026". */
  const formatDate = (value: unknown): string => {
    if (!value) return '-'
    const d = new Date(value as string)
    if (isNaN(d.getTime())) return '-'
    return new Intl.DateTimeFormat('id-ID', { day: '2-digit', month: 'short', year: 'numeric' }).format(d)
  }

  /** Format tanggal + jam, mis. "13 Jun 2026, 11.45". */
  const formatDateTime = (value: unknown): string => {
    if (!value) return '-'
    const d = new Date(value as string)
    if (isNaN(d.getTime())) return '-'
    return new Intl.DateTimeFormat('id-ID', {
      day: '2-digit', month: 'short', year: 'numeric', hour: '2-digit', minute: '2-digit',
    }).format(d)
  }

  return {
    formatRupiah,
    formatNominal,
    formatNumber,
    formatUnit,
    formatPercent,
    parseNumber,
    formatDate,
    formatDateTime,
  }
}
