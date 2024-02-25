<script setup lang="ts" generic="T extends any, O extends any">
import type { QueryCatesParams } from '~/services/pkg'

const toast = useToast()
const query = shallowRef<QueryCatesParams>({})

const { data, loading, error, reload } = useInvoke(getCategories, query)

const total = computed(() => data.value?.total || 0)
const dataSource = computed(() => data.value?.data || [])

function onSearch(data: QueryCatesParams) {
  query.value = data
}

watchEffect(() => {
  if (error.value)
    toast.add({ severity: 'error', summary: 'Error', detail: error.value, life: 5000 })
})
</script>

<template>
  <Toast position="top-right" />
  <div class="mt-4 px-3">
    <div class="rounded-md border-none bg-[#18181b] p-2">
      <CateQuery class="mb-5" @search="onSearch" />
      <CateTable :loading="loading" :total="total" :data-source="dataSource" @reload="reload" />
    </div>
  </div>
</template>
