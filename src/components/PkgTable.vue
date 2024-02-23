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
    <Column field="name" header="Name" style="width: 25%" />
    <Column field="description" header="Description" style="width: 25%" />
    <Column field="reason" header="Reason" style="width: 25%" />
    <Column field="link" header="Link" style="width: 25%">
      <template #body="{ data }">
        <a :href="data.link" target="_blank">
          <Button label="Open" link />
        </a>
      </template>
    </Column>
    <Column column-key="operation" header="Operation" style="width: 25%">
      <template #body="">
        <Button label="Edit" />
      </template>
    </Column>
  </DataTable>
</template>
