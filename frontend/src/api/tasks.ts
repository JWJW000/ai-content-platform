import type { Task, TaskUpdate } from '@/types/task.d.ts'
import { http } from '@/lib/http'

export async function getAllTasks(): Promise<Task[]> {
  const res = await http<{ code: number; data: Task[]; message: string }>('/api/tasks')
  return res.data || []
}

export async function createTask(data: any): Promise<Task> {
  const res = await http<{ code: number; data: Task; message: string }>('/api/tasks', {
    method: 'POST',
    body: JSON.stringify(data),
  })
  return res.data
}

export async function updateTask(taskId: string, data: TaskUpdate): Promise<Task> {
  const res = await http<{ code: number; data: Task; message: string }>(`/api/tasks/${taskId}`, {
    method: 'PATCH',
    body: JSON.stringify(data),
  })
  return res.data
}

export async function startTask(taskId: string): Promise<void> {
  await http(`/api/tasks/${taskId}/start`, { method: 'POST' })
}

export async function stopTask(taskId: string): Promise<void> {
  await http(`/api/tasks/${taskId}/stop`, { method: 'POST' })
}

export async function deleteTask(taskId: string): Promise<void> {
  await http(`/api/tasks/${taskId}`, { method: 'DELETE' })
}
