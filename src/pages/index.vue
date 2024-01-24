<script setup lang="ts" generic="T extends any, O extends any">
defineOptions({
  name: 'IndexPage',
})

const name = ref('')
</script>

<template>
  <div>
    <TheInput
      v-model="name"
      placeholder="What's your name?"
      autocomplete="false"
    />
  </div>
</template>
