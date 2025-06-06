<template>
  <div class="app-content">
    <div class="content-box">
      <slot></slot>
    </div>
  </div>
</template>

<script setup>
</script>

<style scoped lang="scss">
.app-content {
  flex: 1;
  height: 100%;
  padding: 5px;
  box-sizing: border-box;
  overflow: hidden;

  .content-box {
    width: calc(100vw - 90px);
    height: 100%;
    background: #ffffff;
    border-radius: 5px;
    box-shadow: 0 0 4px rgba(0, 0, 0, 0.12);
  }
}
</style>
