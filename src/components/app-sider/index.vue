<template>
  <div class="app-sider">
    <div class="app-sider-menu" :class="{ active: activePath === item.value }" @click="selectMenu(item)"
      v-for="(item, index) in tabs" :key="index">
      <div class="app-sider-menu-icon"></div>
      <div class="app-sider-menu-text">{{ item.label }}</div>
    </div>
  </div>
</template>

<script setup>
import {ref } from "vue";
import { useRouter } from 'vue-router';

const tabs = ref([
  { label: '项目管理', value: 'project', path: '/' },
  { label: '应用管理', value: 'ide', path: '/ide' },
])

const activePath = ref('project')

const router = useRouter()
function selectMenu(menu) {
  activePath.value = menu.value
  router.push(menu.path)
}
</script>

<style scoped lang="scss">
.app-sider {
  height: 100%;
  width: 80px;
  padding: 5px 0 5px 5px;
  box-sizing: border-box;
  font-size: 12px;
  flex-shrink: 0;
}

.app-sider-menu {
  padding: 0 5px;
  height: 28px;
  display: flex;
  align-items: center;
  line-height: 28px;
  border-radius: 4px;
  cursor: pointer;

  &.active {
    background-color: #ddd;
  }
}

.app-sider-menu-group {

  .group-title {
    padding: 0 5px;
    cursor: pointer;
    display: flex;
    align-items: center;

    .text {
      flex: 1;
    }

    .svg-icon {
      width: 12px;
      height: 12px;
    }
  }
}
</style>
