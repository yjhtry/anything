<script setup lang="ts">
import type { InputTextProps } from 'primevue/inputtext'

interface Props {
  name: string
  label?: string
  inputProps?: InputTextProps
}

const {
  name,
  label,
  inputProps,
} = defineProps<Props>()

const { errorMessage, value } = useField<string>(() => name, undefined, {
  validateOnValueUpdate: false,
})
</script>

<template>
  <FloatLabel>
    <InputText
      v-bind="inputProps"
      v-model="value"
      :invalid="!!errorMessage"
      type="text"
      w-full
    />
    <label>{{ label }}</label>
    <span v-if="!!errorMessage" class="text-xs text-red-500">{{ errorMessage }}</span>
  </FloatLabel>
</template>
