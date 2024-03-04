<script setup lang="ts">
const route = useRoute()
const router = useRouter()

watchEffect(() => {
  if (route.path === '/')
    router.push('/package')
})
</script>

<template>
  <div>
    <slot />
  </div>
</template>
