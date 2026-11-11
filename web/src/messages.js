import { reactive, watch } from 'vue'

const KEY = 'msgUnread'
const saved = Number(localStorage.getItem(KEY) ?? '3')

// 演示用的消息状态：未读数 + 已读操作，不落库
export const messageState = reactive({
  unread: Number.isFinite(saved) ? saved : 3,
  clear() {
    this.unread = 0
  },
  readOne() {
    if (this.unread > 0) this.unread -= 1
  }
})

watch(
  () => messageState.unread,
  (v) => localStorage.setItem(KEY, String(v))
)
