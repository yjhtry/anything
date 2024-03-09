<script setup lang="ts">
const toast = useToast()

const { data } = useInvoke(getOssTree, {})

async function openFile() {
  try {
    await invoke('move_file_to_oss', {})
    toast.add({ severity: 'success', summary: 'Success', detail: 'Move file success!', life: 3000 })
  }
  catch (e) {
    toast.add({ severity: 'error', summary: 'Error', detail: e, life: 5000 })
  }
}

onMounted(() => {
  getAppSettings().then(log)
})
</script>

<template>
  <Toast />
  <div>
    <Button @click="openFile">
      Open
    </Button>
    <Button @click="getOssTree">
      fetch tree
    </Button>
    <TabView>
      <TabPanel v-for="(list, kind) in data" :key="kind" :header="kind as string">
        <DataView :value="list" data-key="1" paginator :rows="5">
          <template #header>
            header
          </template>
          <template #list="{ items }">
            <div v-for="item in items" :key="item">
              {{ item }}
            </div>
          </template>
        </DataView>
      </TabPanel>
    </TabView>
  </div>
</template>
