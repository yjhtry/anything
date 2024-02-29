import type { App } from 'vue'

import 'primeicons/primeicons.css'

import ToastService from 'primevue/toastservice'
import Tooltip from 'primevue/tooltip'
import ConfirmationService from 'primevue/confirmationservice'
import FloatLabel from 'primevue/floatlabel'

import Toast from 'primevue/toast'
import ConfirmPopup from 'primevue/confirmpopup'
import PrimeVue from 'primevue/config'

// Most of components will be imported automatically in Vite
// see https://github.com/unplugin/unplugin-vue-components/blob/main/src/core/resolvers/prime-vue.ts

// !! some components need to register their own service, so we need to import them here
// ConfirmDialog, ConfirmPopup, Toast, Tooltip

export function install({ app }: { app: App }) {
  app.component('Toast', Toast)
  app.component('ConfirmPopup', ConfirmPopup)
  app.component('FloatLabel', FloatLabel)
  app.directive('tooltip', Tooltip)

  app.use(ToastService)
  app.use(ConfirmationService)
  app.use(PrimeVue, { ripple: true })
}
