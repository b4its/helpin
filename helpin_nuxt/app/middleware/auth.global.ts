export default defineNuxtRouteMiddleware(async (to) => {
  if (process.server) return

  const publicPaths = ['/login', '/']
  if (publicPaths.includes(to.path)) return

  const token = localStorage.getItem('access_token')
  if (!token) {
    return navigateTo('/login')
  }

  // Role-based panel access guard
  const { user, fetchMe } = useAuth()

  // Ensure user is loaded
  if (!user.value) {
    await fetchMe()
  }

  const role = user.value?.role
  if (!role) return // token exists but profile failed — let page handle error

  const path = to.path

  // Define which roles can access which panels
  const fullAccess = ['admin', 'karyawan']     // semua panel
  const petaniPaths = ['/panel_petani']
  const peternakPaths = ['/panel_peternak']
  const hybridPaths = ['/panel_hybrid']        // admin/karyawan only
  const ecommercePaths = ['/ecommerce']        // petani, peternak, pembeli, supplier, admin

  if (fullAccess.includes(role)) return // admin & karyawan boleh semua

  if (petaniPaths.some(p => path.startsWith(p)) && role !== 'petani') {
    return navigateTo(getRoleHome(role))
  }

  if (peternakPaths.some(p => path.startsWith(p)) && role !== 'peternak') {
    return navigateTo(getRoleHome(role))
  }

  if (hybridPaths.some(p => path.startsWith(p)) && !fullAccess.includes(role)) {
    return navigateTo(getRoleHome(role))
  }
})

function getRoleHome(role: string): string {
  const map: Record<string, string> = {
    petani: '/panel_petani/dashboard_petani',
    peternak: '/panel_peternak/dashboard_peternak',
    admin: '/panel_admin/dashboard_admin',
    karyawan: '/panel_karyawan/dashboard_karyawan',
    pembeli: '/ecommerce/produk',
    supplier: '/ecommerce/produk',
  }
  return map[role] || '/login'
}
