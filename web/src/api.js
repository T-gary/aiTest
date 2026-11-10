export async function api(path, options = {}) {
  const headers = { 'Content-Type': 'application/json', ...(options.headers || {}) }
  const token = localStorage.getItem('token')
  if (token) headers.Authorization = `Bearer ${token}`
  const res = await fetch(`/api${path}`, { ...options, headers })
  const data = await res.json().catch(() => ({}))
  if (!res.ok) {
    if (res.status === 401) localStorage.removeItem('token')
    throw new Error(data.error || `请求失败 (${res.status})`)
  }
  return data
}

export const auth = {
  token: () => localStorage.getItem('token'),
  save(r) {
    localStorage.setItem('token', r.token)
    localStorage.setItem('user', JSON.stringify({ username: r.username, is_admin: r.is_admin }))
  },
  user() {
    try {
      return JSON.parse(localStorage.getItem('user') || 'null')
    } catch {
      return null
    }
  },
  logout() {
    localStorage.removeItem('token')
    localStorage.removeItem('user')
  }
}
