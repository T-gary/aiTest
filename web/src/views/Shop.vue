<script setup>
import { computed, onBeforeUnmount, onMounted, ref } from 'vue'
import { api } from '../api'
import { cart } from '../cart'
import { mobileView } from '../viewmode'
import MTabs from '../components/MTabs.vue'
const items = ref([])
const total = ref(0)
const page = ref(1)
const size = ref(20)
const pages = ref(1)
const q = ref('')
const searchTerm = ref('')
const category = ref('')
const cats = ref([])
const featured = ref([])
const slide = ref(0)
const loading = ref(false)
const error = ref('')

let timer = 0
let clock = 0
let hintTimer = 0

// 搜索框 placeholder 轮播：优先用真实分类名，保证搜得到
const fallbackHints = ['水果', '家电', '键盘', '耳机']
const hints = computed(() => (cats.value.length ? cats.value.map((c) => c.name) : fallbackHints))
const hint = ref(0)

// 回到顶部
const showTop = ref(false)

function onScroll(e) {
  const t = e.target
  const y = t === document || t === window ? window.scrollY || 0 : t.scrollTop || 0
  showTop.value = y > 400
}

function backToTop() {
  const el = document.querySelector('.content')
  if (el && el.scrollHeight > el.clientHeight) el.scrollTo({ top: 0, behavior: 'smooth' })
  else window.scrollTo({ top: 0, behavior: 'smooth' })
}

