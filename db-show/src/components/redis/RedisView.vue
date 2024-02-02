<script lang="ts" setup>
import { onMounted, ref, watch } from "vue";
import { useRedisStore } from "../../storage/redis_pina.ts";
import { invoke } from "@tauri-apps/api/tauri";
import { TableOutlined } from "@ant-design/icons-vue";
import { useEventbus } from "../../ent/events.ts";
import RedisKeyTable from "./RedisKeyTable.vue";

const eventbus = useEventbus();

const redisStore = useRedisStore();
const observedDbConfigId = ref(redisStore.cur_db_config_id);
const server_info = ref({});
const clients_info = ref({});
const memory_info = ref({});
const stats_info = ref({});
const persistence_info = ref({});
const replication_info = ref({});
const cpu_info = ref({});
const keyspace = ref([]);
const columns = [
  {
    title: "index",
    dataIndex: "index",
    key: "index",
  },
  {
    title: "keys",
    dataIndex: "keys",
    key: "keys",
  },
  {
    title: "expires",
    dataIndex: "expires",
    key: "expires",
  },
  {
    title: "avg_ttl",
    dataIndex: "avg_ttl",
    key: "avg_ttl",
  },
];
const get_redis_status = async (id: any) => {
  let resp = await invoke("redis_info", {
    param: { id: id },
  });
  if (resp.status_code == 20000) {
    server_info.value = resp.data.server_info;
    clients_info.value = resp.data.clients_info;
    memory_info.value = resp.data.memory_info;
    stats_info.value = resp.data.stats_info;
    persistence_info.value = resp.data.persistence_info;
    replication_info.value = resp.data.replication_info;
    cpu_info.value = resp.data.cpu_info;
    keyspace.value = resp.data.keyspace;
  }
  console.log("redis 服务信息获取完毕");
};
const onEdit = (targetKey: string | MouseEvent, action: string) => {
  core_active.value = "server";
};
const core_active = ref("server");

watch(
  () => redisStore.cur_db_config_id,
  async (newDbId, oldDbId) => {
    if (newDbId !== oldDbId) {
      observedDbConfigId.value = newDbId;
      console.log("=========");
      await get_redis_status(observedDbConfigId.value);
    }
  },
  { immediate: true },
);

eventbus.customOn("redis_db_select_change", () => {
  core_active.value = "keys";
});
eventbus.customOn("redis_config_select_change", () => {
  core_active.value = "server";
});
onMounted(async () => {});
</script>

<template>
  <a-layout>
    <a-layout-content class="content">
      <a-tabs
        v-if="observedDbConfigId"
        v-model:activeKey="core_active"
        type="editable-card"
        @edit="onEdit"
      >
        <a-tab-pane key="server" :closable="false">
          <template #tab>
            <span>
              <table-outlined />
              服务器信息
            </span>
          </template>
          <div class="tab-content">
            <a-descriptions
              bordered
              title="Redis服务信息"
              :column="{ xxl: 4, xl: 3, lg: 3, md: 3, sm: 2, xs: 1 }"
            >
              <a-descriptions-item label="版本"
                >{{ server_info.redis_version }}
              </a-descriptions-item>
              <a-descriptions-item label="系统版本"
                >{{ server_info.os }}
              </a-descriptions-item>
              <a-descriptions-item label="redis_mode"
                >{{ server_info.redis_mode }}
              </a-descriptions-item>
              <a-descriptions-item label="process_id"
                >{{ server_info.process_id }}
              </a-descriptions-item>
              <a-descriptions-item label="used_memory"
                >{{ memory_info.used_memory }}
              </a-descriptions-item>
              <a-descriptions-item label="used_memory_peak"
                >{{ memory_info.used_memory_peak }}
              </a-descriptions-item>
              <a-descriptions-item label="used_memory_lua"
                >{{ memory_info.used_memory_lua }}
              </a-descriptions-item>
              <a-descriptions-item label="total_commands_processed"
                >{{ stats_info.total_commands_processed }}
              </a-descriptions-item>
            </a-descriptions>

            <br />

            <a-card title="Key Statistics">
              <a-table :columns="columns" :data-source="keyspace" bordered>
                <template #bodyCell="{ column, text }">
                  <template v-if="column.dataIndex === 'index'">
                    <a>db{{ text }}</a>
                  </template>
                  <template v-else>
                    <a>{{ text }}</a>
                  </template>
                </template>
              </a-table>
            </a-card>
          </div>
        </a-tab-pane>
        <a-tab-pane key="keys" :closable="false">
          <template #tab>
            <span>
              <table-outlined />
              键信息
            </span>
          </template>
          <div class="tab-content">
            <redis-key-table></redis-key-table>
          </div>
        </a-tab-pane>
      </a-tabs>
    </a-layout-content>
  </a-layout>
</template>

<style scoped>
.content {
  margin: 10px 16px 0;
  height: 100%;
}

.tab-content {
  padding: 24px;
  background: #fff;
  min-height: 360px;
  height: 100%;
}
</style>
