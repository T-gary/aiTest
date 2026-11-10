<script setup>
import { onMounted, ref, watch } from 'vue'
import { api } from '../api'

const items = ref([])
const total = ref(0)
const page = ref(1)
const size = 12
const q = ref('')
const loading = ref(false)
const error = ref('')

async function load() {
  loading.value = true
  error.value = ''
  try {
    const params = new URLSearchParams({ page: page.value })
    if (q.value.trim()) params.set('q', q.value.trim())
    const data = await api(`/products?${params}`)
    items.value = data.items
    total.value = data.total
  } catch (e) {
    error.value = e.message
  } finally {
    loading.value = false
  }
}

watch(page, load)
onMounted(load)

const pages = ref(1)
watch(total, () => {
  pages.value = Math.max(1, Math.ceil(total.value / size))
})

function search() {
  if (page.value === 1) load()
  else page.value = 1
}
</script>

<template>
  <div>
    <div class="row" style="justify-content: space-between">
      <h1>商品</h1>
      <div class="row">
        <input v-model="q" placeholder="搜索商品名称" style="width: 200px" @keyup.enter="search" />
        <button class="ghost" @click="search">搜索</button>
      </div>
    </div>

    <p v-if="error" class="error">{{ error }}</p>
    <p v-else-if="loading" class="muted">加载中…</p>
    <p v-else-if="!items.length" class="muted">暂无商品</p>

    <div class="grid">
      <article v-for="p in items" :key="p.id" class="product">
        <img v-if="p.image_url" :src="p.image_url" :alt="p.name" loading="lazy" />
        <div v-else class="ph">暂无图片</div>
        <div class="body">
          <div class="name">{{ p.name }}</div>
          <div class="desc">{{ p.description }}</div>
          <div class="price">¥{{ p.price.toFixed(2) }}</div>
        </div>
      </article>
    </div>

    <div v-if="pages > 1" class="pager">
      <button class="ghost" :disabled="page <= 1" @click="page--">上一页</button>
      <span class="muted">{{ page }} / {{ pages }}</span>
      <button class="ghost" :disabled="page >= pages" @click="page++">下一页</button>
    </div>
  </div>
</template>
