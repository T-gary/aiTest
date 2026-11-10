<script setup>
import { useRouter } from 'vue-router'
import { cart } from '../cart'

const router = useRouter()
</script>

<template>
  <div class="checkout">
    <div class="row" style="justify-content: space-between; margin-bottom: 14px">
      <h1 style="margin: 0">购物车</h1>
      <RouterLink to="/" class="link">继续购物</RouterLink>
    </div>

    <div v-if="!cart.items.value.length" class="card empty-cart">
      <p class="muted">购物车还是空的</p>
      <button @click="router.push('/')">去挑商品</button>
    </div>

    <template v-else>
      <div class="card">
        <table class="cart-table">
          <thead>
            <tr><th>商品</th><th>单价</th><th>数量</th><th>小计</th><th></th></tr>
          </thead>
          <tbody>
            <tr v-for="i in cart.items.value" :key="i.id">
              <td>
                <div class="cart-cell">
                  <img v-if="i.image_url" :src="i.image_url" :alt="i.name" />
                  <div v-else class="thumb-ph">无图</div>
                  <RouterLink to="/" class="cname">{{ i.name }}</RouterLink>
                </div>
              </td>
              <td class="muted">¥{{ i.price.toFixed(2) }}</td>
              <td>
                <div class="qty">
                  <button class="ghost" @click="cart.setQty(i.id, i.qty - 1)">−</button>
                  <span>{{ i.qty }}</span>
                  <button class="ghost" @click="cart.setQty(i.id, i.qty + 1)">+</button>
                </div>
              </td>
              <td class="price">¥{{ (i.price * i.qty).toFixed(2) }}</td>
              <td><button class="link" @click="cart.remove(i.id)">删除</button></td>
            </tr>
          </tbody>
        </table>
      </div>

      <div class="card sumbar">
        <div class="row" style="gap: 16px">
          <button class="link" @click="cart.clear()">清空购物车</button>
          <span class="muted">共 {{ cart.count.value }} 件</span>
        </div>
        <div class="row" style="gap: 16px">
          <span class="muted">合计</span>
          <span class="price big">¥{{ cart.total.value.toFixed(2) }}</span>
          <button disabled title="演示项目，不支持下单支付">去结算</button>
        </div>
      </div>
      <p class="muted small">演示项目：购物车只存在本机浏览器，不支持下单和支付。</p>
    </template>
  </div>
</template>
