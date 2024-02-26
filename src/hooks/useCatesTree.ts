import type { QueryCatesParams } from '~/services/pkg'

export function useCatesTree(query: QueryCatesParams = { page: 1, page_size: 1000 }) {
  const { data } = useInvoke(getCategories, query)

  return computed(() => {
    const { data: list = [] } = data.value || {}

    return retrieveCateTreeData(list)
  })
}
