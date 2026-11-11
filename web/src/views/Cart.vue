<script setup>
import { computed, ref } from 'vue'
import { cart } from '../cart'

const cfg = ref({ discount: 66.82 })

const checkedCount = computed(() => cart.checkedCount.value)
const checkedPieces = computed(() => cart.checkedPieces.value)
const checkedTotal = computed(() => cart.checkedTotal.value)
const totalAll = computed(() => cart.total.value)

function removeChecked() {
  cart.items.value.filter((i) => i.checked).forEach((i) => cart.remove(i.id))
}

function clearAll() {
  if (confirm('确定清空购物车？')) cart.clear()
}
</script>

<template>
  <div class="dcart">
    <!-- 顶部 -->
    <header class="dc-head">
      <div class="dc-brand">
        <RouterLink to="/" class="dc-logo">极简商城</RouterLink>
        <em>购物车 <span>({{ cart.kinds.value }})</span></em>
      </div>
      <div class="dc-deliver">配送至 <b>北京 朝阳区 慧谷阳光100号楼</b></div>
      <div class="dc-search">
        <input placeholder="搜索商品" />
        <button>搜索</button>
      </div>
    </header>

    <div class="dc-coupon">
      <span class="dc-coupon-icon">￥</span>
      <span>惊喜红包 4 张 最高省 77 元待使用</span>
    </div>

    <!-- 工具栏 -->
    <div class="dc-toolbar">
      <label class="dcheck">
        <input type="checkbox" :checked="cart.allChecked.value" @change="cart.toggleAll($event.target.checked)" />
        <span class="dcheck-box"></span>
        <span class="dcheck-label">全选</span>
      </label>
      <button class="ghost" @click="removeChecked">删除</button>
      <button class="ghost">移入收藏</button>
      <button class="ghost" @click="clearAll">清理购物车</button>
      <div class="dc-toolbar-right">
        <span>🕘 刚看过</span>
        <span>☆ 刚收藏</span>
        <span>📄 刚买过</span>
      </div>
    </div>

    <p v-if="!cart.items.value.length" class="dc-empty">
      购物车还是空的，<RouterLink to="/" class="link">去挑商品</RouterLink>
    </p>

    <template v-else>
      <!-- 店铺 -->
      <section class="dc-store">
        <header class="dc-store-head">
          <label class="dcheck">
            <input type="checkbox" :checked="cart.allChecked.value" @change="cart.toggleAll($event.target.checked)" />
            <span class="dcheck-box"></span>
          </label>
          <span class="dc-store-name">极简商城自营官方店</span>
        </header>

        <article v-for="i in cart.items.value" :key="i.id" class="dc-row">
          <label class="dcheck">
            <input type="checkbox" :checked="i.checked" @change="cart.toggle(i.id)" />
            <span class="dcheck-box"></span>
          </label>

          <img v-if="i.image_url" :src="i.image_url" :alt="i.name" class="dc-thumb" />
          <div v-else class="dc-thumb dc-thumb-ph">无图</div>

          <div class="dc-name">
            <p class="dc-title">{{ i.name }}</p>
            <p class="dc-spec">商品　{{ i.name }}</p>
            <p class="dc-spec">服务　可选择增值服务 ⋎</p>
          </div>

          <div class="dc-price">
            <div class="dc-price-main">¥{{ i.price.toFixed(2) }} <em>到手价</em></div>
            <div class="dc-price-old">¥{{ (i.price * 1.11).toFixed(1) }}</div>
            <div class="dc-price-tip">已享单品满 1 件 9 折</div>
          </div>

          <div class="dc-qty">
            <button :disabled="i.qty <= 1" @click="cart.setQty(i.id, i.qty - 1)">−</button>
            <span>{{ i.qty }}</span>
            <button @click="cart.setQty(i.id, i.qty + 1)">+</button>
          </div>

          <div class="dc-actions">
            <button class="link">移入收藏</button>
            <button class="link" @click="cart.remove(i.id)">删除</button>
          </div>
        </article>
      </section>

      <!-- 底部结算 -->
      <footer class="dc-settle">
        <div class="dc-settle-left">
          <span class="muted">已选 {{ checkedCount }} 种 / {{ checkedPieces }} 件</span>
          <button class="link" @click="cart.clear()">清空</button>
        </div>
        <div class="dc-settle-right">
          <div class="dc-sum">
            <div><span>商品总额：</span>¥{{ totalAll.toFixed(2) }}</div>
            <div><span>共减：</span><em>¥{{ cfg.discount.toFixed(2) }}</em></div>
            <div class="muted small">优惠明细</div>
          </div>
          <div class="dc-total">
            <span>合计：</span><b>¥{{ checkedTotal.toFixed(2) }}</b>
          </div>
          <button class="dc-settle-btn" disabled title="演示项目，不支持下单支付">去结算（{{ checkedCount }}）</button>
        </div>
      </footer>
    </template>
  </div>
</template>
