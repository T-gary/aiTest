import { computed, ref, watch } from 'vue'

const KEY = 'cart'
const items = ref(load())

function load() {
  try {
    const raw = JSON.parse(localStorage.getItem(KEY) || '[]')
    return Array.isArray(raw) ? raw.filter((i) => i && i.id && i.qty > 0) : []
  } catch {
    return []
  }
}

watch(items, (v) => localStorage.setItem(KEY, JSON.stringify(v)), { deep: true })

export const cart = {
  items,
  count: computed(() => items.value.reduce((n, i) => n + i.qty, 0)),
  total: computed(() => items.value.reduce((n, i) => n + i.price * i.qty, 0)),
  has: (id) => items.value.some((i) => i.id === id),
  add(p) {
    const hit = items.value.find((i) => i.id === p.id)
    if (hit) hit.qty++
    else items.value.push({ id: p.id, name: p.name, price: p.price, image_url: p.image_url, qty: 1 })
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
