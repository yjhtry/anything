import type { App } from 'vue'

import 'primevue/resources/themes/lara-dark-teal/theme.css'

import ToastService from 'primevue/toastservice'

import Toast from 'primevue/toast'
import PrimeVue from 'primevue/config'

// Most of components will be imported automatically in Vite
// see https://github.com/unplugin/unplugin-vue-components/blob/main/src/core/resolvers/prime-vue.ts

// !! some components need to register their own service, so we need to import them here
// ConfirmDialog, ConfirmPopup, Toast, Tooltip

export function install({ app }: { app: App }) {
  app.component('Toast', Toast)

  app.use(ToastService)
  app.use(PrimeVue, { ripple: true })
}
