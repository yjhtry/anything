function transform(node: any) {
  return {
    label: node.name,
    data: node.id,
    key: node.id,
  }
}

export function useCatesTree() {
  const { data } = useInvoke(getCategories, {})

  return computed(() => {
    const { data: list = [] } = data.value || {}

    const map = new Map()
    const result = [] as any[]

    list.forEach((item) => {
      map.set(item.id, { ...transform(item), children: [] })
    })

    list.forEach((item) => {
      if (item.parent_id === 0) {
        result.push(map.get(item.id))
      }
      else {
        const parent = map.get(item.parent_id)
        if (parent)
          parent.children.push(map.get(item.id))
      }
    })

    return result
  })
}
