<script setup>
import { computed, onBeforeUnmount, onMounted, ref } from 'vue'
import { api } from '../api'
import { cart } from '../cart'

const items = ref([])
const total = ref(0)
const page = ref(1)
const size = ref(10)
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
            <img v-if="p.image_url" :src="p.image_url" :alt="p.name" loading="lazy" />
            <div v-else class="thumb-ph">暂无图片</div>
            <div class="pinfo">
              <div class="pname">{{ p.name }}</div>
              <div class="pdesc">{{ p.description }}</div>
              <div class="row">
                <span v-if="p.category" class="chip">{{ p.category }}</span>
                <span class="muted small">{{ p.created_at }}</span>
              </div>
            </div>
            <div class="pbuy">
              <span class="price">¥{{ p.price.toFixed(2) }}</span>
              <button @click="cart.add(p)">{{ cart.has(p.id) ? '再加一件' : '加入购物车' }}</button>
            </div>
          </li>
        </ul>

        <div v-if="pages > 1" class="pager">
          <button class="ghost" :disabled="page <= 1" @click="go(page - 1)">上一页</button>
          <span class="muted">{{ page }} / {{ pages }}</span>
          <button class="ghost" :disabled="page >= pages" @click="go(page + 1)">下一页</button>
        </div>
      </section>

      <aside class="cartside">
        <div class="carthead">
          <span>购物车</span>
          <em>{{ cart.count.value }}</em>
        </div>
        <div class="cartbody">
          <p v-if="!cart.items.value.length" class="muted small">还没加购商品</p>
          <div v-for="i in cart.items.value" :key="i.id" class="cartitem">
            <img v-if="i.image_url" :src="i.image_url" :alt="i.name" />
            <div class="cartinfo">
              <div class="cname">{{ i.name }}</div>
              <div class="muted small">¥{{ i.price.toFixed(2) }}</div>
              <div class="qty">
                <button class="ghost" @click="cart.setQty(i.id, i.qty - 1)">−</button>
                <span>{{ i.qty }}</span>
                <button class="ghost" @click="cart.setQty(i.id, i.qty + 1)">+</button>
                <button class="link" @click="cart.remove(i.id)">删除</button>
              </div>
            </div>
          </div>
        </div>
        <div class="cartfoot">
          <div class="row" style="justify-content: space-between">
            <span class="muted">合计</span>
            <span class="price">¥{{ cart.total.value.toFixed(2) }}</span>
          </div>
          <button v-if="cart.items.value.length" class="ghost" style="width: 100%" @click="cart.clear()">清空</button>
          <p class="muted small" style="margin: 8px 0 0">仅展示，不支持下单支付</p>
        </div>
      </aside>
    </div>
  </div>
</template>
