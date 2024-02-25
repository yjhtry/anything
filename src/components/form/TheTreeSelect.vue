<script setup lang="ts">
import type { TreeSelectProps } from 'primevue/treeselect'

interface Props {
  name: string
  label?: string
  treeProps?: TreeSelectProps
  transform?: (value: any) => any
}

const {
  name,
  label,
  treeProps,
  transform = v => v,
} = defineProps<Props>()

const _value = ref<any>()

const { value, handleChange, errorMessage } = useField<any>(() => name, undefined, {
  validateOnValueUpdate: false,
})
function onChange(checked: any) {
  const mode = treeProps?.selectionMode || 'single'

  const result = Object.keys(checked).map(transform)

  handleChange(mode === 'single' ? result[0] : result)

  handleChange(result)
}

watchEffect(() => {
  _value.value = value.value?.reduce((acc: any, cur: any) => {
    acc[cur] = true
    return acc
  }, {})
})
</script>

<template>
  <FloatLabel>
    <TreeSelect
      v-bind="treeProps"
      :model-value="_value"
      class="w-full"
      :invalid="!!errorMessage"
      @change="onChange"
    />
    <label>{{ label }}</label>
  </FloatLabel>
</template>
