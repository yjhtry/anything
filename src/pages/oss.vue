<script setup lang="ts">
import { writeImageBinary, writeText } from 'tauri-plugin-clipboard-api'

const confirm = useConfirm()
const toast = useToast()

const { data, reload } = useInvoke(getOssTree, {})

// sort by key , image -> video -> audio -> other

const sortedData = computed(() => {
  if (!data?.value)
    return []

  const keys = Object.keys(data.value)
  keys.sort((a, b) => {
    if (a === 'image')
      return -1
    if (b === 'image')
      return 1
    if (a === 'video')
      return -1
    if (b === 'video')
      return 1
    if (a === 'audio')
      return -1
    if (b === 'audio')
      return 1
    return 0
  })

  return keys.map(key => ({ data: data.value![key], kind: key }))
})

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

async function removeFile(event: any, path: string) {
  confirm.require({
    target: event.currentTarget,
    message: 'Do you want to delete this file?',
    icon: 'pi pi-info-circle',
    rejectClass: 'p-button-secondary p-button-outlined p-button-sm',
    acceptClass: 'p-button-danger p-button-sm ml-3',
    rejectLabel: 'Cancel',
    acceptLabel: 'Delete',
    accept: async () => {
      try {
        await invoke('remove_file_from_oss', { path })

        toast.add({ severity: 'success', summary: 'Success', detail: 'Remove file success!', life: 3000 })

        reload()
      }
      catch (e) {
        toast.add({ severity: 'warn', summary: 'Error', detail: e, life: 3000 })
      }
    },
  })
}

onMounted(() => {
  getAppSettings().then(log)
})
</script>

<template>
  <Toast position="top-center" />
  <ConfirmPopup />
  <div m-3>
    <TabView>
      <TabPanel v-for="(data, index) in sortedData" :key="index" :header="data.kind as string">
        <DataView :value="data.data" data-key="1" paginator :rows="5">
          <template #header>
            <div class="flex justify-end">
              <Button @click="addFile">
                Add to OSS
              </Button>
            </div>
          </template>
          <template #list="{ items }">
            <div
              v-for="(item, index) in items" :key="item" class="flex justify-between p-4"
              :class="{ 'bd-t': index !== 0 }"
            >
              <div class="flex gap-6">
                <template v-if="data.kind === 'image'">
                  <Image :src="item.url" :alt="item.name" width="160" preview />
                </template>
                <template v-else-if="data.kind === 'video'">
                  <video controls width="250">
                    <source :src="item.url" type="video/webm">
                  </video>
                </template>
                <template v-else-if="data.kind === 'audio'">
                  <audio width="220" controls :src="item.url" />
                </template>
                <div class="flex flex-col justify-center gap-8">
                  <div>{{ item.name }}</div>
                  <div>{{ beatifyBytes(item.size) }}</div>
                </div>
              </div>
              <div class="flex items-center gap-3">
                <Button icon="pi pi-copy" @click="copyUrl(item.url)" />
                <Button v-if="data.kind === 'image'" icon="pi pi-download" @click="downloadFile(item.url)" />
                <Button icon="pi pi-trash" @click="removeFile($event, item.path)" />
              </div>
            </div>
          </template>
        </DataView>
      </TabPanel>
    </TabView>
  </div>
</template>
