<script setup>
import { computed, ref } from 'vue'
import { useRouter } from 'vue-router'
import { auth } from '../api'
import { cart } from '../cart'
import MTabs from '../components/MTabs.vue'

const router = useRouter()
const user = computed(() => auth.user())
const surveyOpen = ref(false)

const stats = [
  { n: '16 张', t: '优惠券', tag: '快过期' },
  { n: '538', t: '京豆' },
  { n: '¥1.7', t: '红包' },
  { n: '¥4600', t: '可提现' },
  { n: '抽¥20', t: '秒送', tag: '外卖' }
]

const quick = [
  { n: '51', t: '足迹' },
  { n: '8', t: '收藏' },
  { n: '2', t: '关注' },
  { n: '发现', t: '种草' }
]

const orders = [
  { icon: 'wallet', label: '待付款' },
  { icon: 'box', label: '待收货' },
  { icon: 'ticket', label: '待使用' },
  { icon: 'comment', label: '待评价', badge: 1 },
  { icon: 'refund', label: '退换/售后' },
  { icon: 'more', label: '全部' }
]

function goOrder() {
  router.push('/checkout')
}
</script>

<template>
  <div class="profile">
    <!-- 顶部 -->
    <header class="pf-top">
      <span class="pf-review">
        <em>点评</em>
        专业评测 · 真实体验
        <i>›</i>
      </span>
      <div class="pf-top-icons">
        <span>
          <svg viewBox="0 0 24 24"><path d="M5 13a7 7 0 0 1 14 0" /><rect x="3" y="13" width="3.5" height="5.5" rx="1.7" /><rect x="17.5" y="13" width="3.5" height="5.5" rx="1.7" /><path d="M19.2 18.5c0 1.6-1.6 2.5-3.7 2.5" /></svg>
          <b>客户服务</b>
        </span>
        <span>
          <svg viewBox="0 0 24 24"><path d="M12 21s7-6.2 7-11a7 7 0 1 0-14 0c0 4.8 7 11 7 11z" /><circle cx="12" cy="10" r="2.4" /></svg>
          <b>地址</b>
        </span>
        <span>
          <svg viewBox="0 0 24 24"><rect x="4" y="4" width="7" height="7" rx="1.5" /><rect x="13" y="4" width="7" height="7" rx="1.5" /><rect x="4" y="13" width="7" height="7" rx="1.5" /><rect x="13" y="13" width="7" height="7" rx="1.5" /></svg>
          <b>设置</b>
        </span>
      </div>
    </header>

    <!-- 用户卡 -->
    <section class="pf-user">
      <div class="pf-user-row">
        <span class="pf-avatar">{{ (user?.username || '客').slice(0, 1).toUpperCase() }}</span>
        <div class="pf-user-info">
          <div class="pf-username">{{ user?.username || '未登录' }}</div>
          <div class="pf-level">{{ user?.is_admin ? '管理员' : '普通会员' }}</div>
        </div>
        <RouterLink v-if="!user" to="/login" class="pf-login">登录 / 注册</RouterLink>
        <div v-else class="pf-plus">
          <em>PLUS</em>
          <div>
            <strong>领超级补贴</strong>
            <span>1505 元限时专享</span>
          </div>
        </div>
      </div>

      <div class="pf-stats">
        <div v-for="s in stats" :key="s.t" class="pf-stat">
          <i v-if="s.tag" class="pf-stat-tag">{{ s.tag }}</i>
          <b>{{ s.n }}</b>
          <span>{{ s.t }}</span>
        </div>
        <div class="pf-stat pf-stat-more">
          <b>更多</b>
          <span>›</span>
        </div>
      </div>

      <div class="pf-promos">
        <button class="pf-promo left" @click="router.push('/checkout')">
          <span class="pf-promo-art">￥</span>
          <span>
            <b>充值更便宜</b>
            <em>全球买手</em>
          </span>
        </button>
        <div class="pf-promo-mid">
          <b>11.11</b>
          <span>限时返场</span>
          <em>去逛逛 ›</em>
        </div>
        <button class="pf-promo right" @click="router.push('/checkout')">
          <span>
            <b>直降千元</b>
            <em>手机数码</em>
          </span>
          <span class="pf-promo-art">📱</span>
        </button>
      </div>

      <div class="pf-signin">
        <b>每日必领</b>
        <span>签到得积分，上限 66 分！</span>
        <button class="link">去领取 ›</button>
      </div>
    </section>

    <!-- 快捷数据 -->
    <section class="pf-quick">
      <div v-for="q in quick" :key="q.t" class="pf-quick-item">
        <span>{{ q.t }}</span>
        <em>{{ q.n }}</em>
      </div>
    </section>

    <!-- 订单入口 -->
    <section class="pf-orders">
      <button v-for="o in orders" :key="o.label" class="pf-order" @click="goOrder">
        <span class="pf-order-icon">
          <svg v-if="o.icon === 'wallet'" viewBox="0 0 24 24"><rect x="3" y="6" width="18" height="12" rx="2" /><path d="M3 10h18" /></svg>
          <svg v-else-if="o.icon === 'box'" viewBox="0 0 24 24"><path d="M12 3l8 4.5v9L12 21l-8-4.5v-9z" /><path d="M4 7.5l8 4.5 8-4.5M12 12v9" /></svg>
          <svg v-else-if="o.icon === 'ticket'" viewBox="0 0 24 24"><path d="M4 8h16v8H4z" /><path d="M9 8v8" /></svg>
          <svg v-else-if="o.icon === 'comment'" viewBox="0 0 24 24"><path d="M4 5.5h16v11H9l-5 4v-15z" /><path d="M8 10h8" /></svg>
          <svg v-else-if="o.icon === 'refund'" viewBox="0 0 24 24"><circle cx="12" cy="12" r="8.5" /><path d="M12 7.5v9M9.6 10.2h3.9a1.9 1.9 0 0 1 0 3.8H9.6" /></svg>
          <svg v-else viewBox="0 0 24 24"><circle cx="12" cy="12" r="8.5" /><path d="M9 12h6M12 9v6" /></svg>
          <i v-if="o.badge" class="pf-order-badge">{{ o.badge }}</i>
        </span>
        <span class="pf-order-label">{{ o.label }}</span>
      </button>
    </section>

    <!-- 三栏服务 -->
    <section class="pf-cols">
      <div class="pf-col">
        <h3>钱包 ›</h3>
        <p class="pf-col-sub"><em>银行卡</em>支付优惠</p>
        <div class="pf-kv"><b>1 个额度 <i class="dot">1</i></b><span>白条</span></div>
        <div class="pf-kv"><b>17100</b><span>金条借款</span></div>
        <div class="pf-kv"><b>¥860.8</b><span>黄金</span></div>
      </div>

      <div class="pf-col">
        <h3>上门服务 ›</h3>
        <p class="pf-col-sub"><em>官方</em>专属惊喜</p>
        <div class="pf-card green">
          <b>888 京豆</b>
          <span>24 点前领取 ›</span>
        </div>
        <div class="pf-kv"><b>京东快递</b></div>
        <div class="pf-kv"><b>京东互医</b></div>
      </div>

      <div class="pf-col">
        <h3>互动游戏 ›</h3>
        <p class="pf-col-sub"><em>海量</em>京豆免费拿</p>
        <div class="pf-card red">
          <b>最高 88 京豆</b>
          <span>24 点失效 ›</span>
        </div>
        <div class="pf-kv"><b>农场</b></div>
        <div class="pf-kv"><b>赚红包</b></div>
      </div>
    </section>

    <!-- 我的商品/管理入口 -->
    <section class="pf-links">
      <RouterLink :to="user ? '/admin' : '/login'" class="pf-link">
        <span>{{ user?.is_admin ? '进入后台管理' : '我的商品' }}</span>
        <em>›</em>
      </RouterLink>
      <RouterLink to="/messages" class="pf-link">
        <span>消息中心</span>
        <em>›</em>
      </RouterLink>
      <RouterLink to="/checkout" class="pf-link">
        <span>购物车（{{ cart.count.value }} 种商品）</span>
        <em>›</em>
      </RouterLink>
    </section>

    <MTabs />

    <!-- 参与调研浮层 -->
    <button class="pf-survey" aria-label="参与调研" @click="surveyOpen = !surveyOpen">
      <em>✦</em>
      参与<br />调研
    </button>
    <div v-if="surveyOpen" class="pf-survey-note">
      <span>暂无进行中的调研问卷</span>
      <button class="link" @click="surveyOpen = false">知道了</button>
    </div>
  </div>
</template>
