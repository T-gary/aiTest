<script setup>
import { ref, computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { cart, placeOrder } from '../cart'
import { auth } from '../api'
import MTabs from '../components/MTabs.vue'

const route = useRoute()
const router = useRouter()
const filter = ref('全部')
const filters = [
  { name: '全部' },
  { name: '购物' },
  { name: '秒送', tag: '外卖' },
  { name: '服务', tag: '宠物' }
]
const chips = ['国家补贴', '惊喜红包', '降价', '多买多返', '分组']

const checkedCount = computed(() => cart.checkedCount.value)
const checkedPieces = computed(() => cart.checkedPieces.value)

const busy = ref(false)
const error = ref('')
const done = ref(null)

async function settle() {
  if (!checkedCount.value || busy.value) return
  if (!auth.token()) {
    router.push({ path: '/login', query: { redirect: '/checkout' } })
    return
  }
  busy.value = true
  error.value = ''
  try {
    done.value = await placeOrder()
  } catch (e) {
    error.value = e.message
    if (!auth.token()) router.push({ path: '/login', query: { redirect: '/checkout' } })
  } finally {
    busy.value = false
  }
}
</script>

<template>
  <div class="cartpage">
    <header class="cart-head">
      <div class="cart-title">购物车 <em>({{ cart.kinds.value }})</em></div>
      <div class="cart-addr">朝阳区慧谷阳光100号楼</div>
      <div class="cart-actions">
        <button>对比</button>
        <button>管理</button>
        <span class="dots3">···</span>
      </div>
    </header>

    <nav class="cart-filters">
      <button v-for="f in filters" :key="f.name" :class="{ on: filter === f.name }" @click="filter = f.name">
        {{ f.name }}<em v-if="f.tag">{{ f.tag }}</em>
      </button>
      <span class="cart-search">
        <svg viewBox="0 0 24 24"><circle cx="11" cy="11" r="6.5" /><path d="M16 16l4.5 4.5" /></svg>
      </span>
    </nav>

    <div class="cart-chips">
      <span v-for="c in chips" :key="c">{{ c }}</span>
    </div>

    <div class="cart-promo">
      <span class="cart-promo-icon">￥</span>
      <span class="cart-promo-text">惊喜红包 4 张 最高省 77 元待使用</span>
      <button class="link">去使用</button>
    </div>

    <p v-if="!cart.items.value.length" class="cart-empty">购物车还是空的<RouterLink to="/" class="link">去挑商品</RouterLink></p>

    <section v-if="cart.items.value.length" class="cart-shop">
      <header class="cart-shop-head">
        <label class="gcheck">
          <input type="checkbox" :checked="cart.allChecked.value" @change="cart.toggleAll($event.target.checked)" />
          <span class="gcheck-box"></span>
        </label>
        <span class="shop-tag">自营</span>
        <span class="shop-name">极简商城官方店</span>
      </header>

      <article v-for="i in cart.items.value" :key="i.id" class="cart-row">
        <label class="gcheck">
          <input type="checkbox" :checked="i.checked" @change="cart.toggle(i.id)" />
          <span class="gcheck-box"></span>
        </label>
        <img v-if="i.image_url" :src="i.image_url" :alt="i.name" class="cart-thumb" />
        <div v-else class="cart-thumb cart-thumb-ph">无图</div>
        <div class="cart-info">
          <div class="cart-name">{{ i.name }}</div>
          <div class="cart-meta">
            <span class="cart-note">7 天价保险</span>
          </div>
          <div class="cart-price-row">
            <span class="price">¥{{ i.price.toFixed(2) }}</span>
            <div class="qty">
              <button class="ghost" @click="cart.setQty(i.id, i.qty - 1)">−</button>
              <span>{{ i.qty }}</span>
              <button class="ghost" @click="cart.setQty(i.id, i.qty + 1)">+</button>
            </div>
          </div>
          <button class="link cart-del" @click="cart.remove(i.id)">删除</button>
        </div>
      </article>
    </section>

    <div v-if="cart.items.value.length" class="cart-promo2">
      <span class="cart-promo2-tag">%</span>
      <span>跨店每满 200 减 30</span>
      <button class="link">去凑单</button>
    </div>

    <div v-if="cart.items.value.length" class="cart-settle">
      <label class="gcheck settle-all">
        <input type="checkbox" :checked="cart.allChecked.value" @change="cart.toggleAll($event.target.checked)" />
        <span class="gcheck-box"></span>
        <span class="settle-all-label">全选</span>
      </label>
      <div class="settle-sum">
        <div class="settle-total">
          <span class="muted small">合计</span>
          <span class="price big">¥{{ cart.checkedTotal.value.toFixed(2) }}</span>
        </div>
        <div class="muted small">已选 {{ checkedPieces }} 件</div>
      </div>
      <button
        class="settle-btn"
        :disabled="!checkedCount || busy"
        :title="checkedCount ? '' : '请先勾选商品'"
        @click="settle"
      >
        {{ busy ? '提交中…' : `去结算(${checkedCount})` }}
      </button>
    </div>

    <p v-if="error" class="error cart-tip">{{ error }}</p>
    <p v-if="done" class="notice cart-tip">
      下单成功，订单号 #{{ done.id }}，合计 ¥{{ done.total.toFixed(2) }}
      <RouterLink to="/orders" class="link">查看订单</RouterLink>
    </p>

    <p class="muted small cart-tip">结算需登录；购物车本身存在本机浏览器，支付为模拟，不接真实支付。</p>

    <MTabs />
  </div>
</template>
