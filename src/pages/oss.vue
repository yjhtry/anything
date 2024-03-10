<script setup lang="ts">
import { writeImageBinary, writeText } from 'tauri-plugin-clipboard-api'

const toast = useToast()

const { data, reload } = useInvoke(getOssTree, {})

async function addFile() {
  try {
    await invoke('move_file_to_oss', {})
    toast.add({ severity: 'success', summary: 'Success', detail: 'Move file success!', life: 3000 })

    reload()
  }
  catch (e) {
    toast.add({ severity: 'warn', summary: 'Error', detail: e, life: 3000 })
  }
}

async function copyUrl(url: string) {
  try {
    await writeText(url)
    toast.add({ severity: 'success', summary: 'Success', detail: 'Copy url success!', life: 3000 })
  }
  catch (e) {
    toast.add({ severity: 'warn', summary: 'Error', detail: e, life: 3000 })
  }
}

async function downloadFile(url: string) {
  const successToast = { severity: 'info', summary: 'waiting', detail: 'downloading', closable: false } as any
  try {
    toast.add(successToast)

    const response = await fetch(url)
    const blob = await response.blob()
    const buffer = await blob.arrayBuffer()
    const bytes = Array.from(new Uint8Array(buffer))
    await writeImageBinary(bytes)
  }
  catch (e) {
    toast.add({ severity: 'warn', summary: 'Error', detail: e, life: 3000 })
  }
  finally {
    toast.remove(successToast)
  }
}

async function removeFile(path: string) {
  try {
    log(path)
    await invoke('remove_file_from_oss', { path })

    toast.add({ severity: 'success', summary: 'Success', detail: 'Remove file success!', life: 3000 })

    reload()
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
  <Toast position="top-center" />
  <div m-3>
    <TabView>
      <TabPanel v-for="(list, kind) in data" :key="kind" :header="kind as string">
        <DataView :value="list" data-key="1" paginator :rows="5">
          <template #header>
            <div class="flex justify-end">
              <Button @click="addFile">
                Add to OSS
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
                    <div>{{ beatifyBytes(item.size) }}</div>
                  </div>
                </div>
                <div class="flex items-center gap-3">
                  <Button icon="pi pi-copy" @click="copyUrl(item.url)" />
                  <Button icon="pi pi-download" @click="downloadFile(item.url)" />
                  <Button icon="pi pi-trash" @click="removeFile(item.path)" />
                </div>
              </div>
            </template>
          </template>
        </DataView>
      </TabPanel>
    </TabView>
  </div>
</template>
