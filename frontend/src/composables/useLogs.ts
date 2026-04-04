import { ref } from 'vue'
import { getLogs, type Log } from '@/api/logs'

const logs = ref<Log[]>([])
const isLoading = ref(false)
const error = ref<string | null>(null)

export function useLogs() {
  async function fetchLogs() {
    isLoading.value = true
    error.value = null
    try {
      logs.value = await getLogs()
    } catch (e) {
      error.value = (e as Error).message
    } finally {
      isLoading.value = false
    }
  }

  return {
    logs,
    isLoading,
    error,
    fetchLogs,
  }
}
