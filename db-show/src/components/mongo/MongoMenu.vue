<script lang="ts" setup>
import { DatabaseOutlined } from "@ant-design/icons-vue";
import { getCurrentInstance, onMounted, reactive, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { useEventbus } from "../../ent/events.ts";
import { useMongoStore } from "../../storage/mongo_pina.ts";

const mongoStore = useMongoStore();
const eventbus = useEventbus();

const event_push = getCurrentInstance();
const mongo_config_data = ref([]);
const mongo_conn_db_names = reactive({});
const show_db_names = async (mongo_config_id: string) => {
  mongoStore.setDbConfigId(mongo_config_id);
  let resp = await invoke("mongo_db_names", {
    param: {
      id: mongo_config_id,
    },
  });
  console.log("============");
  if (resp.status_code == 20000) {
    mongo_conn_db_names[mongo_config_id] = resp.data;
  }
};
const show_table_names = (a: string, b: string) => {};

const init_mongo_connection_config = async () => {
  let resp = await invoke("query_all_mongo");
  console.log(resp);
  if (resp.status_code == 20000) {
    mongo_config_data.value = resp.data;
  }
};

onMounted(async () => {
  await init_mongo_connection_config();
  eventbus.customOn("mongo:created", async () => {
    await init_mongo_connection_config();
  });
});
const db_click = (x: any) => {
  event_push?.emit("cur_type", "MONGO");
};
</script>

<template>
  <a-sub-menu key="4">
    <template #title>
      <span>
        <database-outlined />
        <span>Mongo</span>
      </span>
    </template>
    <a-sub-menu
      v-for="x in mongo_config_data"
      :key="x.id"
      :title="x.name"
      @click="db_click(x)"
      @titleClick="show_db_names(x.id)"
    >
      <a-menu-item
        v-for="xx in mongo_conn_db_names[x.id]"
        :key="`${x.id}@${xx}`"
        @click="show_table_names(x.id, xx)"
        >{{ xx }}
      </a-menu-item>
    </a-sub-menu>
  </a-sub-menu>
</template>

<style scoped></style>
