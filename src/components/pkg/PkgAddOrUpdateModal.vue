<script setup lang="ts">
import { array, number, object, string } from 'yup'
import type { Package, PackageWithoutDefault } from '~/services/pkg'

type State = PackageWithoutDefault

const { mode = 'add', row } = defineProps<{
  mode?: 'add' | 'edit'
  row?: Package
}>()

const emit = defineEmits<{
  reload: []
}>()

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

const { handleSubmit, resetForm, setValues } = useForm<State>({
  validationSchema,
})

function onClose() {
  visible.value = false
  resetForm()
}

const onSubmit = handleSubmit(async (values) => {
  try {
    if (mode === 'add') {
      await addPackage({ ...values })
      toast.add({
        severity: 'success',
        summary: 'Success',
        detail: 'Add success!',
        life: 3000,
      })
    }
    else {
      await updatePackage({ ...values, id: row!.id })
      toast.add({
        severity: 'success',
        summary: 'Success',
        detail: 'Update success!',
        life: 3000,
      })
    }

    emit('reload')

    onClose()
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

watchEffect(() => {
  if (mode === 'edit' && row)
    setValues({ ...row })
})
</script>

<template>
  <Button v-if="mode === 'add'" label="Add" @click="visible = true" />
  <Button v-else label="edit" text class="px-2" @click="visible = true" />
  <Dialog v-model:visible="visible" modal header="Edit Profile" w-120>
    <div class="mt-8 space-y-8">
      <div class="align-items-center mb-3 flex gap-3">
        <TheInput name="name" label="Name" w-100 />
      </div>
      <div class="align-items-center mb-3 flex gap-3">
        <TheTextarea
          name="description" label="Description" w-100
          :control-props="{ autoResize: true, rows: 3 }"
        />
      </div>
      <div class="align-items-center mb-3 flex gap-3">
        <TheTextarea
          name="reason" label="Reason" w-100
          :control-props="{ autocomplete: 'false', autoResize: true, rows: 3 }"
        />
      </div>
      <div class="align-items-center mb-3 flex gap-3">
        <TheInput
          name="link" label="Link" w-100
          :control-props="{ autocomplete: 'false' }"
        />
      </div>
      <div v-if="mode === 'add'" class="align-items-center mb-3 flex gap-3">
        <TheTreeSelect
          w-100
          name="categories" label="Categories"
          :transform="Number"
          :control-props="{ options: catesOptions, selectionMode: 'multiple' }"
        />
      </div>
    </div>
    <div class="mt-8 flex justify-end gap-2">
      <Button type="button" label="Cancel" severity="secondary" @click="onClose" />
      <Button type="button" label="Save" @click="onSubmit" />
    </div>
  </Dialog>
</template>
