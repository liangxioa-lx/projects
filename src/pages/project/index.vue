<template>
    <div class="page">
        <table class="table">
            <tr>
                <td>项目名称</td>
                <td>项目地址</td>
                <td>默认打开应用</td>
                <td>操作</td>
            </tr>
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