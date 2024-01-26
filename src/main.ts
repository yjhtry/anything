import { createApp } from 'vue'
import { createRouter, createWebHashHistory } from 'vue-router/auto'
import type { App as VueApp } from 'vue'
import App from './App.vue'

import '@unocss/reset/normalize.css'
import './styles/main.css'
import 'uno.css'

const app = createApp(App)
const router = createRouter({
  history: createWebHashHistory(import.meta.env.BASE_URL),
})
app.use(router)

app.mount('#app')

Object.values(import.meta.glob<{ install: (ctx: { app: VueApp }) => void }>('./modules/*.ts', { eager: true }))
  .forEach((module) => {
    module.install({ app })
  })
