<script setup lang="ts">
import { useForm } from 'vee-validate'
import type { QueryPkgsParams } from '~/services/pkg'

const { onSearch } = defineProps<{
  onSearch: (query: QueryPkgsParams) => void
}>()

const isLoading = ref(false)

const { handleSubmit, resetForm } = useForm<QueryPkgsParams>({ })

const onSubmit = handleSubmit(async (values) => {
  try {
    isLoading.value = true
    onSearch(values)
  }
  catch (error) {

  }
  finally {
    isLoading.value = false
  }
})

function onReset() {
  resetForm()
  onSearch({})
}

const [DefineInput, ReuseInput] = createReusableTemplate<{ name: string, label: string, placeholder?: string }>()
</script>

<template>
  <DefineInput v-slot="{ name, label }">
    <TheInput
      :name="name" :label="label" :disabled="isLoading"
    />
  </DefineInput>
  <div v-bind="$attrs" px-2 pt-4>
    <form @submit="onSubmit">
      <div class="flex flex-wrap gap-8">
        <ReuseInput name="name" label="Name" />
        <ReuseInput name="description" label="Description" />
        <ReuseInput name="reason" label="Reason" />
      </div>
      <div class="mt-4 flex justify-end gap-4">
        <Button severity="secondary" :loading="isLoading" @click="onReset">
          Reset
        </Button>
        <Button type="submit" :loading="isLoading">
          Search
        </Button>
      </div>
    </form>
  </div>
</template>
