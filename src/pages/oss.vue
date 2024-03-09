<script setup lang="ts">
const toast = useToast()

const { data } = useInvoke(getOssTree, {})

async function openFile() {
  try {
    await invoke('move_file_to_oss', {})
    toast.add({ severity: 'success', summary: 'Success', detail: 'Move file success!', life: 3000 })
  }
  catch (e) {
    toast.add({ severity: 'warn', summary: 'Error', detail: e, life: 3000 })
  }
}

onMounted(() => {
  getAppSettings().then(log)
})
</script>

<template>
  <Toast />
  <div m-3>
    <TabView>
      <TabPanel v-for="(list, kind) in data" :key="kind" :header="kind as string">
        <DataView :value="list" data-key="1" paginator :rows="5">
          <template #header>
            <div class="flex justify-end">
              <Button @click="openFile">
                Open
              </Button>
            </div>
          </template>
          <template #list="{ items }">
            <template v-if="kind === 'image'">
              <div
                v-for="(item, index) in items" :key="item" class="flex justify-between p-4"
                :class="{ 'bd-t': index !== 0 }"
              >
                <div class="flex gap-6">
                  <Image :src="item.url" :alt="item.name" width="160" preview />
                  <div class="flex flex-col justify-center gap-8">
                    <div>{{ item.name }}</div>
                    <div>{{ item.size }}</div>
                  </div>
                </div>
                <div class="flex items-center gap-3">
                  <Button icon="pi pi-copy" />
                  <Button icon="pi pi-download" />
                  <Button icon="pi pi-trash" />
                </div>
              </div>
            </template>
          </template>
        </DataView>
      </TabPanel>
    </TabView>
  </div>
</template>
