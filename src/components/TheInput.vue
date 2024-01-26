<script setup lang="ts">
const modelValue = defineModel<string>()
</script>

<template>
  <InputText
    v-model="modelValue"
    type="text"
    v-bind="$attrs"
  />
</template>
