import { createRouter, createWebHashHistory } from 'vue-router'
import Home from '../views/App.vue'

const About = { template: '<div>About</div>' }

const routes = [
  { path: '/', component: Home },
  { path: '/about', component: About },
]


const router = createRouter({
  history: createWebHashHistory(),
  routes,
})

export default router;