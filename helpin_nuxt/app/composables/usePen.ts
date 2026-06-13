export const usePen = () => {
  const { apiFetch } = useApi()

  const list = async () => apiFetch<any[]>('/api/pens')
  const create = async (data: any) => apiFetch<any>('/api/pens', { method: 'POST', body: data })
  const update = async (id: string, data: any) => apiFetch<any>(`/api/pens/${id}`, { method: 'PUT', body: data })
  const remove = async (id: string) => apiFetch<void>(`/api/pens/${id}`, { method: 'DELETE' })

  // Ternak di dalam kandang
  const listLivestock = async (id: string) => apiFetch<any[]>(`/api/pens/${id}/livestock`)

  // Analisa pakan per kandang (top-tier feeds + proyeksi kualitas)
  const analyzeFeed = async (id: string) => apiFetch<any>(`/api/pens/${id}/feed-analysis`, { method: 'POST' })
  const getFeedAnalysis = async (id: string) => apiFetch<any[]>(`/api/pens/${id}/feed-analysis`)

  return { list, create, update, remove, listLivestock, analyzeFeed, getFeedAnalysis }
}
