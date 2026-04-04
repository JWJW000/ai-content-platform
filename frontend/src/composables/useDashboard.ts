import { ref } from 'vue'
import type { Task } from '@/types/task.d.ts'
import { getAllTasks } from '@/api/tasks'

const stats = ref({
  enabledTasks: 0,
  runningTasks: 0,
  scannedItems: 0,
  resultFiles: 0,
})

const focusTask = ref<Task | null>(null)
const focusInsights = ref<string[]>([])
const suggestion = ref('')
const activities = ref<any[]>([])
const isLoading = ref(false)
const error = ref<string | null>(null)

export function useDashboard() {
  async function fetchStats() {
    isLoading.value = true
    try {
      const tasks = await getAllTasks()
      
      stats.value = {
        enabledTasks: tasks.length,
        runningTasks: tasks.filter((t: Task) => t.status === 'running').length,
        scannedItems: tasks.length,
        resultFiles: tasks.length,
      }
      
      focusTask.value = tasks.length > 0 ? tasks[0] ?? null : null
    } catch (e) {
      error.value = (e as Error).message
    } finally {
      isLoading.value = false
    }
  }

  return {
    stats,
    focusTask,
    focusInsights,
    suggestion,
    activities,
    isLoading,
    error,
    fetchStats,
  }
}
