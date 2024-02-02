<script lang="ts" setup>
import { onMounted, ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { TableOutlined } from "@ant-design/icons-vue";
import { useEventbus } from "../../ent/events.ts";
import { useMongoStore } from "../../storage/mongo_pina.ts";

const eventbus = useEventbus();

const mongoStore = useMongoStore();
const observedDbConfigId = ref(mongoStore.cur_db_config_id);
const mongo_server_state = ref({});
const serverInfo = ref({});
const memInfo = ref({});
const connectionsInfo = ref({});

const get_mongo_status = async (id: any) => {
  let resp = await invoke("mongo_info", {
    param: { id: id },
  });
  if (resp.status_code == 20000) {
  }
  console.log("MONGO 服务信息获取完毕");
  console.log(resp.data);
  mongo_server_state.value = resp.data;
  serverInfo.value = resp.data.mongo_server_info;
  memInfo.value = resp.data.mongo_mem_info;
  connectionsInfo.value = resp.data.mongo_connections_info;
  console.log(JSON.stringify(resp.data));
};
const onEdit = (targetKey: string | MouseEvent, action: string) => {
  core_active.value = "server";
};
const core_active = ref("server");

watch(
  () => mongoStore.cur_db_config_id,
  async (newDbId, oldDbId) => {
    if (newDbId !== oldDbId) {
      observedDbConfigId.value = newDbId;
      console.log("=========");
      await get_mongo_status(observedDbConfigId.value);
    }
  },
  { immediate: true },
);
onMounted(async () => {});
</script>

<template>
  <a-layout>
    <a-layout-content style="margin: 10px 16px 0; height: 100%">
      <a-tabs
        v-if="observedDbConfigId != null && observedDbConfigId != ''"
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

          <a-layout>
            <a-layout-content style="padding: 3px">
              <a-card title="Mongo Server Info">
                <a-descriptions :bordered="true" :column="1">
                  <a-descriptions-item label="Host">{{
                    serverInfo.host
                  }}</a-descriptions-item>
                  <a-descriptions-item label="Version">{{
                    serverInfo.version
                  }}</a-descriptions-item>
                  <a-descriptions-item label="Process">{{
                    serverInfo.process
                  }}</a-descriptions-item>
                  <a-descriptions-item label="PID">{{
                    serverInfo.pid
                  }}</a-descriptions-item>
                  <!-- Add other items as needed -->
                </a-descriptions>
              </a-card>

              <a-card title="Mongo Memory Info">
                <a-descriptions :bordered="true" :column="1">
                  <a-descriptions-item label="Bits">{{
                    memInfo.bits
                  }}</a-descriptions-item>
                  <a-descriptions-item label="Resident">{{
                    memInfo.resident
                  }}</a-descriptions-item>
                  <a-descriptions-item label="Virtual">{{
                    memInfo.virtual
                  }}</a-descriptions-item>
                  <!-- Add other items as needed -->
                </a-descriptions>
              </a-card>

              <a-card title="Mongo Connections Info">
                <a-descriptions :bordered="true" :column="1">
                  <a-descriptions-item label="Current">{{
                    connectionsInfo.current
                  }}</a-descriptions-item>
                  <a-descriptions-item label="Available">{{
                    connectionsInfo.available
                  }}</a-descriptions-item>
                  <a-descriptions-item label="Total Created">{{
                    connectionsInfo.total_created
                  }}</a-descriptions-item>
                  <a-descriptions-item label="Rejected">{{
                    connectionsInfo.rejected
                  }}</a-descriptions-item>
                  <!-- Add other items as needed -->
                </a-descriptions>
              </a-card>
            </a-layout-content>
          </a-layout>
        </a-tab-pane>
      </a-tabs>
    </a-layout-content>
  </a-layout>
</template>

<style scoped></style>
