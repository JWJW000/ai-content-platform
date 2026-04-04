import { http } from '@/lib/http'

export interface Log {
  id: string
  level: 'info' | 'warn' | 'error'
  message: string
  task_id?: string
  created_at: string
}

export async function getLogs(): Promise<Log[]> {
  const res = await http<{ code: number; data: Log[]; message: string }>('/api/logs')
  return res.data || []
}
