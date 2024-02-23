<script setup lang="ts" generic="T extends any, O extends any">
import { invoke } from '@tauri-apps/api'
import { useToast } from 'primevue/usetoast'

defineOptions({
  name: 'IndexPage',
})

const toast = useToast()

const name = ref('')

async function onGreeting() {
  const pkgs = await invoke<any>('query_packages', { data: {} })

  toast.add({ severity: 'success', summary: 'Get package count', detail: pkgs?.total })
}
</script>

<template>
  <Toast position="bottom-right" />
  <div>
    <TheInput
      v-model="name"
      placeholder="What's your name?"
      autocomplete="false"
    />

    <Button
      @click="onGreeting"
    >
      greeting
    </Button>
  </div>
</template>
