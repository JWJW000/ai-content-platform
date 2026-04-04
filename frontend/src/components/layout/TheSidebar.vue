<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { RouterLink, useRoute } from 'vue-router'
import { useMobileNav } from '@/composables/useMobileNav'
import { LayoutDashboard, ListTodo, Users, Terminal } from 'lucide-vue-next'

const emit = defineEmits<{ (e: 'navigate'): void }>()
const { t } = useI18n()
const route = useRoute()
const { closeMobileNav } = useMobileNav()

const navItems = [
  { to: '/dashboard', label: t('sidebar.dashboard') || '仪表盘', icon: LayoutDashboard },
  { to: '/tasks', label: t('sidebar.tasks') || '任务管理', icon: ListTodo },
  { to: '/accounts', label: t('sidebar.accounts') || '账号管理', icon: Users },
  { to: '/logs', label: t('sidebar.logs') || '日志查看', icon: Terminal },
]

function isActive(path: string) {
  return route.path === path
}

function handleNavigate() {
  emit('navigate')
  closeMobileNav()
}
</script>

<template>
  <nav class="space-y-1">
    <RouterLink
      v-for="item in navItems"
      :key="item.to"
      :to="item.to"
      class="group relative flex items-center px-4 py-3 rounded-xl transition-all duration-200"
      :class="isActive(item.to) ? 'bg-primary/10 text-primary' : 'text-slate-600 hover:bg-slate-100'"
      @click="handleNavigate"
    >
      <component :is="item.icon" class="h-5 w-5 mr-3" />
      <span class="text-sm font-medium">{{ item.label }}</span>
    </RouterLink>
  </nav>
</template>
