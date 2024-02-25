<script setup lang="ts">
import type { TreeSelectProps } from 'primevue/treeselect'

interface Props {
  modelValue: any
  treeProps?: TreeSelectProps
  transform?: (value: any) => any
}

const {
  modelValue: value,
  treeProps,
  transform = v => v,
} = defineProps<Props>()

const emit = defineEmits<{ 'update:modelValue': [any] }>()

// TressSelect state is {[key: string]: boolean}
const _value = ref<any>()

function onChange(checked: any) {
  const mode = treeProps?.selectionMode || 'single'

  const result = Object.keys(checked).map(transform)

  emit('update:modelValue', mode === 'single' ? result[0] : result)
}

watchEffect(() => {
  const list = Array.isArray(value) ? value : value ? [value] : []

  _value.value = list.reduce((acc: any, cur: any) => {
    acc[cur] = true
    return acc
  }, {})
})
</script>

<template>
  <TreeSelect
    v-bind="treeProps"
    :model-value="_value"
    class="w-full"
    @change="onChange"
  />
</template>
