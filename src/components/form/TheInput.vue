<script setup lang="ts">
import type { InputTextProps } from 'primevue/inputtext'
import { useField } from 'vee-validate'

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
  </FloatLabel>
</template>
