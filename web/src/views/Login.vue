<script setup>
import { computed, onMounted, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { api, auth } from '../api'

const route = useRoute()
const router = useRouter()

const role = ref('personal')
const mode = ref('login')
const registering = ref(false)
const username = ref('')
const password = ref('')
const password2 = ref('')
const remember = ref(true)
const showPass = ref(false)
const busy = ref(false)
const error = ref('')
const message = ref('')

// 输入框记忆：只存在本机浏览器
// shortcut: 明文保存密码，仅适用于演示项目；接真实账号前应改为只记账号名 + 浏览器密码管理
const KEY = 'login_saved'
onMounted(() => {
  try {
    const s = JSON.parse(localStorage.getItem(KEY) || 'null')
    if (!s) return
    username.value = s.u || ''
    password.value = s.p || ''
    remember.value = s.remember !== false
  } catch {}
})

function rememberInput() {
  localStorage.setItem(
    KEY,
    JSON.stringify({ u: username.value, p: remember.value ? password.value : '', remember: remember.value })
  )
}

// 演示用二维码：25×25 伪随机点阵，不做真实扫码
const qrSvg = ref('')
const qrValid = ref(true)
let qrTimer = null
function refreshQr() {
  let seed = (Date.now() % 100000) + 1
  const rnd = () => (seed = (seed * 1103515245 + 12345) % 2147483648) / 2147483648
  const n = 25
  let cells = ''
  const inFinder = (r, c) => (r < 7 && c < 7) || (r < 7 && c >= n - 7) || (r >= n - 7 && c < 7)
  for (let r = 0; r < n; r++) {
    for (let c = 0; c < n; c++) {
      if (rnd() > 0.52 && !inFinder(r, c)) cells += `<rect x="${c}" y="${r}" width="1" height="1"/>`
    }
  }
  const finder = (x, y) =>
    `<rect x="${x}" y="${y}" width="7" height="7"/><rect x="${x + 1}" y="${y + 1}" width="5" height="5" fill="#fff"/>` +
    `<rect x="${x + 2}" y="${y + 2}" width="3" height="3"/>`
  qrSvg.value = cells + finder(0, 0) + finder(n - 7, 0) + finder(0, n - 7)
  qrValid.value = true
  clearTimeout(qrTimer)
  qrTimer = setTimeout(() => (qrValid.value = false), 12000)
}

function notify(text) {
  error.value = ''
  message.value = text
}

async function submit() {
  error.value = ''
  message.value = ''
  if (!username.value || !password.value) {
    error.value = '请输入账号和密码'
    return
  }
  busy.value = true
  try {
    const r = await api('/login', {
      method: 'POST',
      body: JSON.stringify({ username: username.value, password: password.value })
    })
    auth.save(r)
    rememberInput()
    const back = typeof route.query.redirect === 'string' ? route.query.redirect : '/admin'
    router.push(back)
  } catch (e) {
    error.value = e.message
  } finally {
    busy.value = false
  }
}

async function submitRegister() {
  error.value = ''
  message.value = ''
  if (password.value !== password2.value) {
    error.value = '两次输入的密码不一致'
    return
  }
  busy.value = true
  try {
    const r = await api('/register', {
      method: 'POST',
      body: JSON.stringify({ username: username.value, password: password.value })
    })
    message.value = r.message
    registering.value = false
    mode.value = 'login'
    password2.value = ''
  } catch (e) {
    error.value = e.message
  } finally {
    busy.value = false
  }
}

onMounted(refreshQr)
const tabTitle = computed(() => (role.value === 'personal' ? '个人用户登录' : '企业用户登录'))
</script>

<template>
  <div class="lg">
    <header class="lg-top">
      <RouterLink to="/" class="lg-logo">极简商城</RouterLink>
      <button class="lg-suggest" @click="notify('改进建议请发给项目管理员，感谢反馈。')">
        <svg viewBox="0 0 24 24" aria-hidden="true"><circle cx="12" cy="12" r="9" /><path d="M12 10.5v6M12 7.6v.2" /></svg>
        登录页面，改进建议
      </button>
    </header>

    <div class="lg-shell">
      <nav class="lg-tabs" :aria-label="tabTitle">
        <button :class="{ on: role === 'personal' }" @click="role = 'personal'">个人用户登录</button>
        <button :class="{ on: role === 'biz' }" @click="role = 'biz'">企业用户登录</button>
      </nav>

      <section class="lg-card">
        <div class="lg-qr">
          <h2>手机扫码安全登录</h2>
          <p class="lg-qr-hint">支持 <b>极简商城 APP</b> · 微信扫码</p>
          <div class="lg-qr-box">
            <svg class="lg-qr-svg" viewBox="0 0 25 25" aria-hidden="true" v-html="qrSvg"></svg>
            <div v-if="!qrValid" class="lg-qr-mask">
              <span>二维码已失效</span>
              <button @click="refreshQr">刷新</button>
            </div>
          </div>
          <p class="lg-qr-tip">演示二维码，扫码登录暂未接入</p>
        </div>

        <div class="lg-form">
          <div v-if="role === 'biz'" class="lg-blocked">
            <p>企业用户登录暂未开放。</p>
            <button class="link" @click="role = 'personal'">返回个人用户登录</button>
          </div>

          <template v-else-if="!registering">
            <div class="lg-switch">
              <button :class="{ on: mode === 'login' }" @click="mode = 'login'">密码登录</button>
              <button :class="{ on: mode === 'sms' }" @click="mode = 'sms'">短信登录</button>
            </div>

            <form v-if="mode === 'login'" class="lg-fields" @submit.prevent="submit">
              <input v-model.trim="username" placeholder="账号名" autocomplete="username" />
              <div class="lg-pass">
                <input
                  v-model="password"
                  :type="showPass ? 'text' : 'password'"
                  placeholder="密码"
                  autocomplete="current-password"
                />
                <button
                  type="button"
                  class="lg-eye"
                  :aria-label="showPass ? '隐藏密码' : '显示密码'"
                  @click="showPass = !showPass"
                >
                  <svg viewBox="0 0 24 24" aria-hidden="true">
                    <path d="M2.5 12s3.5-6 9.5-6 9.5 6 9.5 6-3.5 6-9.5 6-9.5-6-9.5-6z" />
                    <circle cx="12" cy="12" r="2.6" />
                    <path v-if="!showPass" d="M4 20 20 4" />
                  </svg>
                </button>
              </div>
              <label class="lg-remember">
                <input v-model="remember" type="checkbox" />
                <span>记住账号密码（仅存本机）</span>
              </label>
              <button class="lg-submit" :disabled="busy">{{ busy ? '登录中…' : '登录' }}</button>

              <p v-if="error" class="error">{{ error }}</p>
              <p v-if="message" class="notice">{{ message }}</p>

              <div class="lg-links">
                <button type="button" @click="notify('微信登录暂未开放。')">微信登录</button>
                <i>|</i>
                <button type="button" @click="notify('QQ 登录暂未开放。')">QQ 登录</button>
                <i>|</i>
                <button type="button" @click="notify('暂不支持自助找回密码，请联系管理员重置。')">忘记密码</button>
                <i>|</i>
                <button type="button" @click="registering = true; error = ''; message = ''">立即注册</button>
              </div>
            </form>

            <div v-else class="lg-blocked">
              <p>短信登录暂未开放，请使用密码登录。</p>
              <button class="link" @click="mode = 'login'">去密码登录</button>
            </div>
          </template>

          <template v-else>
            <h2 class="lg-register-title">注册新账号</h2>
            <p class="lg-register-tip">注册后需管理员审核，审核通过才能登录。</p>
            <form class="lg-fields" @submit.prevent="submitRegister">
              <input v-model.trim="username" placeholder="用户名（2-50 个字符）" autocomplete="username" />
              <input v-model="password" type="password" placeholder="密码（至少 6 位）" autocomplete="new-password" />
              <input v-model="password2" type="password" placeholder="确认密码" autocomplete="new-password" />
              <button class="lg-submit" :disabled="busy">{{ busy ? '提交中…' : '提交注册' }}</button>
              <p v-if="error" class="error">{{ error }}</p>
              <p v-if="message" class="notice">{{ message }}</p>
              <div class="lg-links">
                <button type="button" @click="registering = false; error = ''; message = ''">返回登录</button>
              </div>
            </form>
          </template>
        </div>
      </section>

      <footer class="lg-foot">
        <p>
          <span>关于我们</span><i>|</i><span>联系我们</span><i>|</i><span>人才招聘</span><i>|</i><span>商家入驻</span>
          <i>|</i><span>广告服务</span><i>|</i><span>手机商城</span><i>|</i><span>友情链接</span><i>|</i><span>销售联盟</span>
        </p>
        <p>Copyright © 2026 极简商城 版权所有</p>
      </footer>
    </div>
  </div>
</template>
