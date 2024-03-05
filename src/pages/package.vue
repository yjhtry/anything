<script setup lang="ts" generic="T extends any, O extends any">
import type { QueryPkgsParams } from '~/services/pkg'

defineOptions({
  name: 'IndexPage',
})

const toast = useToast()
const query = shallowRef<QueryPkgsParams>({ page: 1, page_size: 10 })

const { data, loading, error, reload } = useInvoke(getPackages, query)

const total = computed(() => data.value?.total || 0)
const dataSource = computed(() => data.value?.data || [])

function onSearch(data: QueryPkgsParams) {
  const { page, page_size } = query.value

  query.value = { page, page_size, ...data }
}

watchEffect(() => {
  if (error.value)
    toast.add({ severity: 'error', summary: 'Error', detail: error.value, life: 5000 })
})
</script>

<template>
  <Toast position="top-right" />
  <div class="mt-4 px-3">
    <div class="border-1 border-[#e2e8f0] rounded-md border-solid bg-white p-2 dark:border-transparent dark:bg-[#18181b]">
      <PkgQuery class="mb-5" @search="onSearch" />
      <PkgTable :loading="loading" :total="total" :data-source="dataSource" @page-change="onSearch" @reload="reload" />
    </div>
  </div>
</template>
