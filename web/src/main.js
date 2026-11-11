import { createApp } from 'vue'
import { createRouter, createWebHistory } from 'vue-router'
import App from './App.vue'
import Shop from './views/Shop.vue'
import Login from './views/Login.vue'
import Admin from './views/Admin.vue'
import Checkout from './views/Checkout.vue'
import Messages from './views/Messages.vue'
import Profile from './views/Profile.vue'
import Category from './views/Category.vue'
import './style.css'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/', component: Shop },
    { path: '/login', component: Login },
    { path: '/admin', component: Admin },
    { path: '/checkout', component: Checkout },
    { path: '/messages', component: Messages },
    { path: '/profile', component: Profile },
    { path: '/category', component: Category }
  ]
})

createApp(App).use(router).mount('#app')
