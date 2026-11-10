<script setup>
import { computed, onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'
import { api, auth } from '../api'

const router = useRouter()
const user = ref(auth.user())
const tab = ref('products')
const error = ref('')
const message = ref('')

const stats = ref(null)
const products = ref([])
const page = ref(1)
const pages = ref(1)
const total = ref(0)
const pageSize = 12

const form = ref({ name: '', price: '', category: '', image_url: '', description: '', featured: false })
const busy = ref(false)

const tabs = computed(() => {
  const t = [{ id: 'products', label: '商品' }]
  if (user.value?.is_admin) {
    t.unshift({ id: 'board', label: '看板' })
    t.push({ id: 'accounts', label: '账号审核' })
  }
  return t
})

function fail(e) {
  error.value = e.message
  if (/未登录|登录已过期/.test(e.message)) router.push('/login')
}

async function loadProducts() {
  try {
    const data = await api(`/products?page=${page.value}`)
    const all = data.items
    products.value = all
    total.value = data.total
    pages.value = Math.max(1, Math.ceil(data.total / pageSize))
    if (user.value?.is_admin) await loadStats()
  } catch (e) {
    fail(e)
  }
}

async function loadStats() {
  try {
    stats.value = await api('/admin/stats')
  } catch (e) {
    fail(e)
  }
}

async function submitProduct() {
  error.value = ''
  message.value = ''
  busy.value = true
  try {
    await api('/products', {
      method: 'POST',
      body: JSON.stringify({
        name: form.value.name,
        price: Number(form.value.price),
        category: form.value.category,
        image_url: form.value.image_url,
        description: form.value.description,
        featured: form.value.featured
      })
    })
    message.value = '商品已上架'
    form.value = { name: '', price: '', category: '', image_url: '', description: '', featured: false }
    page.value = 1
    await loadProducts()
  } catch (e) {
    fail(e)
  } finally {
    busy.value = false
  }
}

async function removeProduct(p) {
  if (!confirm(`删除商品「${p.name}」？`)) return
  try {
    await api(`/products/${p.id}`, { method: 'DELETE' })
    await loadProducts()
  } catch (e) {
    fail(e)
  }
}

async function toggleFeatured(p) {
  try {
    await api(`/admin/products/${p.id}/featured`, { method: 'POST' })
    await loadProducts()
  } catch (e) {
    fail(e)
  }
}

async function approve(account) {
  try {
    await api(`/admin/accounts/${account.id}/approve`, { method: 'POST' })
    message.value = `已通过 ${account.username} 的审核`
    await loadStats()
  } catch (e) {
    fail(e)
  }
}

function goPage(p) {
  page.value = p
  loadProducts()
}

onMounted(() => {
  if (!auth.token()) return router.push('/login')
  user.value = auth.user()
  if (user.value?.is_admin) tab.value = 'board'
  loadProducts()
})
</script>

<template>
  <div>
    <div class="row" style="justify-content: space-between">
      <h1>后台</h1>
      <div class="row">
        <button v-for="t in tabs" :key="t.id" :class="tab === t.id ? '' : 'ghost'" @click="tab = t.id">
          {{ t.label }}
        </button>
      </div>
    </div>

    <p v-if="error" class="error">{{ error }}</p>
    <p v-if="message" class="notice">{{ message }}</p>

    <section v-if="tab === 'board' && stats">
      <div class="stats">
        <div class="stat"><div class="n">{{ stats.total_products }}</div><div class="t">商品总数</div></div>
        <div class="stat"><div class="n">{{ stats.total_accounts }}</div><div class="t">账号总数</div></div>
        <div class="stat"><div class="n">{{ stats.total_logins }}</div><div class="t">累计登录次数</div></div>
      </div>

      <h2>账号登录与上架情况</h2>
      <div class="card">
        <table>
          <thead>
            <tr><th>账号</th><th>状态</th><th>登录次数</th><th>最近登录</th><th>上架产品</th></tr>
          </thead>
          <tbody>
            <tr v-for="a in stats.accounts" :key="a.id">
              <td>{{ a.username }}<span v-if="a.is_admin" class="muted">（管理员）</span></td>
              <td>{{ a.status === 'approved' ? '已通过' : a.status === 'pending' ? '待审核' : '已拒绝' }}</td>
              <td>{{ a.login_count }}</td>
              <td>{{ a.last_login || '—' }}</td>
              <td>{{ a.product_count }}</td>
            </tr>
          </tbody>
        </table>
      </div>

      <h2>最近登录记录</h2>
      <div class="card">
        <p v-if="!stats.recent_logins.length" class="muted">暂无登录记录</p>
        <table v-else>
          <thead><tr><th>账号</th><th>时间</th></tr></thead>
          <tbody>
            <tr v-for="(l, i) in stats.recent_logins" :key="i"><td>{{ l.username }}</td><td>{{ l.at }}</td></tr>
          </tbody>
        </table>
      </div>
    </section>

    <section v-if="tab === 'products'">
      <div class="card form" style="max-width: none">
        <h2 style="margin: 0">录入商品</h2>
        <div class="row">
          <input v-model="form.name" placeholder="商品名称" style="flex: 1; min-width: 160px" />
          <input v-model="form.price" type="number" min="0" step="0.01" placeholder="价格" style="width: 130px" />
        </div>
        <div class="row">
          <input v-model="form.category" placeholder="分类（如 数码 / 家电，可留空）" style="flex: 1; min-width: 160px" />
          <label class="row muted" style="gap: 6px; font-size: 13px">
            <input v-model="form.featured" type="checkbox" style="width: auto" />
            首页轮播
          </label>
        </div>
        <input v-model="form.image_url" placeholder="图片外链 URL（可留空）" />
        <textarea v-model="form.description" placeholder="商品描述"></textarea>
        <div class="row">
          <button :disabled="busy" @click="submitProduct">上架</button>
          <span class="muted">商品会立即出现在前台首页</span>
        </div>
      </div>

      <h2>已上架商品</h2>
      <div class="card">
        <p v-if="!products.length" class="muted">还没有商品</p>
        <table v-else>
          <thead>
            <tr><th>名称</th><th>价格</th><th>分类</th><th>轮播</th><th>录入人</th><th>时间</th><th></th></tr>
          </thead>
          <tbody>
            <tr v-for="p in products" :key="p.id">
              <td>{{ p.name }}</td>
              <td>¥{{ p.price.toFixed(2) }}</td>
              <td>{{ p.category || '—' }}</td>
              <td>
                <button v-if="user?.is_admin" class="link" @click="toggleFeatured(p)">
                  {{ p.featured ? '已上轮播' : '未上轮播' }}
                </button>
                <span v-else>{{ p.featured ? '是' : '否' }}</span>
              </td>
              <td>{{ p.created_by }}</td>
              <td>{{ p.created_at }}</td>
              <td>
                <button v-if="user?.is_admin || p.created_by === user?.username" class="link" @click="removeProduct(p)">删除</button>
              </td>
            </tr>
          </tbody>
        </table>
        <div v-if="pages > 1" class="pager">
          <button class="ghost" :disabled="page <= 1" @click="goPage(page - 1)">上一页</button>
          <span class="muted">{{ page }} / {{ pages }}（共 {{ total }} 件）</span>
          <button class="ghost" :disabled="page >= pages" @click="goPage(page + 1)">下一页</button>
        </div>
      </div>
    </section>

    <section v-if="tab === 'accounts' && stats">
      <h2>账号管理</h2>
      <div class="card">
        <table>
          <thead>
            <tr><th>账号</th><th>状态</th><th>注册时间</th><th>上架产品</th><th></th></tr>
          </thead>
          <tbody>
            <tr v-for="a in stats.accounts" :key="a.id">
              <td>{{ a.username }}</td>
              <td>{{ a.status === 'approved' ? '已通过' : a.status === 'pending' ? '待审核' : '已拒绝' }}</td>
              <td>{{ a.created_at }}</td>
              <td>{{ a.product_count }}</td>
              <td>
                <button v-if="a.status === 'pending'" @click="approve(a)">审核通过</button>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </section>
  </div>
</template>
