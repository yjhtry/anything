<script setup lang="ts">
import type { TreeSelectProps } from 'primevue/treeselect'
import { useField } from 'vee-validate'

interface Props {
  name: string
  label?: string
  treeProps?: TreeSelectProps
}

const {
  name,
  label,
  treeProps,
} = defineProps<Props>()

const _value = ref<any>()

const { value, handleChange, errorMessage } = useField<any>(() => name, undefined, {
  validateOnValueUpdate: false,
})

function onChange(node: any, select: boolean) {
  let mode = 'single'
  if (treeProps?.selectionMode)
    mode = treeProps.selectionMode

  if (mode === 'single') {
    handleChange(select ? node.data : undefined)

    return
  }

  const result = [...value.value || []]

  if (select) {
    result.push(node.data)
  }
  else {
    const index = result.indexOf(node.data)
    if (index > -1)
      result.splice(index, 1)
  }

  handleChange(result)
}
</script>

<template>
  <FloatLabel>
    <TreeSelect
      v-bind="treeProps"
      v-model="_value"
      class="w-full"
      :invalid="!!errorMessage"
      @node-select="(node) => onChange(node, true)"
      @node-unselect="(node) => onChange(node, false)"
    />
    <label>{{ label }}</label>
  </FloatLabel>
</template>
