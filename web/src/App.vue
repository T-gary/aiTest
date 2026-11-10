<script setup>
import { computed, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { auth } from './api'

const route = useRoute()
const router = useRouter()
const user = ref(auth.user())
const open = ref(false)

const navs = computed(() => {
  const items = [{ to: '/', label: '商品首页' }]
  if (!user.value) items.push({ to: '/login', label: '登录 / 注册' })
  if (user.value) items.push({ to: '/admin', label: user.value.is_admin ? '后台管理' : '我的商品' })
  return items
})

function logout() {
  auth.logout()
  user.value = null
  open.value = false
  router.push('/')
}

// 登录成功后同步导航状态
router.afterEach(() => {
  user.value = auth.user()
  open.value = false
})
</script>

<template>
  <div class="layout" :class="{ 'nav-open': open }">
    <aside class="sidebar">
      <div class="brand">极简商城</div>
      <nav>
        <RouterLink v-for="n in navs" :key="n.to" :to="n.to" :class="{ active: route.path === n.to }">
          {{ n.label }}
        </RouterLink>
      </nav>
      <div v-if="user" class="who">
        <span>{{ user.username }}</span>
        <button class="link" @click="logout">退出</button>
      </div>
    </aside>
    <button class="burger" @click="open = !open">菜单</button>
    <main class="content">
      <RouterView />
    </main>
  </div>
</template>
