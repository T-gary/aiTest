<script setup>
import { computed, onBeforeUnmount, ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { auth } from './api'
import { mobileView } from './viewmode'
import SideRail from './components/SideRail.vue'

const route = useRoute()
const router = useRouter()
const user = ref(auth.user())

// 侧边栏：除后台管理外所有页面显示；手机预览时由移动版组件接管
const showRail = computed(() => route.path !== '/admin')

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

// 手机预览：容器宽度锁在 390px，样式靠容器查询切换；直接按 ESC 退出
function onKey(e) {
  if (e.key === 'Escape') mobileView.value = false
}
window.addEventListener('keydown', onKey)
onBeforeUnmount(() => window.removeEventListener('keydown', onKey))
watch(mobileView, (v) => document.documentElement.classList.toggle('in-phone', v), { immediate: true })
</script>

<template>
  <div class="stage" :class="{ phone: mobileView }">
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
      <main class="content with-rail">
        <RouterView />
      </main>
    </div>
    <SideRail v-if="showRail" />
    <button v-if="mobileView" class="phone-exit" @click="mobileView = false">退出移动版</button>
  </div>
</template>
