<script setup lang="ts">
import { useMobileNav } from '@/composables/useMobileNav'
import { Menu } from 'lucide-vue-next'

const { isMobileNavOpen } = useMobileNav()
</script>

<template>
  <header class="sticky top-0 z-40 w-full border-b border-slate-200/60 bg-white/80 backdrop-blur-md">
    <div class="flex h-16 items-center gap-4 px-4">
      <button 
        class="md:hidden rounded-lg p-2 hover:bg-slate-100"
        @click="isMobileNavOpen = !isMobileNavOpen"
      >
        <Menu class="h-5 w-5" />
      </button>
      <div class="flex-1">
        <h2 class="text-lg font-semibold">AI 内容发布平台</h2>
      </div>
      <div class="flex items-center gap-2">
        <span class="text-xs text-emerald-500 font-medium">● 在线</span>
      </div>
    </div>
  </header>
</template>
