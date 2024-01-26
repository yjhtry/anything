import type { App } from 'vue'

import 'primevue/resources/themes/aura-dark-green/theme.css'

import ToastService from 'primevue/toastservice'

import Toast from 'primevue/toast'
import PrimeVue from 'primevue/config'
import InputText from 'primevue/inputtext'

export function install({ app }: { app: App }) {
  app.component('Toast', Toast)
  app.component('InputText', InputText)

  app.use(ToastService)
  app.use(PrimeVue, { ripple: true })
}
