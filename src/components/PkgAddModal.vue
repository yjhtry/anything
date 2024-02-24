<script setup lang="ts">
import { useForm } from 'vee-validate'
import { array, number, object, string } from 'yup'
import { useToast } from 'primevue/usetoast'
import type { PackageWithoutDefault } from '~/services/pkg'

const toast = useToast()

const visible = ref(false)
const catesOptions = useCatesTree()

const validationSchema = toTypedSchema(object({
  name: string().max(36).trim().required('Name is required'),
  description: string().trim().required('Description is required'),
  reason: string().trim().required('Reason is required'),
  link: string().trim().url().required('Link is required'),
  categories: array().of(number()),
}))

const { handleSubmit, resetForm } = useForm<PackageWithoutDefault>({
  validationSchema,
})

function onClose() {
  visible.value = false
  resetForm()
}

const onSubmit = handleSubmit(async (values) => {
  try {
    await addPackage(values)
  }
  catch (error) {
    toast.add({ severity: 'error', summary: 'Error', detail: (error as any).message })
  }
})
</script>

<template>
  <Button label="Show" @click="visible = true" />
  <Dialog v-model:visible="visible" modal header="Edit Profile" :style="{ width: '25rem' }">
    <div class="mt-8 space-y-8">
      <div class="align-items-center mb-3 flex gap-3">
        <TheInput name="name" label="Name" w-67 />
      </div>
      <div class="align-items-center mb-3 flex gap-3">
        <TheInput name="description" label="Description" w-67 />
      </div>
      <div class="align-items-center mb-3 flex gap-3">
        <TheInput name="reason" label="Reason" w-67 />
      </div>
      <div class="align-items-center mb-3 flex gap-3">
        <TheInput name="link" label="Link" w-67 />
      </div>
      <div class="align-items-center mb-3 flex gap-3">
        <TheTreeSelect
          w-67
          name="categories" label="Categories"
          :tree-props="{ options: catesOptions, selectionMode: 'multiple' }"
        />
      </div>
    </div>
    <div class="mt-8 flex justify-end gap-2">
      <Button type="button" label="Cancel" severity="secondary" @click="onClose" />
      <Button type="button" label="Save" @click="onSubmit" />
    </div>
  </Dialog>
</template>
