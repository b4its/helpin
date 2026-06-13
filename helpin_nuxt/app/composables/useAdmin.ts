export const useAdmin = () => {
  const { apiFetch } = useApi()

  // Pengguna
  const listUsers = () => apiFetch<any[]>('/api/admin/users')

  // Karyawan
  const listEmployees = () => apiFetch<any[]>('/api/admin/employees')
  const createEmployee = (d: any) => apiFetch<any>('/api/admin/employees', { method: 'POST', body: d })
  const updateEmployee = (id: string, d: any) => apiFetch<any>(`/api/admin/employees/${id}`, { method: 'PUT', body: d })
  const deleteEmployee = (id: string) => apiFetch<any>(`/api/admin/employees/${id}`, { method: 'DELETE' })

  // Supplier
  const listSuppliers = () => apiFetch<any[]>('/api/admin/suppliers')
  const createSupplier = (d: any) => apiFetch<any>('/api/admin/suppliers', { method: 'POST', body: d })
  const updateSupplier = (id: string, d: any) => apiFetch<any>(`/api/admin/suppliers/${id}`, { method: 'PUT', body: d })
  const deleteSupplier = (id: string) => apiFetch<any>(`/api/admin/suppliers/${id}`, { method: 'DELETE' })

  // Aktivitas (audit trail dari MongoDB)
  const listActivities = () => apiFetch<any[]>('/api/admin/activities')

  // Statistik dashboard
  const stats = () => apiFetch<any>('/api/admin/stats')

  return {
    listUsers,
    listEmployees, createEmployee, updateEmployee, deleteEmployee,
    listSuppliers, createSupplier, updateSupplier, deleteSupplier,
    listActivities, stats,
  }
}
