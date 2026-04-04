interface FetchOptions extends RequestInit {
  params?: Record<string, string | number | boolean | undefined>;
}

export async function http<T = any>(url: string, options: FetchOptions = {}): Promise<T> {
  const headers = new Headers(options.headers)

  // Auto set Content-Type for JSON
  if (options.body && !headers.get('Content-Type')) {
    headers.set('Content-Type', 'application/json')
  }

  let fullUrl = url
  if (options.params) {
    const searchParams = new URLSearchParams()
    Object.entries(options.params).forEach(([key, value]) => {
      if (value !== undefined && value !== null) {
        searchParams.append(key, String(value))
      }
    })
    const queryString = searchParams.toString()
    if (queryString) {
      fullUrl += (url.includes('?') ? '&' : '?') + queryString
    }
  }

  const config: RequestInit = {
    ...options,
    headers,
  }

  const response = await fetch(fullUrl, config)

  if (!response.ok) {
    const errorData = await response.json().catch(() => ({}))
    throw new Error(errorData.message || errorData.detail || `HTTP error! status: ${response.status}`)
  }

  if (response.status === 204) {
    return null as T
  }

  return response.json()
}
