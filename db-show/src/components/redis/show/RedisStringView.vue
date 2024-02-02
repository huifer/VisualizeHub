<script lang="ts" setup>
import { defineProps, onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const { key_type, key_name, db_config_id, db_index } = defineProps([
  "key_type",
  "key_name",
  "db_config_id",
  "db_index",
]);
const vl = ref("");

async function refresh() {
  const resp = await invoke("redis_get_string_data", {
    param: {
      key_type,
      key_name,
      db_config_id,
      db_index: Number(db_index),
    },
  });
  vl.value = resp.data;
  return resp;
}

onMounted(async () => {
  const resp = await refresh();
  console.log(resp);
});
const save = async () => {
  await invoke("redis_set_string_data", {
    param: {
      key_type,
      key_name,
      db_config_id,
      db_index: Number(db_index),
      value: vl.value,
    },
  });
  const resp = await refresh();
};
</script>

<template>
  <div>
    <div>数据类型: String</div>

    <div>键: {{ key_name }}</div>
    <a-input v-model:value="vl" />
    <a-button @click="save">修改</a-button>
  </div>
</template>

<style scoped></style>
