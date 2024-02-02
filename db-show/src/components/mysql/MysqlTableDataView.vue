<!--用来显示表数据的-->
<script setup lang="ts">
const props = defineProps(["tableName", "dbName", "configId"]);
</script>

<template>
  <div>
    <div>表名: {{ props.tableName }}</div>
    <div>类型: {{ props.dbName }}</div>
    <div>类型: {{ props.configId }}</div>
  </div>
</template>

<style scoped></style>
