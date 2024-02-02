<script lang="ts" setup>
import { DatabaseOutlined } from "@ant-design/icons-vue";
import { getCurrentInstance, onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { useRedisStore } from "../../storage/redis_pina.ts";
import { useEventbus } from "../../ent/events.ts";

const redisStore = useRedisStore();
const eventbus = useEventbus();

const event_push = getCurrentInstance();
const redis_config_data = ref([]);
const show_db_names = (redis_config_id: string) => {
  redisStore.setDbConfigId(redis_config_id);
};
const show_table_names = (a: string, b: string) => {};

const init_redis_connection_config = async () => {
  let resp = await invoke("query_all_redis");
  console.log(resp);
  if (resp.status_code == 20000) {
    redis_config_data.value = resp.data;
  }
};

onMounted(async () => {
  await init_redis_connection_config();
  eventbus.customOn("Redis:created", async () => {
    await init_redis_connection_config();
  });
});
const db_click = (x: any) => {
  event_push?.emit("cur_type", "REDIS");
  eventbus.customEmit("redis_db_select", x.id);
};
</script>

<template>
  <a-sub-menu key="3">
    <template #title>
      <span>
        <database-outlined />
        <span>Redis</span>
      </span>
    </template>
    <a-sub-menu
      v-for="x in redis_config_data"
      :key="x.id"
      :title="x.name"
      @click="db_click(x)"
      @titleClick="show_db_names(x.id)"
    >
      <!--      <a-menu-item-->
      <!--        v-for="xx in redis_conn_db_names[x.id]"-->
      <!--        :key="xx"-->
      <!--        @click="show_table_names(x.id, xx)"-->
      <!--        >{{ xx }}-->
      <!--      </a-menu-item>-->
    </a-sub-menu>
  </a-sub-menu>
</template>

<style scoped></style>
