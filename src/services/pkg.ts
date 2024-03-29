export interface Package {
  id: number
  name: string
  description: string
  reason: string
  link: string
  categories: number[]
  update_at: string
  create_at: string
}

type OmitDefault<T> = Omit<T, 'id' | 'update_at' | 'create_at'>

export type PackageWithoutDefault = OmitDefault<Package>

export function addPackage(data: PackageWithoutDefault) {
  return invoke<{ id: number }>('add_package', { data })
}

export function updatePackage(data: Omit<Package, 'create_at' | 'update_at'>) {
  return invoke<{ id: number }>('update_package', { data })
}

export function updatePkgCates(data: { id: number, categories: number[] }) {
  return invoke<{ id: number }>('update_package_categories', { data })
}

export function deletePackage(id: number) {
  return invoke<undefined>('delete_package', { id })
}

export interface QueryPkgsParams {
  name?: string
  description?: string
  reason?: string
  categories?: number[]
  page?: number
  page_size?: number
}

export interface QueryPkgsRes {
  total: number
  data: Package[]
}

export function getPackages(data: QueryPkgsParams = {}) {
  return invoke<QueryPkgsRes>('query_packages', { data })
}

export function getPackageById(id: number) {
  return invoke<Package>('get_package_by_id', { id })
}

export interface Category {
  id: number
  name: string
  parent_id: number
  create_at: string
  update_at: string
}

export type CategoryWithoutDefault = OmitDefault<Category>

export function addCategory(data: CategoryWithoutDefault) {
  return invoke<{ id: number }>('add_category', { data })
}

export function updateCategory(data: Omit<Category, 'create_at' | 'update_at'>) {
  return invoke<{ id: number }>('update_category', { data })
}

export function deleteCategory(id: number) {
  return invoke<undefined>('delete_category', { id })
}

export interface QueryCatesParams {
  name?: string
  parent_id?: number
  page?: number
  page_size?: number
}

export interface QueryCatesRes {
  total: number
  data: Category[]
}

export function getCategories(data: QueryCatesParams = {}) {
  return invoke<QueryCatesRes>('query_categories', { data })
}
