<script setup lang="ts">
import type { Package } from '~/services/pkg'

const { loading, dataSource, total } = defineProps<{
  loading: boolean
  dataSource: Package[]
  total: number
}>()

const emit = defineEmits<{
  reload: []
  pageChange: [{ page: number, page_size: number }]
}>()

const syncLoading = ref(false)
const confirm = useConfirm()
const toast = useToast()
const router = useRouter()
const { data: categories } = useInvoke(getCategories, { page: 1, page_size: 1000 })

const cateMap = computed(() => {
  if (categories.value) {
    return categories.value.data.reduce((acc: Record<number, string>, cur) => {
      acc[cur.id] = cur.name
      return acc
    }, {})
  }

  return {}
})

const cateTreeOptions = computed(() => {
  if (categories.value)
    return retrieveCateTreeData(categories.value.data)

  return []
})

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
        await deletePackage(id)
        emit('reload')
        toast.add({
          severity: 'success',
          summary: 'Confirmed',
          detail: 'Record deleted',
          life: 3000,
        })
      }
      catch (error) {
        toast.add({
          severity: 'error',
          summary: 'Error',
          detail: 'Failed to delete record',
          life: 5000,
        })
      }
    },
  })
}

function openCatePage() {
  router.push('/category')
}

async function syncLocalData2Pg() {
  try {
    syncLoading.value = true
    await invoke('sync_data_to_postgres', {})

    toast.add({ severity: 'success', summary: 'Success', detail: 'Sync success!', life: 3000 })
  }
  catch (error) {
    toast.add({ severity: 'error', summary: 'Error', detail: error, life: 5000 })
  }
  finally {
    syncLoading.value = false
  }
}

async function onCellEditComplete(event: any) {
  try {
    const { data, newValue = [], value = [] } = event
    if (newValue.sort().toString() === value.sort().toString())
      return

    await updatePkgCates({ id: data.id, categories: newValue })

    emit('reload')

    toast.add({ severity: 'success', summary: 'Success', detail: 'Update success!', life: 3000 })
  }
  catch (error) {
    toast.add({ severity: 'error', summary: 'Error', detail: error, life: 5000 })
  }
}
</script>

<template>
  <ConfirmPopup />
  <DataTable
    scroll-height="58vh"
    scrollable
    paginator
    data-key="id"
    :loading="loading"
    :value="dataSource"
    :total-records="total"
    :rows="10"
    :rows-per-page-options="[5, 10, 20, 50]"
    class="text-3"
    edit-mode="cell"
    @page="emit('pageChange', { page: $event.page + 1, page_size: $event.rows })"
    @cell-edit-complete="onCellEditComplete"
  >
    <template #header>
      <div class="flex justify-end gap-3">
        <Button
          v-tooltip.top="{
            value: 'You should set env variable `POSTGRESQL_URL`',
            showDelay: 100,
            hideDelay: 300,
            class: 'min-w-100',
          }"
          label="Sync" :loading="syncLoading"
          @click="syncLocalData2Pg"
        />
        <Button label="Category" severity="secondary" @click="openCatePage" />
        <PkgAddOrUpdateModal @reload="emit('reload')" />
      </div>
    </template>
    <template #empty>
      <div text="center" class="py-4">
        No packages found
      </div>
    </template>
    <Column field="name" header="Name" class="min-w-60" />
    <Column field="description" header="Description" class="min-w-80">
      <template #body="{ data }">
        <BtnView :btn-props="{ label: data.description, text: true }">
          {{ data.description }}
        </BtnView>
      </template>
    </Column>
    <Column field="reason" header="Reason" class="min-w-80">
      <template #body="{ data }">
        <BtnView :btn-props="{ label: data.reason, text: true }">
          {{ data.reason }}
        </BtnView>
      </template>
    </Column>
    <Column field="categories" header="Categories" class="min-w-60">
      <template #body="{ data }">
        <Tag v-for="cateId in data.categories" :key="cateId" ml-2>
          {{ cateMap[cateId] }}
        </Tag>
      </template>
      <template #editor="{ data, field }">
        <HumanTreeSelect
          v-model="data[field]"
          :transform="Number"
          :tree-props="{ options: cateTreeOptions, selectionMode: 'multiple' }"
        />
      </template>
    </Column>
    <Column
      column-key="operation"
      header="Operation"
      class="min-w-60"
      frozen
      align-frozen="right"
    >
      <template #body="{ data }">
        <a :href="data.link" target="_blank">
          <Button label="open" text class="px-2" />
        </a>
        <PkgAddOrUpdateModal mode="edit" :row="data" @reload="emit('reload')" />
        <Button
          label="del" text class="px-2"
          @click="onDelete($event, data.id)"
        />
      </template>
    </Column>
  </DataTable>
</template>
