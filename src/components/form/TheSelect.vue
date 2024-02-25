<script setup lang="ts">
import type { DropdownProps } from 'primevue/dropdown'

interface Props {
  name: string
  label?: string
  controlProps?: DropdownProps
}

const {
  name,
  label,
  controlProps,
} = defineProps<Props>()

const isSubmitting = useIsSubmitting()
const { value, errorMessage } = useField<any>(() => name, undefined, {
  validateOnValueUpdate: false,
})
</script>

<template>
  <FloatLabel>
    <Dropdown
      v-bind="controlProps"
      v-model="value"
      class="w-full"
      :disabled="isSubmitting"
      :invalid="!!errorMessage"
    />
    <label>{{ label }}</label>
  </FloatLabel>
</template>
