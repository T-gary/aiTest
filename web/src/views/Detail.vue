<script setup>
import { computed, onMounted, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { api } from '../api'
import { cart } from '../cart'

const route = useRoute()
const router = useRouter()

const p = ref(null)
const loading = ref(true)
const error = ref('')
const tab = ref('基本信息')
const qty = ref(1)

onMounted(async () => {
  try {
    p.value = await api(`/products/${route.params.id}`)
  } catch (e) {
    error.value = e.message
  } finally {
    loading.value = false
  }
})

function addToCart() {
  if (!p.value) return
  for (let i = 0; i < qty.value; i++) cart.add(p.value)
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
          <img v-if="p.image_url" :src="p.image_url" :alt="p.name" />
          <div v-else class="dt-noimg">暂无图片</div>
          <span class="dt-index">1/1</span>
        </div>

        <div class="dt-pricebar">
          <span class="dt-price">¥{{ p.price.toFixed(2) }}</span>
          <div class="dt-countdown">
            <span>距离本场结束还剩</span>
            <em>01 : 41 : 07</em>
          </div>
        </div>

        <div class="dt-info">
          <h1 class="dt-name">{{ p.name }}</h1>
          <p class="dt-pickup">提货时间：长期有效</p>
          <div class="dt-qty">
            <button :disabled="qty <= 1" @click="qty--">−</button>
            <span>{{ qty }}</span>
            <button @click="qty++">+</button>
          </div>
        </div>

        <div class="dt-divider"><span>商品详情</span></div>

        <div class="dt-desc">{{ p.description }}</div>
      </section>

      <!-- 图文详情 -->
      <section v-else class="dt-rich">
        <p class="dt-rich-text">{{ p.description }}</p>
        <img v-if="p.image_url" :src="p.image_url" :alt="p.name" class="dt-rich-img" />
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
