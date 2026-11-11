import { ref, watch } from 'vue'

const KEY = 'mobileView'
export const mobileView = ref(localStorage.getItem(KEY) === '1')

watch(mobileView, (v) => localStorage.setItem(KEY, v ? '1' : '0'))
