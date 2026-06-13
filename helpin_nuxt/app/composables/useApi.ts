export const useApi = () => {
  const config = useRuntimeConfig()
  const baseURL = config.public.apiBase || 'http://localhost:8080'

  const getToken = (): string | null => {
    if (process.client) {
      return localStorage.getItem('access_token')
    }
    return null
  }

  const apiFetch = async <T>(endpoint: string, options: any = {}): Promise<T> => {
    const token = getToken()
    const headers: Record<string, string> = {
      'Content-Type': 'application/json',
      ...options.headers,
    }
    if (token) {
      headers['Authorization'] = `Bearer ${token}`
    }

    const response = await $fetch<T>(`${baseURL}${endpoint}`, {
      ...options,
      headers,
    })
    return response
  }

  return { apiFetch, baseURL, getToken }
}
