<script setup lang="ts">
import type { Package } from '../services/pkg'

const { loading, dataSource, total } = defineProps<{
  loading: boolean
  dataSource: Package[]
  total: number
}>()
</script>

<template>
  <DataTable
    paginator
    :loading="loading"
    :value="dataSource"
    :total-records="total"
    :rows="10"
    :rows-per-page-options="[5, 10, 20, 50]"
    table-style="min-width: 50rem"
  >
    <template #empty>
      <div text="center" class="py-4">
        No packages found
      </div>
    </template>
    <Column field="name" header="Name" style="width: 25%" />
    <Column field="description" header="Description" style="width: 25%" />
    <Column field="reason" header="Reason" style="width: 25%" />
    <Column column-key="operation" header="Operation" style="width: 25%">
      <template #body="{ data }">
        <a :href="data.link" target="_blank">
          <Button label="Open" text />
        </a>
        <Button label="Edit" text />
      </template>
    </Column>
  </DataTable>
</template>
