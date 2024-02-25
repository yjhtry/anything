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

const isSubmitting = useIsSubmitting()
const { value, errorMessage } = useField<any>(() => name, undefined, {
  validateOnValueUpdate: false,
})
</script>

<template>
  <FloatLabel>
    <HumanTreeSelect
      v-model="value"
      :transform="transform"
      :tree-props="{ ...controlProps, disabled: isSubmitting, invalid: !!errorMessage }"
    />
    <label>{{ label }}</label>
  </FloatLabel>
</template>
