import { createRouter, createWebHistory } from 'vue-router'
import Home from '@p/Home/Home.vue';
import PageNotFound from '@p/utility/404/404.vue';
import About from '@p/About/About.vue';
import Calculator from '@p/Calculator/Calculator.vue';
import CalculatorPrefixes from '@p/Calculator/CalculatorPrefixes.vue';
const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      component: Home,
    },
    {
      path: '/subnet-calculator',
      component: Calculator,
    },
    {
      path: '/subnet-calculator/prefixes',
      component: CalculatorPrefixes,
    },
    {
      path: '/network-visualizer',
      component: Home,
    },
    {
      path: '/about',
      component: About,
    },
    {
      path: '/:catchAll(.*)',
      component: PageNotFound
    },

  ]
})

export default router
