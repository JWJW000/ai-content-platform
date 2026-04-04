<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { listAccounts, deleteAccount } from '@/api/accounts'
import type { Account } from '@/types/account.d'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Trash2, RefreshCw, User } from 'lucide-vue-next'

const accounts = ref<Account[]>([])
const isLoading = ref(true)

async function loadAccounts() {
  isLoading.value = true
  try {
    accounts.value = await listAccounts()
  } catch (e) {
    console.error('Failed to load accounts:', e)
  } finally {
    isLoading.value = false
  }
}

async function handleDelete(account: Account) {
  if (confirm(`确定删除账号 "${account.username}" 吗？`)) {
    await deleteAccount(account.id)
    await loadAccounts()
  }
}

onMounted(loadAccounts)
</script>

<template>
  <div class="space-y-6">
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-bold tracking-tight">账号管理</h1>
        <p class="text-muted-foreground">管理发布平台账号</p>
      </div>
      <Button @click="loadAccounts">
        <RefreshCw class="h-4 w-4 mr-2" />
        刷新
      </Button>
    </div>

    <div v-if="isLoading" class="text-center py-12 text-muted-foreground">加载中...</div>

    <div v-else-if="accounts.length === 0" class="text-center py-12 text-muted-foreground">
      暂无账号
    </div>

    <div v-else class="grid gap-4">
      <Card v-for="account in accounts" :key="account.id">
        <CardHeader class="flex flex-row items-center justify-between">
          <div class="flex items-center gap-3">
            <User class="h-5 w-5" />
            <CardTitle>{{ account.username }}</CardTitle>
          </div>
          <Button size="sm" variant="destructive" @click="handleDelete(account)">
            <Trash2 class="h-4 w-4" />
          </Button>
        </CardHeader>
        <CardContent>
          <div class="flex gap-6 text-sm text-muted-foreground">
            <span>平台: {{ account.platform }}</span>
            <span>状态: {{ account.status }}</span>
          </div>
        </CardContent>
      </Card>
    </div>
  </div>
</template>
