<script setup lang="ts">
import { onMounted } from 'vue'
import { useTasks } from '@/composables/useTasks'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card'
import { Play, Square, Trash2, RefreshCw } from 'lucide-vue-next'

const { tasks, isLoading, fetchTasks, startTask, stopTask, removeTask } = useTasks()

async function handleStart(task: any) {
  await startTask(task.id)
}

async function handleStop(task: any) {
  await stopTask(task.id)
}

async function handleDelete(task: any) {
  if (confirm(`确定删除任务 "${task.name}" 吗？`)) {
    await removeTask(task.id)
  }
}

onMounted(fetchTasks)
</script>

<template>
  <div class="space-y-6">
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-bold tracking-tight">任务管理</h1>
        <p class="text-muted-foreground">管理 AI 内容生成任务</p>
      </div>
      <Button @click="fetchTasks">
        <RefreshCw class="h-4 w-4 mr-2" />
        刷新
      </Button>
    </div>

    <div v-if="isLoading" class="text-center py-12 text-muted-foreground">加载中...</div>

    <div v-else-if="tasks.length === 0" class="text-center py-12 text-muted-foreground">
      暂无任务
    </div>

    <div v-else class="grid gap-4">
      <Card v-for="task in tasks" :key="task.id">
        <CardHeader class="flex flex-row items-center justify-between">
          <CardTitle class="text-lg">{{ task.name }}</CardTitle>
          <div class="flex gap-2">
            <Button 
              v-if="task.status !== 'running'" 
              size="sm" 
              @click="handleStart(task)"
            >
              <Play class="h-4 w-4" />
            </Button>
            <Button 
              v-else 
              size="sm" 
              variant="destructive"
              @click="handleStop(task)"
            >
              <Square class="h-4 w-4" />
            </Button>
            <Button 
              size="sm" 
              variant="destructive"
              @click="handleDelete(task)"
            >
              <Trash2 class="h-4 w-4" />
            </Button>
          </div>
        </CardHeader>
        <CardContent>
          <div class="flex gap-6 text-sm text-muted-foreground">
            <span>平台: {{ task.platform }}</span>
            <span>状态: {{ task.status }}</span>
            <span>内容数: {{ task.content_count }}</span>
          </div>
        </CardContent>
      </Card>
    </div>
  </div>
</template>
