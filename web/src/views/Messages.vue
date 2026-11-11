<script setup>
import { computed, onMounted, ref } from 'vue'
import { api } from '../api'
import { cart } from '../cart'
import { messageState } from '../messages'
import MTabs from '../components/MTabs.vue'

const active = ref('会话')
const filter = ref('全部')
const products = ref([])

const entries = [
  { name: '会话', tone: 'session', unread: true },
  { name: '物流', tone: 'logistics' },
  { name: '提醒', tone: 'remind' },
  { name: '优惠', tone: 'promo' },
  { name: '互动', tone: 'interact' }
]

const filters = [
  { name: '全部' },
  { name: '购物' },
  { name: '秒送', tag: '外卖' }
]

const messages = computed(() => [
  { id: 1, title: '极简商城 客服', official: true, desc: '欢迎光临，有任何问题都可以在这里留言', time: '3 天前' }
])

onMounted(async () => {
  try {
    const data = await api('/products?size=6&page=1')
    products.value = data.items
  } catch {
    products.value = []
  }
})
</script>

<template>
  <div class="msgs">
    <header class="msg-head">
      <h1>消息</h1>
      <div class="row">
        <button class="ghost-light" @click="messageState.clear()">清除未读</button>
        <span class="dots3">···</span>
      </div>
    </header>

    <nav class="msg-entries">
      <button
        v-for="e in entries"
        :key="e.name"
        :class="['entry', e.tone, { on: active === e.name }]"
        @click="active = e.name"
      >
        <span class="entry-icon">
          <svg v-if="e.tone === 'session'" viewBox="0 0 24 24"><circle cx="12" cy="12" r="9" /><path d="M8.5 12.5c1 1.2 2.3 1.2 3.5 1.2s2.5 0 3.5-1.2" /></svg>
          <svg v-else-if="e.tone === 'logistics'" viewBox="0 0 24 24"><path d="M3 7h9v9H3zM12 10h4l4 3.5V16h-8z" /><circle cx="7" cy="18" r="1.6" /><circle cx="17" cy="18" r="1.6" /></svg>
          <svg v-else-if="e.tone === 'remind'" viewBox="0 0 24 24"><path d="M6 16V11a6 6 0 0 1 12 0v5l1.5 2H4.5L6 16z" /><path d="M10 20h4" /></svg>
          <svg v-else-if="e.tone === 'promo'" viewBox="0 0 24 24"><path d="M4 4h9l7 7-9 9-7-7V4z" /><circle cx="8.5" cy="8.5" r="1.4" /></svg>
          <svg v-else viewBox="0 0 24 24"><path d="M4 6h10v9H8l-4 3V6z" /><path d="M14 9h6v7l-3 2.5" /></svg>
          <i v-if="e.unread && messageState.unread" class="entry-badge">1</i>
        </span>
        <span class="entry-label">{{ e.name }}</span>
      </button>
    </nav>

    <nav class="msg-filters">
      <button v-for="f in filters" :key="f.name" :class="{ on: filter === f.name }" @click="filter = f.name">
        {{ f.name }}
        <em v-if="f.tag">{{ f.tag }}</em>
      </button>
    </nav>

    <section class="msg-list">
      <p class="msg-date">三天前的消息</p>
      <article v-for="m in messages" :key="m.id" class="msg-item">
        <div class="msg-avatar">
          <svg viewBox="0 0 24 24"><circle cx="12" cy="12" r="9" /><path d="M8.5 12.5c1 1.2 2.3 1.2 3.5 1.2s2.5 0 3.5-1.2" /></svg>
        </div>
        <div class="msg-body">
          <div class="msg-title">
            <span>{{ m.title }}</span>
            <em v-if="m.official">官方</em>
          </div>
          <p class="msg-desc">{{ m.desc }}</p>
        </div>
      </article>
      <p class="msg-end">没有更多消息了</p>
    </section>

    <section class="msg-reco">
      <div class="msg-reco-head">
        <strong>推荐</strong>
        <span>猜你喜欢</span>
      </div>
      <ul class="msg-grid">
        <li v-for="p in products" :key="p.id">
          <img v-if="p.image_url" :src="p.image_url" :alt="p.name" loading="lazy" />
          <div v-else class="thumb-ph">暂无图片</div>
          <div class="msg-pbody">
            <div class="msg-pname">{{ p.name }}</div>
            <div class="msg-pfoot">
              <span class="price">¥{{ p.price.toFixed(2) }}</span>
              <button @click="cart.add(p)">+</button>
            </div>
          </div>
        </li>
      </ul>
    </section>

    <MTabs />
  </div>
</template>
