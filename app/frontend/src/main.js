import './assets/main.css'
import { createI18n } from 'vue-i18n'
import { createApp } from 'vue'


import App from './App.vue'
import router from './router'
import './index.css'
import CS from './translations/cs.json';
import EN from './translations/en.json';
import { getCookie } from '@s/utils/utils.js';
const app = createApp(App)

const i18n = createI18n({
  locale: 'CS', // set locale
  fallbackLocale: 'EN', // set fallback locale
  messages: {
    EN: EN,
    CS: CS,
  },
})
app.use(router)
app.use(i18n)

app.mount('#app')
