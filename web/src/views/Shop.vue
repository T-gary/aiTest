<script setup>
import { computed, onBeforeUnmount, onMounted, ref } from 'vue'
import { api } from '../api'
import { cart } from '../cart'
import { mobileView } from '../viewmode'

const items = ref([])
const total = ref(0)
const page = ref(1)
const size = ref(20)
const pages = ref(1)
const q = ref('')
const category = ref('')
const cats = ref([])
const featured = ref([])
const slide = ref(0)
const loading = ref(false)
const error = ref('')

let timer = 0

async function loadProducts() {
  loading.value = true
  error.value = ''
  try {
    const p = new URLSearchParams({ page: page.value, size: size.value })
    if (q.value.trim()) p.set('q', q.value.trim())
    if (category.value) p.set('category', category.value)
    const data = await api(`/products?${p}`)
    items.value = data.items
    total.value = data.total
    pages.value = Math.max(1, Math.ceil(data.total / size.value))
  } catch (e) {
    error.value = e.message
  } finally {
    loading.value = false
  }
}

async function loadShell() {
  try {
    const [c, f] = await Promise.all([api('/categories'), api('/products?featured=1&size=6')])
    cats.value = c.items
    featured.value = f.items
  } catch (e) {
    error.value = e.message
  }
}

function search() {
  if (page.value === 1) loadProducts()
  else page.value = 1
}

function pick(name) {
  category.value = name
  page.value = 1
  loadProducts()
}

function go(p) {
  page.value = p
  loadProducts()
}

const current = computed(() => featured.value[slide.value % (featured.value.length || 1)])

onMounted(() => {
  loadShell()
  loadProducts()
  timer = setInterval(() => {
    if (featured.value.length > 1) slide.value = (slide.value + 1) % featured.value.length
  }, 4000)
})

onBeforeUnmount(() => clearInterval(timer))
</script>

