import { fileURLToPath, URL } from 'node:url'

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    vue(),
  ],
  resolve: {
    alias: {
      '@a': fileURLToPath(new URL('./src/assets', import.meta.url)),
      '@p': fileURLToPath(new URL('./src/pages', import.meta.url)),
      '@c': fileURLToPath(new URL('./src/components', import.meta.url)),
      '@r': fileURLToPath(new URL('./src/router', import.meta.url)),
      '@s': fileURLToPath(new URL('./src', import.meta.url)),
    }
  }
})
