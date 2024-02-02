<script lang="ts" setup>
import { DatabaseOutlined } from "@ant-design/icons-vue";
import { getCurrentInstance, onMounted, reactive, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { useRedisStore } from "../../storage/redis_pina.ts";
import { useEventbus } from "../../ent/events.ts";

const redisStore = useRedisStore();
const eventbus = useEventbus();

const event_push = getCurrentInstance();
const redis_config_data = ref([]);
const redis_conn_db_count = reactive({});

const show_db_names = async (redis_config_id: string) => {
  redisStore.setDbConfigId(redis_config_id);
  let resp = await invoke("redis_db_count", { param: { id: redis_config_id } });
  if (resp.status_code == 20000) {
    redis_conn_db_count[redis_config_id] = resp.data;
  }
  eventbus.customEmit("redis_config_select_change");
};
const show_db_info = (redis_config_id: string, db_index: string) => {
  redisStore.setDbIndexId(db_index);
  redisStore.setDbConfigId(redis_config_id);
  console.log("redis_config_id = ", redis_config_id, "db_index = ", db_index);
  eventbus.customEmit("redis_db_select_change");
};

const init_redis_connection_config = async () => {
  let resp = await invoke("query_all_redis");
  console.log(resp);
  if (resp.status_code == 20000) {
    redis_config_data.value = resp.data;
  }
  debugger;
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
      <a-menu-item
        v-for="xx in redis_conn_db_count[x.id]"
        :key="`${x.id}@${xx}`"
        @click="show_db_info(x.id, xx - 1)"
        >db{{ xx - 1 }}
      </a-menu-item>
    </a-sub-menu>
  </a-sub-menu>
</template>

<style scoped></style>
