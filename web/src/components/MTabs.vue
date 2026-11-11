<script setup>
import { useRoute, useRouter } from 'vue-router'
import { cart } from '../cart'
import { messageState } from '../messages'

const route = useRoute()
const router = useRouter()

function goCats() {
  if (route.path === '/') document.querySelector('.m-catbar')?.scrollIntoView({ behavior: 'smooth', block: 'center' })
  else router.push('/')
}
</script>

<template>
  <nav class="m-tabs">
    <RouterLink to="/" :class="{ on: route.path === '/' }">
      <svg viewBox="0 0 24 24"><path d="M4 11l8-7 8 7v8.5H4V11z" /></svg>
      <span>首页</span>
    </RouterLink>
    <button @click="goCats">
      <svg viewBox="0 0 24 24"><rect x="4" y="4" width="7" height="7" rx="1" /><rect x="13" y="4" width="7" height="7" rx="1" /><rect x="4" y="13" width="7" height="7" rx="1" /><rect x="13" y="13" width="7" height="7" rx="1" /></svg>
      <span>分类</span>
    </button>
    <RouterLink to="/messages" :class="{ on: route.path === '/messages' }">
      <svg viewBox="0 0 24 24"><path d="M4 5.5h16v11H9l-5 4v-15z" /></svg>
      <span>消息</span>
      <em v-if="messageState.unread" class="m-tab-badge">{{ messageState.unread > 99 ? '99+' : messageState.unread }}</em>
    </RouterLink>
    <RouterLink to="/checkout" :class="{ on: route.path === '/checkout' }">
      <svg viewBox="0 0 24 24"><path d="M3 4h2.2l2 10.5h10.2L20 7H6.1" /><circle cx="9.5" cy="19" r="1.6" /><circle cx="17" cy="19" r="1.6" /></svg>
      <span>购物车</span>
      <em v-if="cart.count.value" class="m-tab-badge">{{ cart.count.value > 99 ? '99+' : cart.count.value }}</em>
    </RouterLink>
    <RouterLink to="/profile" :class="{ on: route.path === '/profile' }">
      <svg viewBox="0 0 24 24"><circle cx="12" cy="8.5" r="3.8" /><path d="M4.5 20c1.2-3.8 4-5.5 7.5-5.5s6.3 1.7 7.5 5.5" /></svg>
      <span>我的</span>
    </RouterLink>
  </nav>
</template>
