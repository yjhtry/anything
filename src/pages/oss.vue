<script setup lang="ts">
async function openFile() {
  const res = await invoke('move_file_to_oss', {})

  log(res)
}
</script>

<template>
  <div>
    <Button @click="openFile">
      Open
    </Button>
  </div>
</template>
