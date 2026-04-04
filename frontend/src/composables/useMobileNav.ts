import { ref } from 'vue'

const isMobileNavOpen = ref(false)

export function useMobileNav() {
  function closeMobileNav() {
    isMobileNavOpen.value = false
  }

  return {
    isMobileNavOpen,
    closeMobileNav,
  }
}
