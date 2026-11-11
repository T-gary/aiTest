<script setup>
import { computed, onBeforeUnmount, onMounted, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { api } from '../api'
import { cart } from '../cart'
import { mobileView } from '../viewmode'

const route = useRoute()
const router = useRouter()

const p = ref(null)
const loading = ref(true)
const error = ref('')
const tab = ref('基本信息')
const pcTab = ref('商品详情')
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

function buyNow() {
  addToCart()
  router.push('/checkout')
}

function step(dir) {
  if (!images.value.length) return
  const n = images.value.length
  activeImage.value = (activeImage.value + dir + n) % n
}

const oldPrice = computed(() => (p.value ? (p.value.price * 1.3).toFixed(2) : '0.00'))
</script>

<template>
  <div class="detail">
    <!-- ========== 移动版（保持原布局） ========== -->
    <div class="dt-m">
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
        <button class="dt-buy" @click="buyNow">立即购买</button>
      </div>
    </div>

    <!-- ========== PC 版（仿京东详情页） ========== -->
    <div class="dt-pc">
      <p v-if="error" class="error dt-err">{{ error }}</p>
      <p v-else-if="loading" class="muted dt-err">加载中…</p>

      <template v-else-if="p">
        <nav class="dt-crumb">
          <RouterLink to="/">首页</RouterLink>
          <span>›</span>
          <span>{{ p.category || '全部商品' }}</span>
          <span>›</span>
          <span class="cur">{{ p.name }}</span>
        </nav>

        <section class="dt-store">
          <div class="dt-store-logo">{{ (p.created_by || '极简').slice(0, 2) }}</div>
          <div class="dt-store-info">
            <b>{{ p.created_by || '极简商城' }} 的店铺</b>
            <span class="dt-store-stars">★★★★★ <em>4.9</em></span>
            <p>精选好物，快来逛逛</p>
          </div>
          <div class="dt-store-actions">
            <button>♡ 关注店铺</button>
            <button>联系客服</button>
          </div>
        </section>

        <div class="dt-pc-body">
          <!-- 左：竖排缩略图 -->
          <div v-if="images.length" class="dt-pc-thumbs">
            <button
              v-for="(src, i) in images"
              :key="'pc-thumb-' + i"
              class="dt-pc-thumb"
              :class="{ on: i === activeImage }"
              @click="activeImage = i"
            >
              <img :src="src" :alt="`图${i + 1}`" loading="lazy" />
            </button>
          </div>

          <!-- 中：主图 -->
          <div class="dt-pc-stage">
            <img v-if="images.length" :src="images[activeImage]" :alt="p.name" />
            <div v-else class="dt-pc-noimg">暂无图片</div>
            <button v-if="images.length > 1" class="dt-pc-arrow prev" aria-label="上一张" @click="step(-1)">‹</button>
            <button v-if="images.length > 1" class="dt-pc-arrow next" aria-label="下一张" @click="step(1)">›</button>
          </div>

          <!-- 右：购买面板 -->
          <aside class="dt-pc-buy">
            <div class="dt-pc-promo">11.11 <span>又好又便宜</span></div>

            <h1 class="dt-pc-name">{{ p.name }}</h1>

            <div class="dt-pc-pricerow">
              <div class="dt-pc-price"><span>¥</span><b>{{ p.price.toFixed(2) }}</b></div>
              <s class="dt-pc-old">¥{{ oldPrice }}</s>
              <span class="dt-pc-sales">累计评价 100+</span>
            </div>

            <div class="dt-pc-deal">
              <span>已享受：限时折上9.7折</span>
              <em>{{ countdown }}</em>
            </div>

            <div class="dt-pc-meta">
              <div class="dt-pc-metarow">
                <span class="lbl">送至</span>
                <div>全国 包邮<br /><b>预计今天发货，后天送达</b></div>
              </div>
              <div class="dt-pc-metarow">
                <span class="lbl">服务</span>
                <div>免费上门退换 · 破损包退换 · 30天价保 · 支持7天无理由退货</div>
              </div>
            </div>

            <div v-if="images.length" class="dt-pc-metarow dt-pc-skurow">
              <span class="lbl">款式</span>
              <div class="dt-pc-sku-list">
                <button
                  v-for="(src, i) in images"
                  :key="'pc-sku-' + i"
                  class="dt-pc-sku-item"
                  :class="{ on: i === activeImage }"
                  @click="activeImage = i"
                >
                  <img :src="src" :alt="`款式${i + 1}`" />
                  <span>{{ i === activeImage ? '已选' : '可选' }}</span>
                </button>
              </div>
            </div>

            <div class="dt-pc-metarow dt-pc-qtyrow">
              <span class="lbl">数量</span>
              <div class="dt-qty">
                <button :disabled="qty <= 1" @click="minus">−</button>
                <span>{{ qty }}</span>
                <button @click="plus">+</button>
              </div>
            </div>

            <div class="dt-pc-actions">
              <button class="dt-pc-cart" @click="addToCart">加入购物车</button>
              <button class="dt-pc-buy" @click="buyNow">立即购买</button>
            </div>
          </aside>
        </div>

        <!-- 下方：详情 / 售后 / 评价 -->
        <section class="dt-pc-detail">
          <nav class="dt-pc-tabs">
            <button :class="{ on: pcTab === '商品详情' }" @click="pcTab = '商品详情'">商品详情</button>
            <button :class="{ on: pcTab === '售后保障' }" @click="pcTab = '售后保障'">售后保障</button>
            <button :class="{ on: pcTab === '买家评价' }" @click="pcTab = '买家评价'">买家评价</button>
          </nav>

          <div v-if="pcTab === '商品详情'" class="dt-pc-pane">
            <p class="dt-pc-text">{{ p.description }}</p>
            <img
              v-for="(src, i) in images"
              :key="'pc-detail-' + i"
              :src="src"
              :alt="p.name"
              class="dt-pc-detail-img"
              loading="lazy"
            />
            <p class="dt-pc-endmeta">
              分类：{{ p.category || '未分类' }} · 录入人：{{ p.created_by }} · 上架时间：{{ p.created_at }}
            </p>
          </div>

          <div v-else-if="pcTab === '售后保障'" class="dt-pc-pane">
            <ul class="dt-pc-svc">
              <li><b>7天无理由退货</b><span>签收后 7 天内，不影响二次销售可退</span></li>
              <li><b>30天价保</b><span>下单后 30 天内降价，可申请补差</span></li>
              <li><b>免费上门退换</b><span>全国范围取件免费</span></li>
              <li><b>破损包退换</b><span>运输破损拍照即赔</span></li>
            </ul>
          </div>

          <div v-else class="dt-pc-pane dt-pc-empty">
            <p>暂无评价，成为第一个评价的人</p>
          </div>
        </section>
      </template>
    </div>

    <!-- PC 右侧悬浮工具栏 -->
    <aside class="dt-rail rail">
      <RouterLink to="/" class="railitem" title="首页">
        <span class="railicon">
          <svg viewBox="0 0 24 24" aria-hidden="true"><path d="M4 11l8-7 8 7" /><path d="M6 10v9h12v-9" /></svg>
        </span>
        <span class="raillabel">首页</span>
      </RouterLink>
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
      <RouterLink to="/profile" class="railitem" title="我的">
        <span class="railicon">
          <svg viewBox="0 0 24 24" aria-hidden="true"><circle cx="12" cy="12" r="8.5" /><path d="M8.5 13.5c1 1.4 2.3 1.4 3.5 1.4s2.5 0 3.5-1.4" /></svg>
        </span>
        <span class="raillabel">我的</span>
      </RouterLink>
      <button class="railitem" title="切换到移动版预览" @click="mobileView = true">
        <span class="railicon">
          <svg viewBox="0 0 24 24" aria-hidden="true"><rect x="7" y="2.5" width="10" height="19" rx="2" /><path d="M10.5 5h3M11 18.5h2" /></svg>
        </span>
        <span class="raillabel">移动版</span>
      </button>
    </aside>
  </div>
</template>
