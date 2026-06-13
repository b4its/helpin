export const useFinance = () => {
  const { apiFetch } = useApi()

  const getBalance = async () => {
    return apiFetch<any>('/api/finance/balance')
  }

  const recordExpense = async (data: any) => {
    return apiFetch<any>('/api/finance/expenses', { method: 'POST', body: data })
  }

  const recordIncome = async (data: any) => {
    return apiFetch<any>('/api/finance/income', { method: 'POST', body: data })
  }

  const getReport = async (start?: string, end?: string) => {
    const query = new URLSearchParams()
    if (start) query.set('start', start)
    if (end) query.set('end', end)
    const qs = query.toString()
    return apiFetch<any[]>(`/api/finance/report${qs ? '?' + qs : ''}`)
  }

  return { getBalance, recordExpense, recordIncome, getReport }
}
