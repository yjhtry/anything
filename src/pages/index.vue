<script setup lang="ts" generic="T extends any, O extends any">
import type { QueryPkgsParams } from '~/services/pkg'

defineOptions({
  name: 'IndexPage',
})

const toast = useToast()
const query = shallowRef<QueryPkgsParams>({})

const { data, loading, error } = useInvoke(getPackages, query)

const total = computed(() => data.value?.total || 0)
const dataSource = computed(() => data.value?.data || [])

function onSearch(data: QueryPkgsParams) {
  query.value = data
}

watchEffect(() => {
  if (error.value)
    toast.add({ severity: 'error', summary: 'Error', detail: error.value })
})
</script>

<template>
  <Toast position="bottom-right" />
  <div class="mt-4 px-3">
    <div class="rounded-md border-none bg-[#18181b] p-2">
      <PkgQuery class="mb-5" :on-search="onSearch" />
      <PkgTable :loading="loading" :total="total" :data-source="dataSource" />
    </div>
  </div>
</template>