<template>
  <div class="shop">
    <header class="topbar">
      <div class="logo">极简商城</div>
      <div class="searchbox">
        <input v-model="q" placeholder="搜索商品" @keyup.enter="search" />
        <button @click="search">搜索</button>
      </div>
    </header>

    <p v-if="error" class="error">{{ error }}</p>

    <div class="shop-body">
      <aside class="cats">
        <button :class="{ on: !category }" @click="pick('')">
          <span>全部商品</span><em>{{ total }}</em>
        </button>
        <button v-for="c in cats" :key="c.name" :class="{ on: category === c.name }" @click="pick(c.name)">
          <span>{{ c.name }}</span><em>{{ c.count }}</em>
        </button>
        <p v-if="!cats.length" class="muted small">暂无分类，录入商品时填写「分类」即可出现在这里。</p>
      </aside>

      <section class="main">
        <div class="banner" :style="current?.image_url ? { backgroundImage: `url(${current.image_url})` } : {}">
          <div v-if="!current" class="banner-empty">在后台标准商品时可勾选「首页轮播」，这里会展示促销位</div>
          <div v-else class="banner-card">
            <div class="banner-tag">限时促销</div>
            <div class="banner-name">{{ current.name }}</div>
            <div class="banner-desc">{{ current.description }}</div>
            <div class="row">
              <span class="banner-price">¥{{ current.price.toFixed(2) }}</span>
              <button @click="cart.add(current)">加入购物车</button>
            </div>
          </div>
          <div v-if="featured.length > 1" class="dots">
            <i v-for="(f, i) in featured" :key="f.id" :class="{ on: i === slide }" @click="slide = i"></i>
          </div>
        </div>

        <div class="listhead">
          <h1>{{ category || '全部商品' }}</h1>
          <span class="muted">共 {{ total }} 件</span>
        </div>

        <p v-if="loading" class="muted">加载中…</p>
        <p v-else-if="!items.length" class="muted">暂无商品</p>

        <ul v-else class="plist">
          <li v-for="p in items" :key="p.id">
            <img v-if="p.image_url" :src="p.image_url" :alt="p.name" class="pthumb" loading="lazy" />
            <div v-else class="thumb-ph">暂无图片</div>
            <div class="pbody">
              <div class="pname">{{ p.name }}</div>
              <div class="pdesc">{{ p.description }}</div>
              <div class="row">
                <span v-if="p.category" class="chip">{{ p.category }}</span>
                <span class="muted small">{{ p.created_at }}</span>
              </div>
              <div class="pfoot">
                <span class="price">¥{{ p.price.toFixed(2) }}</span>
                <button @click="cart.add(p)">{{ cart.has(p.id) ? '再加一件' : '加入购物车' }}</button>
              </div>
            </div>
          </li>
        </ul>

        <div v-if="pages > 1" class="pager">
          <button class="ghost" :disabled="page <= 1" @click="go(page - 1)">上一页</button>
          <span class="muted">{{ page }} / {{ pages }}</span>
          <button class="ghost" :disabled="page >= pages" @click="go(page + 1)">下一页</button>
        </div>
      </section>

      <aside class="rail">
        <RouterLink to="/checkout" class="railitem" title="购物车">
          <span class="railicon">
            <svg viewBox="0 0 24 24" aria-hidden="true">
              <path d="M3 4h2.2l2 10.5h10.2L20 7H6.1" />
              <circle cx="9.5" cy="19" r="1.6" />
              <circle cx="17" cy="19" r="1.6" />
            </svg>
            <em v-if="cart.count.value" class="badge">{{ cart.count.value > 99 ? '99+' : cart.count.value }}</em>
          </span>
          <span class="raillabel">购物车</span>
        </RouterLink>
        <span class="railitem off">
          <span class="railicon">
            <svg viewBox="0 0 24 24" aria-hidden="true"><circle cx="12" cy="12" r="8.5" /><path d="M8.5 13.5c1 1.4 2.3 1.4 3.5 1.4s2.5 0 3.5-1.4" /></svg>
          </span>
          <span class="raillabel">我的</span>
        </span>
        <span class="railitem off">
          <span class="railicon">
            <svg viewBox="0 0 24 24" aria-hidden="true"><path d="M5 13a7 7 0 0 1 14 0" /><rect x="3" y="13" width="3.5" height="5.5" rx="1.7" /><rect x="17.5" y="13" width="3.5" height="5.5" rx="1.7" /><path d="M19.2 18.5c0 1.6-1.6 2.5-3.7 2.5" /></svg>
          </span>
          <span class="raillabel">客服</span>
        </span>
        <button class="railitem" title="切换到移动版预览" @click="mobileView = true">
          <span class="railicon">
            <svg viewBox="0 0 24 24" aria-hidden="true"><rect x="7" y="2.5" width="10" height="19" rx="2" /><path d="M10.5 5h3M11 18.5h2" /></svg>
          </span>
          <span class="raillabel">移动版</span>
        </button>
        <span class="railitem off">
          <span class="railicon">
            <svg viewBox="0 0 24 24" aria-hidden="true"><path d="M8 4h8v3.5h3.5V14a2 2 0 1 1 0 4v2H8v-2.5a2 2 0 1 1 0-4V4z" /></svg>
          </span>
          <span class="raillabel">插件版</span>
        </span>
        <span class="railitem off">
          <span class="railicon">
            <svg viewBox="0 0 24 24" aria-hidden="true"><rect x="3.5" y="4" width="17" height="16" rx="2" /><path d="M7.5 15.5l5-5 2.5 2.5-5 5H7.5v-2.5z" /></svg>
          </span>
          <span class="raillabel">反馈</span>
        </span>
        <span class="railitem off">
          <span class="railicon">
            <svg viewBox="0 0 24 24" aria-hidden="true"><rect x="4" y="3.5" width="16" height="17" rx="2" /><path d="M8 8.5h8M8 12.5h8M8 16.5h5" /></svg>
          </span>
          <span class="raillabel">调研</span>
        </span>
      </aside>
    </div>
  </div>
</template>
