export const usePos = () => {
  const { apiFetch } = useApi()

  const createTransaction = async (data: { items: any[]; subtotal: number; tax: number; total: number; amount_tendered: number; change_amount: number }) => {
    return apiFetch<any>('/api/pos/transactions', { method: 'POST', body: data })
  }

  const listTransactions = async () => {
    return apiFetch<any[]>('/api/pos/transactions')
  }

  return { createTransaction, listTransactions }
}
