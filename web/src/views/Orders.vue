<script setup>
import { ref, onMounted } from 'vue'
import { api, auth } from '../api'
import MTabs from '../components/MTabs.vue'

const orders = ref([])
const error = ref('')
const busy = ref(true)
const logged = ref(!!auth.token())

onMounted(async () => {
  if (!logged.value) {
    busy.value = false
    return
  }
  try {
    orders.value = (await api('/orders')).items
  } catch (e) {
    error.value = e.message
  } finally {
    busy.value = false
  }
})
</script>

<template>
  <div class="orderspage">
    <header class="cart-head">
      <div class="cart-title">我的订单</div>
    </header>

    <p v-if="!logged" class="cart-empty">
      请先<RouterLink to="/login?redirect=/orders" class="link">登录</RouterLink>查看订单
    </p>
    <p v-else-if="busy" class="cart-empty">加载中…</p>
    <p v-else-if="error" class="error cart-empty">{{ error }}</p>
    <p v-else-if="!orders.length" class="cart-empty">
      还没有订单，<RouterLink to="/" class="link">去挑商品</RouterLink>
    </p>

    <section v-for="o in orders" :key="o.id" class="order-card">
      <header class="order-head">
        <span>订单号 #{{ o.id }}</span>
        <span class="muted small">{{ o.created_at }}</span>
        <em class="order-status">{{ o.status }}</em>
      </header>
      <article v-for="(i, idx) in o.items" :key="idx" class="order-row">
        <img v-if="i.image_url" :src="i.image_url" :alt="i.name" class="order-thumb" />
        <div v-else class="order-thumb order-thumb-ph">无图</div>
        <div class="order-info">
          <div class="order-name">{{ i.name }}</div>
          <div class="muted small">¥{{ i.price.toFixed(2) }} × {{ i.qty }}</div>
        </div>
      </article>
      <footer class="order-foot">
        合计 <span class="price">¥{{ o.total.toFixed(2) }}</span>
      </footer>
    </section>

    <MTabs />
  </div>
</template>
