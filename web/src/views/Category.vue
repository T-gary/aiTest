<script setup>
import { computed, onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'
import { api } from '../api'
import { cart } from '../cart'
import MTabs from '../components/MTabs.vue'

const router = useRouter()

const categories = ref([])
const current = ref('')
const items = ref([])
const featured = ref([])
const loading = ref(false)
const error = ref('')
const q = ref('')
const sort = ref('default')

async function loadShell() {
  try {
    const [c, f] = await Promise.all([api('/categories'), api('/products?featured=1&size=3')])
    categories.value = c.items
    featured.value = f.items
    await loadProducts()
  } catch (e) {
    error.value = e.message
  }
}

async function loadProducts() {
  loading.value = true
  error.value = ''
  try {
    const p = new URLSearchParams({ size: 40, page: 1 })
    if (current.value) p.set('category', current.value)
    if (q.value.trim()) p.set('q', q.value.trim())
    const data = await api(`/products?${p}`)
    let list = data.items
    if (sort.value === 'priceAsc') list = [...list].sort((a, b) => a.price - b.price)
    if (sort.value === 'priceDesc') list = [...list].sort((a, b) => b.price - a.price)
    items.value = list
  } catch (e) {
    error.value = e.message
  } finally {
    loading.value = false
  }
}

function pick(name) {
  current.value = name
  loadProducts()
}

function search() {
  loadProducts()
}

function togglePrice() {
  sort.value = sort.value === 'priceAsc' ? 'priceDesc' : 'priceAsc'
  loadProducts()
}

const banner = computed(() => featured.value[0])

function sold(p) {
  return (((p.id * 137) % 90) / 10 + 1).toFixed(1)
}

function count(p) {
  return cart.items.value.find((i) => i.id === p.id)?.qty || 0
}

function plus(p) {
  cart.add(p)
}

onMounted(loadShell)
</script>

<template>
  <div class="catpage">
    <!-- 顶部搜索 -->
    <header class="cat-head">
      <div class="cat-search">
        <svg viewBox="0 0 24 24"><circle cx="11" cy="11" r="6.5" /><path d="M16 16l4.5 4.5" /></svg>
        <input v-model="q" placeholder="搜索商品" @keyup.enter="search" />
      </div>
      <button class="cat-search-btn" @click="search">搜索</button>
    </header>

    <!-- 圆形分类条 -->
    <div class="cat-circles">
      <div class="cat-circle-row">
        <button
          v-for="(c, i) in categories"
          :key="c.name"
          class="cat-circle"
          :class="{ on: current === c.name }"
          @click="pick(c.name)"
        >
          <span class="cat-circle-icon" :class="'tone-' + (i % 6)">{{ c.name.slice(0, 1) }}</span>
          <span class="cat-circle-label">{{ c.name }}</span>
        </button>
      </div>
      <span class="cat-circle-more">展开</span>
    </div>

    <p v-if="error" class="error cat-err">{{ error }}</p>

    <div class="cat-body">
      <!-- 左侧 -->
      <aside class="cat-side">
        <button :class="{ on: !current }" @click="pick('')">热卖商品</button>
        <button
          v-for="c in categories"
          :key="c.name"
          :class="{ on: current === c.name }"
          @click="pick(c.name)"
        >{{ c.name }}</button>
      </aside>

      <!-- 右侧 -->
      <section class="cat-list">
        <!-- 活动 banner -->
        <div class="cat-banner" :style="banner?.image_url ? { backgroundImage: `url(${banner.image_url})` } : {}">
          <div class="cat-banner-card">
            <b>{{ banner?.name || '限时特惠' }}</b>
            <span>{{ banner ? '¥' + banner.price.toFixed(2) + ' 起' : '精选好物' }}</span>
            <button @click="banner && router.push(`/product/${banner.id}`)">立即抢购 ›</button>
          </div>
        </div>

        <!-- 排序 -->
        <div class="cat-sort">
          <button>销量</button>
          <button>折扣</button>
          <button :class="{ on: sort !== 'default' }" @click="togglePrice">
            价格 <em>{{ sort === 'priceDesc' ? '▼' : '▲' }}</em>
          </button>
        </div>

        <!-- 快捷筛选 -->
        <div class="cat-chips">
          <button class="on">全部</button>
          <button v-for="c in categories.slice(1, 3)" :key="c.name" @click="pick(c.name)">{{ c.name }}</button>
          <span class="cat-chips-more">品 ⌄</span>
        </div>

        <p v-if="loading" class="muted cat-tip">加载中…</p>
        <p v-else-if="!items.length" class="muted cat-tip">该分类暂无商品</p>

        <ul v-else>
          <li v-for="p in items" :key="p.id" class="cat-row" @click="router.push(`/product/${p.id}`)">
            <div class="cat-thumb-wrap">
              <img v-if="p.image_url" :src="p.image_url" :alt="p.name" loading="lazy" />
              <div v-else class="cat-thumb-ph">无图</div>
              <span v-if="p.featured" class="cat-badge">热卖</span>
            </div>
            <div class="cat-info">
              <div class="cat-name">
                <span class="cat-brand">自营</span>
                {{ p.name }}
              </div>
              <div class="cat-desc">{{ p.description }}</div>
              <div class="cat-tags">
                <span class="cat-tag">{{ p.category || '未分类' }}</span>
                <span class="cat-tag">{{ sold(p) }}w人回购</span>
              </div>
              <div class="cat-price-row">
                <div>
                  <span class="cat-price">¥{{ p.price.toFixed(2) }}</span>
                  <span class="cat-sold">到手价</span>
                </div>
                <div class="cat-actions" @click.stop>
                  <template v-if="count(p)">
                    <span class="cat-qty">{{ count(p) }}</span>
                    <button class="cat-plus" @click="plus(p)">+</button>
                  </template>
                  <button v-else class="cat-add" @click="plus(p)">+</button>
                </div>
              </div>
            </div>
          </li>
        </ul>
      </section>
    </div>

    <MTabs />
  </div>
</template>
