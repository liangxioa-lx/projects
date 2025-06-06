import {ref} from 'vue'
import {defineStore} from 'pinia'


export const usePathStore = defineStore(
  'path',
  () => {
    const activePath = ref('')
    const searchFileName = ref('')
    const historyPathList = ref([])
      const index = ref(0)

    function openPath(path) {
      activePath.value = path
    if (index.value !== historyPathList.value.length - 1) {
        historyPathList.value.splice(index.value + 1, historyPathList.value.length - index.value - 1)
    }
      historyPathList.value.push(path)
      index.value = historyPathList.value.length - 1
    }

    function back() {
        if (index.value === 0)return
        index.value -= 1
        activePath.value = historyPathList.value[index.value]
    }

    function forward() {
        if (index.value === historyPathList.value.length - 1) return
        index.value += 1
        activePath.value = historyPathList.value[index.value]
    }

    function closePath() {
      activePath.value = ''
    }

    function refresh() {
        activePath.value = ''
        const timer = setTimeout(()=>{
            activePath.value = historyPathList.value[index.value]
            clearTimeout(timer)
        }, 100)
    }

    function openParent() {
        if (activePath.value === '') return
        const paths = activePath.value.split('/')
        paths.pop()
        activePath.value = paths.join('/')
    }

    return {
        activePath,
        searchFileName,
        openPath,
        back,
        forward,
        refresh,
        openParent
    }
  }
)
