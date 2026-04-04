import type { Account } from '@/types/account.d.ts'
import { http } from '@/lib/http'

export async function listAccounts(): Promise<Account[]> {
  const res = await http<{ code: number; data: Account[]; message: string }>('/api/accounts')
  return res.data || []
}

export async function createAccount(data: { platform: string; username: string; password: string }): Promise<Account> {
  const res = await http<{ code: number; data: Account; message: string }>('/api/accounts', {
    method: 'POST',
    body: JSON.stringify(data),
  })
  return res.data
}

export async function deleteAccount(accountId: string): Promise<void> {
  await http(`/api/accounts/${accountId}`, { method: 'DELETE' })
}
