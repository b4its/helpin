export const useLivestock = () => {
  const { apiFetch } = useApi()

  const list = async (params?: { category?: string; search?: string }) => {
    const query = new URLSearchParams()
    if (params?.category) query.set('category', params.category)
    if (params?.search) query.set('search', params.search)
    const qs = query.toString()
    return apiFetch<any[]>(`/api/livestock${qs ? '?' + qs : ''}`)
  }

  const create = async (data: any) => {
    return apiFetch<any>('/api/livestock', { method: 'POST', body: data })
  }

  const getById = async (id: string) => {
    return apiFetch<any>(`/api/livestock/${id}`)
  }

  const update = async (id: string, data: any) => {
    return apiFetch<any>(`/api/livestock/${id}`, { method: 'PUT', body: data })
  }

  const remove = async (id: string) => {
    return apiFetch<void>(`/api/livestock/${id}`, { method: 'DELETE' })
  }

  const recordHealth = async (id: string, data: any = {}) => {
    return apiFetch<any>(`/api/livestock/${id}/health`, { method: 'POST', body: data })
  }

  // Generate kondisi kesehatan otomatis (tanpa input biometrik manual)
  const generateHealth = async (id: string) => {
    return apiFetch<any>(`/api/livestock/${id}/health`, { method: 'POST', body: {} })
  }

  const getHealthHistory = async (id: string) => {
    return apiFetch<any[]>(`/api/livestock/${id}/health-history`)
  }

  return { list, create, getById, update, remove, recordHealth, generateHealth, getHealthHistory }
}
