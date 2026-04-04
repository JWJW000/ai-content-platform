import { ref } from 'vue'
import type { Task } from '@/types/task.d.ts'
import * as api from '@/api/tasks'

const tasks = ref<Task[]>([])
const isLoading = ref(false)
const error = ref<string | null>(null)

export function useTasks() {
  async function fetchTasks() {
    isLoading.value = true
    error.value = null
    try {
      tasks.value = await api.getAllTasks()
    } catch (e) {
      error.value = (e as Error).message
    } finally {
      isLoading.value = false
    }
  }

  async function removeTask(taskId: string) {
    await api.deleteTask(taskId)
    await fetchTasks()
  }

  async function updateTask(taskId: string, data: any) {
    await api.updateTask(taskId, data)
    await fetchTasks()
  }

  async function startTask(taskId: string) {
    await api.startTask(taskId)
    await fetchTasks()
  }

  async function stopTask(taskId: string) {
    await api.stopTask(taskId)
    await fetchTasks()
  }

  return {
    tasks,
    isLoading,
    error,
    fetchTasks,
    removeTask,
    updateTask,
    startTask,
    stopTask,
    stoppingTaskIds: ref<string[]>([]),
  }
}
