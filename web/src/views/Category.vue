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
const loading = ref(false)
const error = ref('')
const q = ref('')
const tab = ref('分类')

async function loadCategories() {
  try {
    const data = await api('/categories')
    categories.value = data.items
    if (!current.value && categories.value.length) current.value = categories.value[0].name
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
    items.value = data.items
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

const count = computed(() => (p) => cart.items.value.find((i) => i.id === p.id)?.qty || 0)

// 演示用的销量文案，按商品 id 生成一个稳定数字
function sold(p) {
  return ((p.id * 137) % 90) / 10 + 1
}

function plus(p) {
  cart.add(p)
}

function minus(p) {
  const hit = cart.items.value.find((i) => i.id === p.id)
  if (hit) cart.setQty(hit.id, hit.qty - 1)
}

onMounted(loadCategories)
</script>

<template>
  <div class="catpage">
    <!-- 绿色头部 -->
    <header class="cat-head">
      <div class="cat-search">
        <svg viewBox="0 0 24 24"><circle cx="11" cy="11" r="6.5" /><path d="M16 16l4.5 4.5" /></svg>
        <input v-model="q" placeholder="搜索商品" @keyup.enter="search" />
      </div>
      <button class="cat-cart" aria-label="购物车" @click="router.push('/checkout')">
        <svg viewBox="0 0 24 24"><path d="M3 4h2.2l2 10.5h10.2L20 7H6.1" /><circle cx="9.5" cy="19" r="1.6" /><circle cx="17" cy="19" r="1.6" /></svg>
        <em v-if="cart.count.value" class="cat-cart-badge">{{ cart.count.value > 99 ? '99+' : cart.count.value }}</em>
      </button>
    </header>

    <!-- 分类 / 商家 -->
    <nav class="cat-switch">
      <button :class="{ on: tab === '分类' }" @click="tab = '分类'">分类</button>
      <button :class="{ on: tab === '商家' }" @click="tab = '商家'">商家</button>
    </nav>

    <p v-if="error" class="error cat-err">{{ error }}</p>

    <div class="cat-body">
      <!-- 左侧一级分类 -->
      <aside class="cat-side">
        <button
          v-for="c in categories"
          :key="c.name"
          :class="{ on: current === c.name }"
          @click="pick(c.name)"
        >{{ c.name }}</button>
        <p v-if="!categories.length" class="muted small">暂无分类</p>
      </aside>

      <!-- 右侧商品列表 -->
      <section class="cat-list">
        <p v-if="tab === '商家'" class="muted cat-tip">商家列表暂未开放</p>
        <p v-else-if="loading" class="muted cat-tip">加载中…</p>
        <p v-else-if="!items.length" class="muted cat-tip">该分类暂无商品</p>

        <ul v-else>
          <li v-for="p in items" :key="p.id" class="cat-row">
            <img v-if="p.image_url" :src="p.image_url" :alt="p.name" loading="lazy" />
            <div v-else class="cat-thumb-ph">无图</div>
            <div class="cat-info">
              <div class="cat-name">{{ p.name }}</div>
              <div class="cat-desc">{{ p.description }}</div>
              <div class="cat-price-row">
                <div>
                  <div class="cat-price">¥{{ p.price.toFixed(2) }}</div>
                  <div class="cat-sold">{{ sold(p) }}w人已买</div>
                </div>
                <div class="cat-actions">
                  <template v-if="count(p)">
                    <button class="cat-minus" @click="minus(p)">−</button>
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
