<script setup lang="ts">
import type { InputTextProps } from 'primevue/inputtext'
import { useField } from 'vee-validate'

interface Props extends InputTextProps {
  name: string
  label?: string
  placeholder?: string
  validateTrigger?: 'change' | 'blur' | 'input'
}

const {
  name,
  label,
  validateTrigger = 'input',
  ...rest
} = defineProps<Props>()

const { errorMessage, value, handleChange, handleBlur } = useField<string>(() => name, undefined, {
  validateOnValueUpdate: false,
})

const validationListeners = {
  blur: (evt: Event) => handleBlur(evt, validateTrigger === 'blur'),
  change: (evt: Event) => handleChange(evt, validateTrigger === 'change'),
  input: (evt: Event) => handleChange(evt, validateTrigger === 'input'),
}
</script>

<template>
  <FloatLabel>
    <InputText
      v-bind="rest"
      v-model="value"
      :value="value"
      :invalid="!!errorMessage"
      type="text"
      v-on="validationListeners"
    />
    <label>{{ label }}</label>
  </FloatLabel>
</template>
