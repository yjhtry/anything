<script setup lang="ts" generic="T extends any, O extends any">
import { useToast } from 'primevue/usetoast'

defineOptions({
  name: 'IndexPage',
})

const toast = useToast()

const { data, loading, error } = useInvoke(getPackages({}))

const total = computed(() => data.value?.total || 0)
const dataSource = computed(() => data.value?.data || [])

watchEffect(() => {
  if (error.value)
    toast.add({ severity: 'error', summary: 'Error', detail: error.value })
})
</script>

<template>
  <Toast position="bottom-right" />
  <div class="mb-4 border-none bg-[#18181b] p-4">
    <PkgTable :loading="loading" :total="total" :data-source="dataSource" />
  </div>
</template>
