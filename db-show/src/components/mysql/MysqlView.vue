<script lang="ts" setup>
import {
  TableOutlined,
  UnorderedListOutlined,
  WhatsAppOutlined,
} from "@ant-design/icons-vue";
import { ref, watch } from "vue";
import { useMysqlStore } from "../../storage/mysql_pina.ts";
import { invoke } from "@tauri-apps/api/tauri";
import MysqlTableDataView from "./MysqlTableDataView.vue";
import { useEventbus } from "../../ent/events.ts";

const mysqlStore = useMysqlStore();
const core_active = ref("table");
const core_active_list = ref([]);
const observedTableNames = ref(mysqlStore.mysqlData);
const observedDbConfigId = ref(mysqlStore.cur_db_config_id);
const observedDbName = ref(mysqlStore.cur_db_name);
const eventbus = useEventbus();

watch(
  () => mysqlStore.mysqlData,
  (newData) => {
    observedTableNames.value = newData;
  },
  { immediate: true },
);
eventbus.customOn("mysql_db_select", async (id) => {
  console.log("mysql_db_select");
  await get_mysql_status(id);
});
watch(
  () => mysqlStore.cur_db_name,
  (newDbId, oldDbId) => {
    if (newDbId !== oldDbId) {
      observedDbName.value = newDbId;

      core_active_list.value.splice(0, core_active_list.value.length);
      core_active.value = "table";
    }
  },
  { immediate: true },
);

watch(
  () => mysqlStore.mysqlData,
  (newData) => {
    observedTableNames.value = newData;
  },
  { immediate: true },
);

watch(
  () => mysqlStore.cur_db_config_id,
  async (newDbId, oldDbId) => {
    observedDbConfigId.value = newDbId;
    console.log("watch mysqlStore.cur_db_config_id");
    await get_mysql_status(observedDbConfigId.value);
  },
);

const mysql_status_data = ref({});
const get_mysql_status = async (id: any) => {
  let resp = await invoke("get_db_status", { id: id });
  console.log("get_db_status", resp);
  if (resp.status_code == 20000) {
    mysql_status_data.value = resp.data;
  }
};
const activeKey = ref("1");
const cur_create_sql = ref("");
const table_click = async (table: string) => {
  console.log("observedDbName", observedDbName.value);
  let resp = await invoke("show_table_create_sql", {
    param: {
      id: observedDbConfigId.value,
      table_name: table,
      db_name: observedDbName.value,
    },
  });
  console.log(resp);
  if (resp.status_code == 20000) {
    activeKey.value = "2";
    cur_create_sql.value = resp.data;
  }
};

const show_table_data = async (table_name, db_name, config_id) => {
  console.log("show_table_data", table_name, db_name, config_id);

  // 检查是否已存在相同元素
  const existingIndex = core_active_list.value.findIndex(
    (item) => item.name === table_name && item.type === "数据表",
  );
  if (existingIndex === -1) {
    // 不存在，则添加新元素
    const newItem = {
      name: table_name,
      type: "数据表",
    };
    core_active_list.value.splice(0, 0, newItem);
    console.log("Item added at index:", 0);
    core_active.value = 0;
  } else {
    core_active.value = existingIndex;
    console.log("Item already exists at index:", existingIndex);
  }
};
const table_design = async (table_name, db_name, config_id) => {
  console.log("table_design", table_name, db_name, config_id);

  // 检查是否已存在相同元素
  const existingIndex = core_active_list.value.findIndex(
    (item) => item.name === table_name && item.type === "设计表",
  );
  if (existingIndex === -1) {
    // 不存在，则添加新元素
    const newItem = {
      name: table_name,
      type: "设计表",
    };
    core_active_list.value.splice(0, 0, newItem);
    console.log("Item added at index:", 0);
    core_active.value = 0;
  } else {
    core_active.value = existingIndex;
    console.log("Item already exists at index:", existingIndex);
  }
};
const onEdit = (targetKey: string | MouseEvent, action: string) => {
  remove_core_active_list(targetKey);
  core_active.value = "table";
};
const remove_core_active_list = (index: any) => {
  core_active_list.value.splice(index, 1);
};
</script>

<template>
  <a-layout>
    <a-layout-header style="background: #52b271; padding: 0">
      <div>数据查询区域</div>
    </a-layout-header>
    <a-layout>
      <a-layout-content style="margin: 10px 16px 0; height: 100%">
        <a-tabs
          v-if="observedDbName != null && observedDbName != ''"
          v-model:activeKey="core_active"
          type="editable-card"
          @edit="onEdit"
        >
          <a-tab-pane key="table" :closable="false">
            <template #tab>
              <span>
                <table-outlined />
                数据表
              </span>
            </template>
            <div
              :style="{
                padding: '24px',
                background: '#fff',
                minHeight: '360px',
                height: '100%',
              }"
            >
              <a-row :gutter="[4, 4]">
                <a-dropdown
                  v-for="(table_name, index) in observedTableNames"
                  :trigger="['contextmenu']"
                >
                  <div>
                    <a-tag
                      :key="index"
                      color="#55acee"
                      @click="table_click(table_name)"
                      >{{ table_name }}
                    </a-tag>
                  </div>
                  <template #overlay>
                    <a-menu>
                      <a-menu-item
                        key="tab_data"
                        @click="
                          show_table_data(
                            table_name,
                            observedDbName,
                            observedDbConfigId,
                          )
                        "
                        >表数据
                      </a-menu-item>
                      <a-menu-item
                        key="tab_design"
                        @click="
                          table_design(
                            table_name,
                            observedDbName,
                            observedDbConfigId,
                          )
                        "
                      >
                        表设计
                      </a-menu-item>
                    </a-menu>
                  </template>
                </a-dropdown>
              </a-row>
            </div>
          </a-tab-pane>

          <a-tab-pane v-for="(x, index) in core_active_list" :key="index">
            <template #tab>
              <span>
                <UnorderedListOutlined v-if="x.type == '数据表'" />
                <WhatsAppOutlined
                  v-else-if="x.type == '设计表'"
                ></WhatsAppOutlined>
                {{ x.name }}
              </span>
            </template>

            <div
              :style="{
                padding: '24px',
                background: '#fff',
                minHeight: '360px',
                height: '100%',
              }"
            >
              <mysql-table-data-view
                :configId="observedDbConfigId"
                :dbName="observedDbName"
                :tableName="x.name"
              ></mysql-table-data-view>
            </div>
          </a-tab-pane>
        </a-tabs>
      </a-layout-content>
      <a-layout-sider style="margin: 10px 2px 0; height: 100%" theme="light">
        <a-tabs v-model:activeKey="activeKey">
          <a-tab-pane key="1" tab="数据库状态">
            <div>数据库版本: {{ mysql_status_data.version }}</div>

            <a-row>
              <!-- 左侧列 -->
              <a-list
                :dataSource="mysql_status_data.status"
                bordered
                style="height: 100vh; overflow-y: auto; width: 100%"
              >
                <template #renderItem="{ item }">
                  <a-list-item>
                    <a-list-item-meta
                      :description="item.value"
                      :title="item.name"
                    />
                  </a-list-item>
                </template>
              </a-list>
            </a-row>
          </a-tab-pane>
          <a-tab-pane key="2" force-render tab="表信息">
            {{ cur_create_sql }}
          </a-tab-pane>
        </a-tabs>
      </a-layout-sider>
    </a-layout>
    <a-layout-footer style="text-align: center; height: 30px">
      Create With HuiFer @2024
    </a-layout-footer>
  </a-layout>
</template>

<style scoped></style>
