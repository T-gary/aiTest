import { computed, ref, watch } from 'vue'

const KEY = 'cart'
const items = ref(load())

function load() {
  try {
    const raw = JSON.parse(localStorage.getItem(KEY) || '[]')
    return Array.isArray(raw)
      ? raw.filter((i) => i && i.id && i.qty > 0).map((i) => ({ ...i, checked: i.checked !== false }))
      : []
  } catch {
    return []
  }
}

watch(items, (v) => localStorage.setItem(KEY, JSON.stringify(v)), { deep: true })

const checked = computed(() => items.value.filter((i) => i.checked))

export const cart = {
  items,
  // 角标与「购物车 (N)」都用商品种数，而不是件数
  count: computed(() => items.value.length),
  total: computed(() => items.value.reduce((n, i) => n + i.price * i.qty, 0)),
  checkedItems: checked,
  checkedCount: computed(() => checked.value.length),
  checkedTotal: computed(() => checked.value.reduce((n, i) => n + i.price * i.qty, 0)),
  allChecked: computed(() => items.value.length > 0 && items.value.every((i) => i.checked)),
  has: (id) => items.value.some((i) => i.id === id),
  add(p) {
    const hit = items.value.find((i) => i.id === p.id)
    if (hit) hit.qty++
    else items.value.push({ id: p.id, name: p.name, price: p.price, image_url: p.image_url, qty: 1, checked: true })
  },
  toggle(id) {
    const hit = items.value.find((i) => i.id === id)
    if (hit) hit.checked = !hit.checked
  },
  toggleAll(v) {
    items.value.forEach((i) => (i.checked = v))
  },
  setQty(id, qty) {
    const hit = items.value.find((i) => i.id === id)
    if (!hit) return
    hit.qty = qty
    if (hit.qty <= 0) cart.remove(id)
  },
  remove(id) {
    items.value = items.value.filter((i) => i.id !== id)
  },
  clear() {
    items.value = []
  }
}
