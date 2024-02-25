<script setup lang="ts">
import type { TreeSelectProps } from 'primevue/treeselect'

interface Props {
  name: string
  label?: string
  controlProps?: TreeSelectProps
  transform?: (value: any) => any
}

const {
  name,
  label,
  controlProps,
  transform = v => v,
} = defineProps<Props>()

const _value = ref<any>()

const isSubmitting = useIsSubmitting()
const { value, handleChange, errorMessage } = useField<any>(() => name, undefined, {
  validateOnValueUpdate: false,
})
function onChange(checked: any) {
  log(controlProps)
  const mode = controlProps?.selectionMode || 'single'

  const result = Object.keys(checked).map(transform)

  handleChange(mode === 'single' ? result[0] : result)
}

watchEffect(() => {
  const list = Array.isArray(value.value) ? value.value : value ? [value] : []

  _value.value = list.reduce((acc: any, cur: any) => {
    acc[cur] = true
    return acc
  }, {})
})
</script>

<template>
  <FloatLabel>
    <TreeSelect
      v-bind="controlProps"
      :model-value="_value"
      class="w-full"
      :disabled="isSubmitting"
      :invalid="!!errorMessage"
      @change="onChange"
    />
    <label>{{ label }}</label>
  </FloatLabel>
</template>
