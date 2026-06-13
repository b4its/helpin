export const useProducts = () => {
  const { apiFetch } = useApi()

  const listProducts = async (params?: { category_id?: string; search?: string; sort?: string }) => {
    const query = new URLSearchParams()
    if (params?.category_id) query.set('category_id', params.category_id)
    if (params?.search) query.set('search', params.search)
    if (params?.sort) query.set('sort', params.sort)
    const qs = query.toString()
    return apiFetch<any[]>(`/api/products${qs ? '?' + qs : ''}`)
  }

  const listCategories = async () => {
    return apiFetch<any[]>('/api/categories')
  }

  const getCart = async () => {
    return apiFetch<any[]>('/api/cart')
  }

  const addToCart = async (productId: string, quantity: number) => {
    return apiFetch<any>('/api/cart', { method: 'POST', body: { product_id: productId, quantity } })
  }

  const updateCartQty = async (itemId: string, quantity: number) => {
    return apiFetch<any>(`/api/cart/${itemId}`, { method: 'PUT', body: { quantity } })
  }

  const removeFromCart = async (itemId: string) => {
    return apiFetch<void>(`/api/cart/${itemId}`, { method: 'DELETE' })
  }

  const checkout = async (shippingAddress?: string) => {
    return apiFetch<any>('/api/orders', { method: 'POST', body: { shipping_address: shippingAddress } })
  }

  const listOrders = async () => {
    return apiFetch<any[]>('/api/orders')
  }

  return { listProducts, listCategories, getCart, addToCart, updateCartQty, removeFromCart, checkout, listOrders }
}
