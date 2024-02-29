<script setup lang="ts">
import type { ButtonProps } from 'primevue/button'
import type { DialogProps } from 'primevue/dialog'

const { btnProps, dialogProps, getLabel = v => ellipsisText(v, 18) } = defineProps<{
  getLabel?: (text: string) => string
  btnProps?: ButtonProps
  dialogProps?: DialogProps
}>()

const label = computed(() => getLabel?.(btnProps?.label || '') || '')

const visible = ref(false)
</script>

<template>
  <Button class="p-0 text-3" v-bind="btnProps" :label="label" @click="visible = true" />
  <Dialog
    v-model:visible="visible"
    modal
    header="Detail"
    v-bind="dialogProps"
    class="min-w-120 pb-10"
  >
    <slot />
  </Dialog>
</template>
