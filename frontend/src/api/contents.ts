import type { Content } from '@/types/content.d.ts'
import { http } from '@/lib/http'

export async function listContents(): Promise<Content[]> {
  const res = await http<{ code: number; data: Content[]; message: string }>('/api/contents')
  return res.data || []
}

export async function getContent(id: string): Promise<Content> {
  const res = await http<{ code: number; data: Content; message: string }>(`/api/contents/${id}`)
  return res.data
}

export async function reviewContent(id: string, approved: boolean): Promise<void> {
  await http(`/api/contents/${id}/review`, {
    method: 'POST',
    body: JSON.stringify({ approved }),
  })
}
