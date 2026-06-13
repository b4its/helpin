// Shared refresh state across all callers so concurrent 401s only trigger
// a single refresh request instead of a stampede.
let refreshPromise: Promise<string | null> | null = null

export const useApi = () => {
  const config = useRuntimeConfig()
  const baseURL = config.public.apiBase || 'http://localhost:8080'

  const getToken = (): string | null => {
    if (import.meta.client) {
      return localStorage.getItem('access_token')
    }
    return null
  }

  const getRefreshToken = (): string | null => {
    if (import.meta.client) {
      return localStorage.getItem('refresh_token')
    }
    return null
  }

  const clearSession = () => {
    if (import.meta.client) {
      localStorage.removeItem('access_token')
      localStorage.removeItem('refresh_token')
    }
  }

  // Try to obtain a new access token using the stored refresh token.
  // De-duplicated: concurrent callers await the same in-flight promise.
  const refreshAccessToken = async (): Promise<string | null> => {
    if (refreshPromise) return refreshPromise

    const refreshToken = getRefreshToken()
    if (!refreshToken) return null

    refreshPromise = (async () => {
      try {
        const res = await $fetch<{ access_token: string }>(`${baseURL}/api/auth/refresh`, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: { refresh_token: refreshToken },
        })
        if (res?.access_token && import.meta.client) {
          localStorage.setItem('access_token', res.access_token)
        }
        return res?.access_token ?? null
      } catch {
        return null
      } finally {
        // Reset after a tick so subsequent independent 401s can refresh again.
        refreshPromise = null
      }
    })()

    return refreshPromise
  }

  const apiFetch = async <T>(endpoint: string, options: any = {}): Promise<T> => {
    const buildHeaders = (token: string | null): Record<string, string> => {
      const headers: Record<string, string> = {
        'Content-Type': 'application/json',
        ...options.headers,
      }
      if (token) headers['Authorization'] = `Bearer ${token}`
      return headers
    }

    const doRequest = (token: string | null) =>
      $fetch<T>(`${baseURL}${endpoint}`, { ...options, headers: buildHeaders(token) })

    try {
      return await doRequest(getToken())
    } catch (err: any) {
      const status = err?.response?.status || err?.statusCode || err?.status
      // Only attempt a refresh+retry on auth failures, and never for the
      // refresh/login endpoints themselves (avoids loops).
      const isAuthEndpoint = endpoint.includes('/api/auth/')
      if (status === 401 && !isAuthEndpoint) {
        const newToken = await refreshAccessToken()
        if (newToken) {
          return await doRequest(newToken)
        }
        // Refresh failed → session is dead. Clean up and bounce to login.
        clearSession()
        if (import.meta.client) {
          await navigateTo('/login')
        }
      }
      throw err
    }
  }

  return { apiFetch, baseURL, getToken }
}
