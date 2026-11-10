<script setup>
import { computed, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { auth } from './api'

const route = useRoute()
const router = useRouter()
const user = ref(auth.user())

const navs = computed(() => {
  const items = [{ to: '/', label: '首页' }]
  if (!user.value) items.push({ to: '/login', label: '登录 / 注册' })
  if (user.value) items.push({ to: '/admin', label: user.value.is_admin ? '后台管理' : '我的商品' })
  return items
})

function logout() {
  auth.logout()
  user.value = null
  router.push('/')
}

router.afterEach(() => {
  user.value = auth.user()
})
</script>

<template>
  <div class="app">
    <header class="hdr">
      <RouterLink to="/" class="brand">极简商城</RouterLink>
      <nav>
        <RouterLink v-for="n in navs" :key="n.to" :to="n.to" :class="{ active: route.path === n.to }">
          {{ n.label }}
        </RouterLink>
        <span v-if="user" class="who">
          {{ user.username }}
          <button class="link" @click="logout">退出</button>
        </span>
      </nav>
    </header>
    <main class="content">
      <RouterView />
    </main>
  </div>
</template>
