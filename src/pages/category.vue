<script setup lang="ts" generic="T extends any, O extends any">
import type { QueryCatesParams } from '~/services/pkg'

const toast = useToast()
const query = shallowRef<QueryCatesParams>({ page: 1, page_size: 10 })

const { data, loading, error, reload } = useInvoke(getCategories, query)

const total = computed(() => data.value?.total || 0)
const dataSource = computed(() => data.value?.data || [])

function onSearch(data: QueryCatesParams) {
  query.value = { ...query.value, ...data }
}

watchEffect(() => {
  if (error.value)
    toast.add({ severity: 'error', summary: 'Error', detail: error.value, life: 5000 })
})
</script>

<template>
  <Toast position="top-right" />
  <div class="mt-4 px-3">
    <div border="~ ##e2e8f0 dark:transparent" class="rounded-md bg-white p-2 dark:bg-[#18181b]">
      <CateQuery class="mb-5" @search="onSearch" />
      <CateTable
        :loading="loading"
        :total="total"
        :data-source="dataSource"
        @reload="reload"
        @page-change="onSearch"
      />
    </div>
  </div>
</template>
