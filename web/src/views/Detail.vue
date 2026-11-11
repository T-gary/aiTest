<script setup>
import { computed, onBeforeUnmount, onMounted, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { api } from '../api'
import { cart } from '../cart'

const route = useRoute()
const router = useRouter()

const p = ref(null)
const loading = ref(true)
const error = ref('')
const tab = ref('基本信息')
const localQty = ref(1)
const activeImage = ref(0)

// 多图：image_url 支持用英文逗号分隔多个地址
const images = computed(() =>
  (p.value?.image_url || '')
    .split(',')
    .map((s) => s.trim())
    .filter(Boolean)
)

// 购物车里的同款商品（存在时数量与购物车实时联动）
const cartItem = computed(() => cart.items.value.find((i) => i.id === p.value?.id))
const qty = computed(() => cartItem.value?.qty ?? localQty.value)

onMounted(async () => {
  try {
    p.value = await api(`/products/${route.params.id}`)
  } catch (e) {
    error.value = e.message
  } finally {
    loading.value = false
    startCountdown()
  }
})

// 本场结束倒计时：每天 24:00 结束，仅展示
const now = ref(Date.now())
let clock = 0

const countdown = computed(() => {
  const d = new Date(now.value)
  const end = new Date(d)
  end.setHours(24, 0, 0, 0)
  let left = Math.max(0, Math.floor((end.getTime() - d.getTime()) / 1000))
  const h = String(Math.floor(left / 3600)).padStart(2, '0')
  left %= 3600
  const m = String(Math.floor(left / 60)).padStart(2, '0')
  const s = String(left % 60).padStart(2, '0')
  return `${h} : ${m} : ${s}`
})

function startCountdown() {
  clearInterval(clock)
  clock = setInterval(() => (now.value = Date.now()), 1000)
}

onBeforeUnmount(() => clearInterval(clock))

function plus() {
  if (!p.value) return
  if (cartItem.value) cart.setQty(p.value.id, cartItem.value.qty + 1)
  else localQty.value++
}

function minus() {
  if (!p.value) return
  if (cartItem.value) {
    if (cartItem.value.qty > 1) cart.setQty(p.value.id, cartItem.value.qty - 1)
    else cart.remove(p.value.id)
  } else if (localQty.value > 1) {
    localQty.value--
  }
}

function addToCart() {
  if (!p.value) return
  if (cartItem.value) cart.setQty(p.value.id, qty.value)
  else for (let i = 0; i < localQty.value; i++) cart.add(p.value)
}
</script>

<template>
  <div class="detail">
    <!-- 顶部栏 -->
    <header class="dt-head">
      <button class="dt-back" aria-label="返回" @click="router.back()">
        <svg viewBox="0 0 24 24"><path d="M14.5 5L8 12l6.5 7" /></svg>
      </button>
      <span class="dt-title">商品信息</span>
      <span class="dt-actions">
        <em>···</em>
        <i></i>
      </span>
    </header>

    <p v-if="error" class="error dt-err">{{ error }}</p>
    <p v-else-if="loading" class="muted dt-err">加载中…</p>

    <template v-else-if="p">
      <!-- 分区切换 -->
      <nav class="dt-tabs">
        <button :class="{ on: tab === '基本信息' }" @click="tab = '基本信息'">基本信息</button>
        <button :class="{ on: tab === '图文详情' }" @click="tab = '图文详情'">图文详情</button>
      </nav>

      <!-- 基本信息 -->
      <section v-if="tab === '基本信息'" class="dt-basic">
        <div class="dt-gallery">
          <img v-if="images.length" :src="images[activeImage] || images[0]" :alt="p.name" />
          <div v-else class="dt-noimg">暂无图片</div>
          <span class="dt-index">{{ images.length ? activeImage + 1 : 1 }}/{{ Math.max(images.length, 1) }}</span>
        </div>

        <!-- 缩略图 / 款式切换 -->
        <div v-if="images.length" class="dt-thumbs">
          <div class="dt-thumb-row">
            <button
              v-for="(src, i) in images"
              :key="src + i"
              class="dt-thumb"
              :class="{ on: i === activeImage }"
              @click="activeImage = i"
            >
              <img :src="src" :alt="`图${i + 1}`" loading="lazy" />
            </button>
          </div>
          <span class="dt-variants">共 {{ images.length }} 款 ›</span>
        </div>

        <div class="dt-pricebar">
          <span class="dt-price">¥{{ p.price.toFixed(2) }}</span>
          <div class="dt-countdown">
            <span>距离本场结束还剩</span>
            <em>{{ countdown }}</em>
          </div>
        </div>

        <div class="dt-info">
          <h1 class="dt-name">{{ p.name }}</h1>
          <p class="dt-pickup">提货时间：长期有效</p>
          <div class="dt-qty">
            <button :disabled="qty <= 1" @click="minus">−</button>
            <span>{{ qty }}</span>
            <button @click="plus">+</button>
          </div>
        </div>

        <div class="dt-divider"><span>商品详情</span></div>

        <div class="dt-desc">{{ p.description }}</div>
      </section>

      <!-- 图文详情 -->
      <section v-else class="dt-rich">
        <p class="dt-rich-text">{{ p.description }}</p>
        <img v-if="images.length" :src="images[activeImage] || images[0]" :alt="p.name" class="dt-rich-img" />
        <p class="dt-rich-meta">
          分类：{{ p.category || '未分类' }}<br />
          录入人：{{ p.created_by }}<br />
          上架时间：{{ p.created_at }}
        </p>
      </section>
    </template>

    <!-- 底部操作栏 -->
    <div v-if="p" class="dt-bar">
      <button class="dt-bar-btn">
        <svg viewBox="0 0 24 24"><path d="M12 15V4M8 8l4-4 4 4" /><path d="M5 14v5.5h14V14" /></svg>
        <span>分享</span>
      </button>
      <button class="dt-bar-btn" @click="router.push('/checkout')">
        <span class="dt-bar-icon">
          <svg viewBox="0 0 24 24"><path d="M3 4h2.2l2 10.5h10.2L20 7H6.1" /><circle cx="9.5" cy="19" r="1.6" /><circle cx="17" cy="19" r="1.6" /></svg>
          <em v-if="cart.count.value" class="dt-bar-badge">{{ cart.count.value > 99 ? '99+' : cart.count.value }}</em>
        </span>
        <span>购物车</span>
      </button>
      <button class="dt-cart" @click="addToCart">加入购物车</button>
      <button class="dt-buy" @click="addToCart(); router.push('/checkout')">立即购买</button>
    </div>
  </div>
</template>
