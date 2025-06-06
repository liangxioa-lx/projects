<template>
    <div class="page">
        <table class="table">
            <thead>
                <tr>
                    <td>项目名称</td>
                    <td>项目地址</td>
                    <td>默认打开应用</td>
                    <td>操作</td>
                </tr>
            </thead>
            <tbody>
                <tr v-for="(item, index) in projects" :key="index">
                    <td>{{ item.name }}</td>
                    <td>{{ item.path }}</td>
                    <td>{{ item.defaultApp }}</td>
                    <td>
                        <button @click="openProject(item)">打开</button>
                    </td>
                </tr>
            </tbody>
        </table>
    </div>

</template>

<script setup>
const projects = ref([])
</script>

<style scoped lang="scss">
.page {
    padding: 10px;
}

.table {
    width: 100%;
    border-collapse: collapse;

    td {
        border: 1px solid #ddd;
        padding: 2px 5px;
    }
}
</style>