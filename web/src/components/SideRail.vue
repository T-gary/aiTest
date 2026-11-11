<script setup>
import { cart } from '../cart'
import { mobileView } from '../viewmode'
</script>

<template>
  <aside class="rail">
    <RouterLink to="/" class="railitem" title="首页">
      <span class="railicon">
        <svg viewBox="0 0 24 24" aria-hidden="true"><path d="M4 11l8-7 8 7v8.5H4V11z" /></svg>
      </span>
      <span class="raillabel">首页</span>
    </RouterLink>
    <RouterLink to="/cart" class="railitem" title="购物车">
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
</template>
