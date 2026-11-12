const base = 'http://127.0.0.1:5050/api'
const j = async (path, opt = {}, token) => {
  const headers = { 'Content-Type': 'application/json' }
  if (token) headers.Authorization = `Bearer ${token}`
  const r = await fetch(base + path, { ...opt, headers })
  const d = await r.json().catch(() => ({}))
  return { status: r.status, d }
}
const post = (p, b, t) => j(p, { method: 'POST', body: JSON.stringify(b) }, t)
const assert = (ok, msg) => {
  if (!ok) throw new Error('FAIL: ' + msg)
  console.log('ok -', msg)
}

const admin = await post('/login', { username: 'admin', password: '123456' })
assert(admin.status === 200 && admin.d.is_admin, '管理员登录')

const name = 'tester' + Math.floor(Math.random() * 99999)
await post('/register', { username: name, password: 'abc123' })
const pending = await post('/login', { username: name, password: 'abc123' })
assert(pending.status === 403, '未审核账号被拒绝登录')

const stats = await j('/admin/stats', {}, admin.d.token)
const acct = stats.d.accounts.find(a => a.username === name)
assert(acct?.status === 'pending', '看板显示待审核')

await post(`/admin/accounts/${acct.id}/approve`, {}, admin.d.token)
const user = await post('/login', { username: name, password: 'abc123' })
assert(user.status === 200 && user.d.is_admin === false, '审核后可登录')

const created = await post('/products', { name: '中文测试商品', price: 19.99, image_url: 'https://example.com/a.png', description: '描述' }, user.d.token)
assert(created.status === 200, '录入商品')

const list = await j('/products?page=1')
const item = list.d.items.find(p => p.id === created.d.id)
assert(item?.name === '中文测试商品' && item.price === 19.99, '列表回显中文与价格')

const search = await j('/products?q=' + encodeURIComponent('中文'))
assert(search.d.items.some(p => p.id === created.d.id), '按名称搜索命中')

const denied = await j('/admin/stats', {}, user.d.token)
assert(denied.status === 403, '普通账号不能查看板')

const order = await post('/orders', { items: [{ id: created.d.id, qty: 2 }] }, user.d.token)
assert(order.status === 200 && order.d.total === 39.98, '下单成功，金额以库里价格为准')

const orders = await j('/orders', {}, user.d.token)
assert(orders.d.items[0]?.id === order.d.id && orders.d.items[0].items[0].qty === 2, '订单列表回显明细')

const orderNoAuth = await post('/orders', { items: [{ id: created.d.id, qty: 1 }] })
assert(orderNoAuth.status === 401, '未登录不能下单')

const del = await j(`/products/${created.d.id}`, { method: 'DELETE' }, user.d.token)
assert(del.status === 200, '删除自己的商品')
const del2 = await j(`/products/${created.d.id}`, { method: 'DELETE' }, admin.d.token)
assert(del2.status === 404, '重复删除返回 404')

const noauth = await j('/products')
assert(noauth.status === 200, '前台商品列表免登录')