async function loadProducts() {
  loading.value = true
  error.value = ''
  try {
    const p = new URLSearchParams({ page: page.value, size: size.value })
    if (searchTerm.value.trim()) p.set('q', searchTerm.value.trim())
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
  // 输入框为空：用当前轮播文字搜索；搜完清空输入框，方便连续点
  searchTerm.value = q.value.trim() || hints.value[hint.value % hints.value.length]
  q.value = ''
  page.value = 1
  loadProducts()
}

function pick(name) {
  category.value = name
  searchTerm.value = ''
  q.value = ''
  page.value = 1
  loadProducts()
}

function go(p) {
  page.value = p
  loadProducts()
}

function scrollToCats() {
  document.querySelector('.m-cats')?.scrollIntoView({ behavior: 'smooth', block: 'start' })
}

const current = computed(() => featured.value[slide.value % (featured.value.length || 1)])

// 秒杀倒计时：整点结束，仅展示
const now = ref(Date.now())
const flashTime = computed(() => {
  const d = new Date(now.value)
  const left = 3600000 - (d.getMinutes() * 60000 + d.getSeconds() * 1000 + d.getMilliseconds())
  const m = String(Math.floor(left / 60000)).padStart(2, '0')
  const s = String(Math.floor((left % 60000) / 1000)).padStart(2, '0')
  return `00:${m}:${s}`
})

onMounted(() => {
  loadShell()
  loadProducts()
  timer = setInterval(() => {
    if (featured.value.length > 1) slide.value = (slide.value + 1) % featured.value.length
  }, 4000)
  clock = setInterval(() => (now.value = Date.now()), 1000)
  hintTimer = setInterval(() => (hint.value = (hint.value + 1) % hints.value.length), 3000)

  const scroller = document.querySelector('.content')
  scroller?.addEventListener('scroll', onScroll, { passive: true })
  window.addEventListener('scroll', onScroll, { passive: true })
})

onBeforeUnmount(() => {
  clearInterval(timer)
  clearInterval(clock)
  clearInterval(hintTimer)
  const scroller = document.querySelector('.content')
  scroller?.removeEventListener('scroll', onScroll)
  window.removeEventListener('scroll', onScroll)
})
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

    <!-- 移动版头部：定位 + 搜索 + 扫码 -->
    <div class="m-head">
      <span class="m-loc">北京<em>⌄</em></span>
      <div class="m-search">
        <input v-model="q" :placeholder="hints[hint % hints.length]" @keyup.enter="search" />

        <button @click="search" aria-label="搜索">
          <svg viewBox="0 0 24 24"><circle cx="11" cy="11" r="6.5" /><path d="M16 16l4.5 4.5" /></svg>
        </button>
      </div>
    </div>

    <!-- 移动版服务条 -->
    <div class="m-service">
      <span>免费包送</span>
      <span>严格安检</span>
      <span>包退包赔</span>
    </div>

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

      <!-- 移动版分类宫格 -->
      <div class="m-cats">
        <button :class="{ on: !category }" @click="pick('')">
          <span class="m-cat-icon all">全</span>
          <span class="m-cat-label">全部</span>
        </button>
        <button v-for="(c, i) in cats" :key="c.name" :class="{ on: category === c.name }" @click="pick(c.name)">
          <span class="m-cat-icon" :class="'tone-' + (i % 6)">{{ c.name.slice(0, 1) }}</span>
          <span class="m-cat-label">{{ c.name }}</span>
        </button>
      </div>

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

        <!-- 移动版限时秒杀（取轮播商品横滑） -->
        <div v-if="featured.length" class="m-flash">
          <div class="m-sec-head">
            <h3>限时秒杀</h3>
            <span class="m-timer">仅剩 <em>{{ flashTime }}</em></span>
          </div>
          <div class="m-flash-row">
            <button v-for="f in featured" :key="f.id" class="m-flash-item" @click="cart.add(f)">
              <img v-if="f.image_url" :src="f.image_url" :alt="f.name" loading="lazy" />
              <div v-else class="thumb-ph">暂无图片</div>
              <span class="m-flash-name">{{ f.name }}</span>
              <span class="price">¥{{ f.price.toFixed(2) }}</span>
            </button>
          </div>
        </div>

        <div class="listhead">
          <h1>{{ searchTerm || category || '今日上新' }}</h1>
          <span class="muted">共 {{ total }} 件</span>
        </div>

        <p v-if="loading" class="muted">加载中…</p>
        <p v-else-if="!items.length" class="muted">暂无商品</p>

        <ul v-else class="plist">
          <li v-for="p in items" :key="p.id">
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
                <button @click="cart.add(p)">{{ cart.has(p.id) ? '再加一件' : '加入购物车' }}</button>
              </div>
            </div>
          </li>
        </ul>

        <div v-if="pages > 1" class="pager">
          <button class="ghost" :disabled="page <= 1" @click="go(page - 1)">上一页</button>
          <span class="muted">{{ page }} / {{ pages }}</span>
          <button class="ghost" :disabled="page >= pages" @click="go(page + 1)">下一页</button>
        </div>
      </section>

      <aside class="rail">
        <RouterLink to="/checkout" class="railitem" title="购物车">
          <span class="railicon">
            <svg viewBox="0 0 24 24" aria-hidden="true">
              <path d="M3 4h2.2l2 10.5h10.2L20 7H6.1" />
              <circle cx="9.5" cy="19" r="1.6" />
              <circle cx="17" cy="19" r="1.6" />
            </svg>
            <em v-if="cart.count.value" class="badge">{{ cart.count.value > 99 ? '99+' : cart.count.value }}</em>
          </span>
          <span class="raillabel">购物车</span>
        </RouterLink>
        <span class="railitem off">
          <span class="railicon">
            <svg viewBox="0 0 24 24" aria-hidden="true"><circle cx="12" cy="12" r="8.5" /><path d="M8.5 13.5c1 1.4 2.3 1.4 3.5 1.4s2.5 0 3.5-1.4" /></svg>
          </span>
          <span class="raillabel">我的</span>
        </span>
        <span class="railitem off">
          <span class="railicon">
            <svg viewBox="0 0 24 24" aria-hidden="true"><path d="M5 13a7 7 0 0 1 14 0" /><rect x="3" y="13" width="3.5" height="5.5" rx="1.7" /><rect x="17.5" y="13" width="3.5" height="5.5" rx="1.7" /><path d="M19.2 18.5c0 1.6-1.6 2.5-3.7 2.5" /></svg>
          </span>
          <span class="raillabel">客服</span>
        </span>
        <button class="railitem" title="切换到移动版预览" @click="mobileView = true">
          <span class="railicon">
            <svg viewBox="0 0 24 24" aria-hidden="true"><rect x="7" y="2.5" width="10" height="19" rx="2" /><path d="M10.5 5h3M11 18.5h2" /></svg>
          </span>
          <span class="raillabel">移动版</span>
        </button>
        <span class="railitem off">
          <span class="railicon">
            <svg viewBox="0 0 24 24" aria-hidden="true"><path d="M8 4h8v3.5h3.5V14a2 2 0 1 1 0 4v2H8v-2.5a2 2 0 1 1 0-4V4z" /></svg>
          </span>
          <span class="raillabel">插件版</span>
        </span>
        <span class="railitem off">
          <span class="railicon">
            <svg viewBox="0 0 24 24" aria-hidden="true"><rect x="3.5" y="4" width="17" height="16" rx="2" /><path d="M7.5 15.5l5-5 2.5 2.5-5 5H7.5v-2.5z" /></svg>
          </span>
          <span class="raillabel">反馈</span>
        </span>
        <span class="railitem off">
          <span class="railicon">
            <svg viewBox="0 0 24 24" aria-hidden="true"><rect x="4" y="3.5" width="16" height="17" rx="2" /><path d="M8 8.5h8M8 12.5h8M8 16.5h5" /></svg>
          </span>
          <span class="raillabel">调研</span>
        </span>
      </aside>
    </div>

    <!-- 移动版底部导航 -->
    <MTabs />

    <!-- 回到顶部 -->
    <button v-show="showTop" class="backtop" aria-label="回到顶部" @click="backToTop">
      <svg viewBox="0 0 24 24"><path d="M12 19V6M6 11.5L12 5.5l6 6" /></svg>
    </button>
  </div>
</template>
