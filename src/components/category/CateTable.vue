<script setup lang="ts">
import type { Category } from '~/services/pkg'

const { loading, dataSource, total } = defineProps<{
  loading: boolean
  dataSource: Category[]
  total: number
}>()

const confirm = useConfirm()
const toast = useToast()

function onDelete(event: any, id: number) {
  confirm.require({
    target: event.currentTarget,
    message: 'Do you want to delete this record?',
    icon: 'pi pi-info-circle',
    rejectClass: 'p-button-secondary p-button-outlined p-button-sm',
    acceptClass: 'p-button-danger p-button-sm ml-3',
    rejectLabel: 'Cancel',
    acceptLabel: 'Delete',
    accept: async () => {
      try {
        await deleteCategory(id)
        toast.add({ severity: 'success', summary: 'Confirmed', detail: 'Record deleted' })
      }
      catch (error) {
        toast.add({ severity: 'error', summary: 'Error', detail: error })
      }
    },
  })
}
</script>

<template>
  <ConfirmPopup />
  <DataTable
    paginator
    :loading="loading"
    :value="dataSource"
    :total-records="total"
    :rows="10"
    :rows-per-page-options="[5, 10, 20, 50]"
    class="w-full"
  >
    <template #header>
      <div class="flex justify-end">
        <CateAddOrUpdateModal />
      </div>
    </template>
    <template #empty>
      <div text="center" class="py-4">
        No packages found
      </div>
    </template>
    <Column field="name" header="Name" style="width: 25%" />
    <Column field="id" header="id" style="width: 25%" />
    <Column field="parent_id" header="parent_id" style="width: 25%" />
    <Column column-key="operation" header="Operation" style="width: 25%">
      <template #body="{ data }">
        <CateAddOrUpdateModal mode="edit" :row="data" />
        <Button
          label="del" text class="px-2"
          @click="onDelete($event, data.id)"
        />
      </template>
    </Column>
  </DataTable>
</template>
