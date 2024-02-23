import { invoke as _invoke } from '@tauri-apps/api'

export type Apis = 'add_package'
  | 'update_package'
  | 'delete_package'
  | 'query_packages'
  | 'get_package_by_id'
  | 'add_category'
  | 'update_category'
  | 'delete_category'
  | 'query_categories'

export function invoke<T>(api: Apis, data: any) {
  return _invoke<T>(api, data)
}
