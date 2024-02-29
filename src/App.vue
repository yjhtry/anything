<script setup lang="ts">
import { usePrimeVue } from 'primevue/config'

const primeVue = usePrimeVue()

watch(isDark, (value) => {
  if (value)
    primeVue.changeTheme('aura-light-green', 'aura-dark-green', 'theme-link', () => {})

  else
    primeVue.changeTheme('aura-dark-green', 'aura-light-green', 'theme-link', () => {})
}, { immediate: true })
</script>

<template>
  <main font-sans>
    <TheNav />
    <RouterView />
  </main>
</template>
