<script setup lang="ts">
import type { InputTextProps } from 'primevue/inputtext'

interface Props {
  name: string
  label?: string
  controlProps?: InputTextProps
}

const {
  name,
  label,
  controlProps,
} = defineProps<Props>()

const isSubmitting = useIsSubmitting()

const { errorMessage, value } = useField<string>(() => name, undefined, {
  validateOnValueUpdate: false,
})
</script>

<template>
  <FloatLabel>
    <InputText
      v-bind="controlProps"
      v-model="value"
      :disabled="isSubmitting"
      :invalid="!!errorMessage"
      type="text"
      w-full
    />
    <label>{{ label }}</label>
    <span v-if="!!errorMessage" class="text-xs text-red-500">{{ errorMessage }}</span>
  </FloatLabel>
</template>
