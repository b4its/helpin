export const useAuth = () => {
  const { apiFetch } = useApi()
  const user = useState<any>('user', () => null)
  const isAuthenticated = computed(() => !!user.value)

  const login = async (email: string, password: string) => {
    const res = await apiFetch<any>('/api/auth/login', {
      method: 'POST',
      body: { email, password }
    })
    if (process.client) {
      localStorage.setItem('access_token', res.access_token)
      localStorage.setItem('refresh_token', res.refresh_token)
    }
    // Fetch full user profile (including role & name) after login
    await fetchMe()
    return { ...res, role: user.value?.role }
  }

  const register = async (data: { name: string; email: string; password: string; role: string }) => {
    const res = await apiFetch<any>('/api/auth/register', {
      method: 'POST',
      body: data
    })
    if (process.client) {
      localStorage.setItem('access_token', res.access_token)
      localStorage.setItem('refresh_token', res.refresh_token)
    }
    // Fetch full user profile after registration
    await fetchMe()
    return { ...res, role: user.value?.role || data.role }
  }

  const logout = () => {
    if (process.client) {
      localStorage.removeItem('access_token')
      localStorage.removeItem('refresh_token')
    }
    user.value = null
    navigateTo('/login')
  }

  const fetchMe = async () => {
    try {
      const res = await apiFetch<any>('/api/auth/me')
      user.value = res
      return res
    } catch {
      user.value = null
      return null
    }
  }

  const initAuth = async () => {
    if (process.client && localStorage.getItem('access_token')) {
      await fetchMe()
    }
  }

  return { user, isAuthenticated, login, register, logout, fetchMe, initAuth }
}
