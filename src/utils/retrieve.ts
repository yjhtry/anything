import type { Category } from '~/services/pkg'

function transformCate(node: Category) {
  return {
    label: node.name,
    data: node.id,
    key: node.id,
  }
}

export function retrieveCateTreeData(list: Category[]) {
  const map = new Map()
  const result = [] as any[]

  list.forEach((item) => {
    map.set(item.id, { ...transformCate(item), children: [] })
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
}
