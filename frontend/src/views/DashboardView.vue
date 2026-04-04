<script setup lang="ts">
import { onMounted } from 'vue'
import { useDashboard } from '@/composables/useDashboard'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { ListTodo, Users, FileText, Activity } from 'lucide-vue-next'

const { stats, isLoading, fetchStats } = useDashboard()

onMounted(fetchStats)
</script>

<template>
  <div class="space-y-6">
    <div>
      <h1 class="text-2xl font-bold tracking-tight">仪表盘</h1>
      <p class="text-muted-foreground">AI 内容发布平台概览</p>
    </div>

    <div v-if="isLoading" class="text-center py-12 text-muted-foreground">加载中...</div>

    <div v-else class="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
      <Card>
        <CardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <CardTitle class="text-sm font-medium">任务总数</CardTitle>
          <ListTodo class="h-4 w-4 text-muted-foreground" />
        </CardHeader>
        <CardContent>
          <div class="text-2xl font-bold">{{ stats.enabledTasks }}</div>
          <p class="text-xs text-muted-foreground">{{ stats.runningTasks }} 运行中</p>
        </CardContent>
      </Card>

      <Card>
        <CardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <CardTitle class="text-sm font-medium">账号数量</CardTitle>
          <Users class="h-4 w-4 text-muted-foreground" />
        </CardHeader>
        <CardContent>
          <div class="text-2xl font-bold">{{ stats.resultFiles }}</div>
          <p class="text-xs text-muted-foreground">已配置平台</p>
        </CardContent>
      </Card>

      <Card>
        <CardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <CardTitle class="text-sm font-medium">内容数量</CardTitle>
          <FileText class="h-4 w-4 text-muted-foreground" />
        </CardHeader>
        <CardContent>
          <div class="text-2xl font-bold">{{ stats.scannedItems }}</div>
          <p class="text-xs text-muted-foreground">已生成内容</p>
        </CardContent>
      </Card>

      <Card>
        <CardHeader class="flex flex-row items-center justify-between space-y-0 pb-2">
          <CardTitle class="text-sm font-medium">系统状态</CardTitle>
          <Activity class="h-4 w-4 text-emerald-500" />
        </CardHeader>
        <CardContent>
          <div class="text-2xl font-bold text-emerald-500">在线</div>
          <p class="text-xs text-muted-foreground">服务正常运行</p>
        </CardContent>
      </Card>
    </div>
  </div>
</template>
