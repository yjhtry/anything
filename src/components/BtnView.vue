<script setup lang="ts">
import type { ButtonProps } from 'primevue/button'
import type { DialogProps } from 'primevue/dialog'

const { btnProps, dialogProps, getLabel = v => ellipsisText(v, 32) } = defineProps<{
  getLabel?: (text: string) => string
  btnProps?: ButtonProps
  dialogProps?: DialogProps
}>()

const label = computed(() => getLabel?.(btnProps?.label || '') || '')

const visible = ref(false)
</script>

<template>
  <Button v-bind="btnProps" :label="label" @click="visible = true" />
  <Dialog v-model:visible="visible" header="Detail" v-bind="dialogProps" modal class="min-w-120">
    <slot />
  </Dialog>
</template>
