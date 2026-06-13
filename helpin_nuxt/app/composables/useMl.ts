export const useMl = () => {
  const { apiFetch } = useApi()

  const predictFeed = async (data: { tag_id: string; breed: string; weight: number; livestock_id?: string }) => {
    return apiFetch<any>('/api/ml/feed-recommendation', { method: 'POST', body: data })
  }

  const evaluateHealth = async (data: any) => {
    return apiFetch<any>('/api/ml/health-evaluation', { method: 'POST', body: data })
  }

  const getFeedHistory = async () => {
    return apiFetch<any[]>('/api/ml/feed-history')
  }

  return { predictFeed, evaluateHealth, getFeedHistory }
}
