/**
 * 点击元素外部指令
 */
export default {
  install(app) {
    app.directive('click-outside', {
      mounted(el, binding) {
        function eventHandler(e) {
          if (el.contains(e.target) || el === e.target) {
            return false
          }
          if (binding.value && typeof binding.value === 'function') {
            binding.value(e)
          }
        }

        el.Tag = eventHandler
        document.addEventListener('click', eventHandler)
      },
      beforeUnmount(el) {
        document.removeEventListener('click', el.Tag)
        delete el.Tag
      }
    })
  }
}
