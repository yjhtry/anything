export function useCatesTree() {
  const { data } = useInvoke(getCategories, {})

  return computed(() => {
    const { data: list = [] } = data.value || {}

    return retrieveCateTreeData(list)
  })
}
