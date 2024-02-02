<script lang="ts" setup>
import { DatabaseOutlined } from "@ant-design/icons-vue";
import { getCurrentInstance, onMounted, reactive, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { useMysqlStore } from "../../storage/mysql_pina.ts";
import { useEventbus } from "../../ent/events.ts";

const eventbus = useEventbus();

const event_push = getCurrentInstance();
const mysql_config_data = ref([]);
const mysql_conn_db_names = reactive({
  a: ["1", "2"],
});
const mysqlStore = useMysqlStore();
const show_db_names = async (id: any) => {
  console.log(id);

  let resp = await invoke("get_db_names", {
    id: id,
  });
  console.log("resp ", resp);
  mysql_conn_db_names[id] = [...resp.data];

  mysqlStore.setDbConfigId(id);
  eventbus.customEmit("mysql_db_select", id);
};
const init_mysql_connection_config = async () => {
  let resp = await invoke("query_all_mysql");
  console.log(resp);
  if (resp.status_code == 20000) {
    mysql_config_data.value = resp.data;
  }
};

onMounted(async () => {
  await init_mysql_connection_config();
  eventbus.customOn("MySQL:created", async () => {
    await init_mysql_connection_config();
  });
});

const show_table_names = async (id: any, db: string) => {
  console.log("数据库id ", id, "数据库名称", db);
  mysqlStore.setDbConfigId(id);
  let resp = await invoke("get_tables_names", {
    param: {
      id: id,
      db_name: db,
    },
  });
  if (resp.status_code == 20000) {
    mysqlStore.setMysqlDataArray(resp.data);
    mysqlStore.setDbName(db);
  }
};
</script>

<template>
  <a-sub-menu key="2">
    <template #title>
      <span>
        <database-outlined />
        <span>MYSQL</span>
      </span>
    </template>
    <a-sub-menu
      v-for="x in mysql_config_data"
      :key="x.id"
      :title="x.name"
      @click="
        () => {
          event_push?.emit('cur_type', 'MYSQL');
        }
      "
      @titleClick="show_db_names(x.id)"
    >
      <a-menu-item
        v-for="xx in mysql_conn_db_names[x.id]"
        :key="xx"
        @click="show_table_names(x.id, xx)"
        >{{ xx }}
      </a-menu-item>
    </a-sub-menu>
  </a-sub-menu>
</template>

<style scoped></style>
