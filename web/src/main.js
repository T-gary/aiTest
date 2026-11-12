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
import Detail from './views/Detail.vue'
import Cart from './views/Cart.vue'
import Orders from './views/Orders.vue'
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
    { path: '/category', component: Category },
    { path: '/product/:id', component: Detail },
    { path: '/cart', component: Cart },
    { path: '/orders', component: Orders }
  ]
})

createApp(App).use(router).mount('#app')
