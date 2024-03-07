<script setup lang="ts">
import type { QueryPkgsParams } from '~/services/pkg'

type State = Omit<QueryPkgsParams, 'page' | 'page_size'>

const emit = defineEmits<{ search: [QueryPkgsParams, boolean?] }>()

const toast = useToast()
const catesOptions = useCatesTree()
const { handleSubmit, resetForm, isSubmitting } = useForm<State>()

const onSubmit = handleSubmit(async (values) => {
  try {
    emit('search', values)
  }
  catch (error) {
    toast.add({
      severity: 'error',
      summary: 'Error',
      detail: error,
      life: 5000,
    })
  }
})

function onReset() {
  resetForm()
  emit('search', {}, true)
}
</script>

<template>
  <div v-bind="$attrs" px-2 pt-4>
    <form @submit="onSubmit">
      <div class="flex flex-wrap gap-8">
        <TheInput name="name" label="Name" class="inline-block w-67" />
        <TheInput name="description" label="Description" class="inline-block w-67" />
        <TheInput name="reason" label="Reason" class="inline-block w-67" />
        <TheTreeSelect
          name="categories"
          label="Categories"
          class="inline-block w-67"
          :transform="Number"
          :control-props="{ options: catesOptions, selectionMode: 'multiple' }"
        />
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
