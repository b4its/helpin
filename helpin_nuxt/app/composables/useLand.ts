export const useLand = () => {
  const { apiFetch } = useApi()

  const list = async () => {
    return apiFetch<any[]>('/api/lands')
  }

  const create = async (data: any) => {
    return apiFetch<any>('/api/lands', { method: 'POST', body: data })
  }

  const update = async (id: string, data: any) => {
    return apiFetch<any>(`/api/lands/${id}`, { method: 'PUT', body: data })
  }

  const remove = async (id: string) => {
    return apiFetch<void>(`/api/lands/${id}`, { method: 'DELETE' })
  }

  const getInventory = async (landId: string) => {
    return apiFetch<any[]>(`/api/lands/${landId}/inventory`)
  }

  const addInventory = async (landId: string, data: any) => {
    return apiFetch<any>(`/api/lands/${landId}/inventory`, { method: 'POST', body: data })
  }

  const addPlant = async (landId: string, data: any) => {
    return apiFetch<any>(`/api/lands/${landId}/plants`, { method: 'POST', body: data })
  }

  const getHarvests = async (landId: string) => {
    return apiFetch<any[]>(`/api/lands/${landId}/harvests`)
  }

  // Inventory yang tersedia untuk lahan (hanya Pupuk, Bibit, Alat Pertanian)
  const listAvailableInventory = async () => {
    return apiFetch<any[]>('/api/inventory/available')
  }

  const createInventory = async (data: any) => {
    return apiFetch<any>('/api/inventory', { method: 'POST', body: data })
  }

  const updateInventory = async (id: string, data: any) => {
    return apiFetch<any>(`/api/inventory/${id}`, { method: 'PUT', body: data })
  }

  const deleteInventory = async (id: string) => {
    return apiFetch<void>(`/api/inventory/${id}`, { method: 'DELETE' })
  }

  // Harvest Prediction
  const predictHarvest = async (landId: string) => {
    return apiFetch<any>(`/api/predict/harvest/${landId}`, { method: 'POST' })
  }

  // Estimasi panen tersimpan (dari hasil prediksi)
  const getPrediction = async (landId: string) => {
    return apiFetch<any>(`/api/lands/${landId}/prediction`)
  }

  // Treatment (Tambah Informasi): pupuk + jumlah, alat → MongoDB + re-prediksi
  const addTreatment = async (landId: string, data: any) => {
    return apiFetch<any>(`/api/lands/${landId}/treatment`, { method: 'POST', body: data })
  }

  const listTreatments = async (landId: string) => {
    return apiFetch<any[]>(`/api/lands/${landId}/treatments`)
  }

  return {
    list, create, update, remove,
    getInventory, addInventory, addPlant, getHarvests,
    listAllInventory: async () => apiFetch<any[]>('/api/inventory'),
    listAvailableInventory,
    createInventory, updateInventory, deleteInventory,
    predictHarvest, getPrediction, addTreatment, listTreatments
  }
}
