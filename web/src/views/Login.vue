<script setup>
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { api, auth } from '../api'

const router = useRouter()
const mode = ref('login')
const username = ref('')
const password = ref('')
const message = ref('')
const error = ref('')
const busy = ref(false)

async function submit() {
  error.value = ''
  message.value = ''
  busy.value = true
  try {
    if (mode.value === 'login') {
      const r = await api('/login', {
        method: 'POST',
        body: JSON.stringify({ username: username.value, password: password.value })
      })
      auth.save(r)
      router.push('/admin')
    } else {
      const r = await api('/register', {
        method: 'POST',
        body: JSON.stringify({ username: username.value, password: password.value })
      })
      message.value = r.message
      mode.value = 'login'
    }
  } catch (e) {
    error.value = e.message
  } finally {
    busy.value = false
  }
}
</script>

<template>
  <div>
    <h1>{{ mode === 'login' ? '登录' : '注册' }}</h1>
    <div class="card form">
      <input v-model="username" placeholder="用户名" autocomplete="username" />
      <input v-model="password" type="password" placeholder="密码（至少 6 位）" autocomplete="current-password" @keyup.enter="submit" />
      <button :disabled="busy" @click="submit">{{ busy ? '请稍候…' : mode === 'login' ? '登录' : '提交注册' }}</button>
      <p v-if="error" class="error">{{ error }}</p>
      <p v-if="message" class="notice">{{ message }}</p>
      <button class="link" @click="mode = mode === 'login' ? 'register' : 'login'">
        {{ mode === 'login' ? '没有账号？注册（需人工审核）' : '已有账号？去登录' }}
      </button>
    </div>
  </div>
</template>
