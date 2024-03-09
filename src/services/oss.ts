export interface OssTree {
  [key: string]: string[]
}
export function getOssTree() {
  return invoke<OssTree>('get_oss_tree', {})
}
