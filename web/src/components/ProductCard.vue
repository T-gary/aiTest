<script setup>
import { cart } from '../cart'

defineProps({ p: { type: Object, required: true } })
</script>

<template>
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
      <button @click.stop="cart.add(p)">{{ cart.has(p.id) ? '再加一件' : '加入购物车' }}</button>
    </div>
  </div>
</template>
