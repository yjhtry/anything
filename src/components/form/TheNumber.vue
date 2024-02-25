<script setup lang="ts">
import type { InputNumberProps } from 'primevue/inputnumber'

interface Props {
  name: string
  label?: string
  controlProps?: InputNumberProps
}

const {
  name,
  label,
  controlProps,
} = defineProps<Props>()

const isSubmitting = useIsSubmitting()
const { errorMessage, value } = useField<number>(() => name, undefined, {
  validateOnValueUpdate: false,
})
</script>

<template>
  <FloatLabel>
    <InputNumber
      v-bind="controlProps"
      v-model="value"
      :disabled="isSubmitting"
      :invalid="!!errorMessage"
      w-full
    />
    <label>{{ label }}</label>
    <span v-if="!!errorMessage" class="text-xs text-red-500">{{ errorMessage }}</span>
  </FloatLabel>
</template>
