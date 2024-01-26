<script setup lang="ts" generic="T extends any, O extends any">
import { invoke } from '@tauri-apps/api'
import { useToast } from 'primevue/usetoast'

defineOptions({
  name: 'IndexPage',
})

const toast = useToast()

const name = ref('')

async function onGreeting() {
  const res = await invoke<string>('hello', { name: name.value })

  toast.add({ severity: 'success', summary: 'Greeting', detail: res, life: 300000 })
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
