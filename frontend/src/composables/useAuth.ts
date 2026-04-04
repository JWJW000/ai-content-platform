import { ref } from 'vue'

const isLoggedIn = ref(true)  // Always logged in for now

export function useAuth() {
  function logout() {
    // No-op for now
  }

  function login(_user: string, _pass: string): Promise<boolean> {
    return Promise.resolve(true)
  }

  return {
    isAuthenticated: isLoggedIn,
    login,
    logout,
  }
}
