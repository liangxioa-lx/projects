import { createRouter, createWebHashHistory } from 'vue-router'


const router = createRouter({
  history: createWebHashHistory(import.meta.env.VITE_CREATEWEBHISTORY_URL),
  routes: [
    {
      path: '/',
      name: 'project',
      component: () => import('../pages/project/index.vue')
    },
  ]
})

export default router
