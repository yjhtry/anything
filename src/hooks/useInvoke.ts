import type { MaybeComputedRef } from '@vueuse/head'
import type { MaybeRef } from 'vue'
import type { Apis } from '~/utils/invoke'

type Callable<T> = (...args: any[]) => Promise<T>

export function useInvoke<T, U = any>(api: MaybeRef<Apis> | Callable<T>, params?: MaybeComputedRef<U>) {
  const loading = ref(false)
  const data = shallowRef<T>()
  const error = ref('')

  const toFetch = () => {
    const payload = toValue(params)

    loading.value = true
    let result: Promise<T>

    if (typeof api === 'function')
      result = api(payload)
    else
      result = invoke<T>(toValue(api), payload)

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
