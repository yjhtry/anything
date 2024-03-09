export interface Settings {
  [key: string]: string[]
}
export function getAppSettings() {
  return invoke<Settings>('get_app_settings', {})
}
