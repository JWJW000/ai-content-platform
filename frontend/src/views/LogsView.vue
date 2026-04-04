<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { getLogs, type Log } from '@/api/logs'
import { Button } from '@/components/ui/button'
import { Card, CardContent } from '@/components/ui/card'
import { RefreshCw } from 'lucide-vue-next'

const { t } = useI18n()
const logs = ref<Log[]>([])
const isLoading = ref(true)

async function loadLogs() {
  isLoading.value = true
  try {
    logs.value = await getLogs()
  } catch (e) {
    console.error('Failed to load logs:', e)
  } finally {
    isLoading.value = false
  }
}

function getLevelClass(level: string) {
  switch (level) {
    case 'error': return 'text-red-500 bg-red-500/10'
    case 'warn': return 'text-yellow-500 bg-yellow-500/10'
    default: return 'text-green-500 bg-green-500/10'
  }
}

onMounted(loadLogs)
</script>

<template>
  <div class="space-y-6">
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-bold tracking-tight">{{ t('logs.title') || '日志查看' }}</h1>
        <p class="text-muted-foreground">{{ t('logs.subtitle') || '查看系统运行日志' }}</p>
      </div>
      <Button @click="loadLogs">
        <RefreshCw class="h-4 w-4 mr-2" />
        {{ t('common.refresh') || '刷新' }}
      </Button>
    </div>

    <div v-if="isLoading" class="text-center py-12 text-muted-foreground">加载中...</div>

    <div v-else-if="logs.length === 0" class="text-center py-12 text-muted-foreground">
      {{ t('logs.empty') || '暂无日志' }}
    </div>

    <div v-else class="space-y-2">
      <Card v-for="log in logs" :key="log.id">
        <CardContent class="py-3">
          <div class="flex items-start gap-3">
            <span 
              class="px-2 py-1 rounded text-xs font-mono font-bold"
              :class="getLevelClass(log.level)"
            >
              {{ log.level.toUpperCase() }}
            </span>
            <span class="text-sm flex-1">{{ log.message }}</span>
            <span class="text-xs text-muted-foreground whitespace-nowrap">
              {{ new Date(log.created_at).toLocaleString() }}
            </span>
          </div>
        </CardContent>
      </Card>
    </div>
  </div>
</template>
