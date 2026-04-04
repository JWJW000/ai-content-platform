import { ref } from 'vue'

const isConnected = ref(true)  // Always connected for now

export function useWebSocket() {
  function start() {
    // No-op
  }

  function stop() {
    // No-op
  }

  return {
    isConnected,
    start,
    stop,
  }
}
