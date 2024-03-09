import { invoke as _invoke } from '@tauri-apps/api'

export type Apis = 'add_package'
  | 'update_package'
  | 'delete_package'
  | 'query_packages'
  | 'update_package_categories'
  | 'get_package_by_id'
  | 'add_category'
  | 'update_category'
  | 'delete_category'
  | 'query_categories'
  | 'sync_data_to_postgres'
  | 'move_file_to_oss'
  | 'get_oss_tree'

export function invoke<T>(api: Apis, data: any) {
  return _invoke<T>(api, data)
}
