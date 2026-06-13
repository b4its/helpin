export const useProductAdmin = () => {
  const { apiFetch } = useApi()

  const list = () => apiFetch<any[]>('/api/products')
  const get = (id: string) => apiFetch<any>(`/api/products/${id}`)
  const create = (d: any) => apiFetch<any>('/api/products', { method: 'POST', body: d })
  const update = (id: string, d: any) => apiFetch<any>(`/api/products/${id}`, { method: 'PUT', body: d })
  const remove = (id: string) => apiFetch<any>(`/api/products/${id}`, { method: 'DELETE' })
  const categories = () => apiFetch<any[]>('/api/categories')

  return { list, get, create, update, remove, categories }
}
