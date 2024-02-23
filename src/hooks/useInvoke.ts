import type { MaybeComputedRef } from '@vueuse/head'
import type { Apis } from '~/utils/invoke'

export function useInvoke<T, U = any>(api: MaybeComputedRef<Apis> | Promise<T>, params?: U) {
  const loading = ref(false)
  const data = shallowRef<T>()
  const error = ref('')

  const toFetch = () => {
    const callApi = toValue(api)

    loading.value = true
    let result

    if (callApi instanceof Promise)
      result = callApi
    else
      result = invoke(callApi, params)

    result
      .then((res) => {
        data.value = res as T
      })
      .catch((err) => {
        error.value = err
      })
      .finally(() => loading.value = false)
  }

  watchEffect(() => {
    toFetch()
  })

  return { loading, data, error }
}
