<script setup lang="ts">
import type { TextareaProps } from 'primevue/textarea'

interface Props {
  name: string
  label?: string
  controlProps?: TextareaProps
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
    <Textarea
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
