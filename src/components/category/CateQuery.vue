<script setup lang="ts">
import type { QueryCatesParams } from '~/services/pkg'

type State = Omit<QueryCatesParams, 'page' | 'page_size'>

const { onSearch } = defineProps<{
  onSearch: (query: QueryCatesParams) => void
}>()

const toast = useToast()
const { handleSubmit, resetForm, isSubmitting } = useForm<State>()

const onSubmit = handleSubmit(async (values) => {
  try {
    onSearch(values)
  }
  catch (error) {
    toast.add({ severity: 'error', summary: 'Error', detail: error })
  }
})

function onReset() {
  resetForm()
  onSearch({})
}
</script>

<template>
  <div v-bind="$attrs" px-2 pt-4>
    <form @submit="onSubmit">
      <div class="flex flex-wrap gap-8">
        <TheInput name="name" label="Name" class="inline-block w-67" />
        <TheNumber name="parent_id" label="ParentId" class="inline-block w-67" />
      </div>
      <div class="mt-4 flex justify-end gap-4">
        <Button severity="secondary" :loading="isSubmitting" @click="onReset">
          Reset
        </Button>
        <Button type="submit" :loading="isSubmitting">
          Search
        </Button>
      </div>
    </form>
  </div>
</template>
