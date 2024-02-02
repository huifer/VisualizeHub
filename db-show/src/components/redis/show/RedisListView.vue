<script lang="ts" setup>
import { defineProps, onMounted, reactive, ref, type UnwrapRef } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { message } from "ant-design-vue";
import { cloneDeep } from "lodash-es";

const { key_type, key_name, db_config_id, db_index } = defineProps([
  "key_type",
  "key_name",
  "db_config_id",
  "db_index",
]);
const vl = ref([]);
const tol = ref("");
const columns = [
  {
    title: "id",
    dataIndex: "id",
    width: "25%",
  },
  {
    title: "Value",
    dataIndex: "value",
    width: "15%",
  },
  {
    title: "operation",
    dataIndex: "operation",
  },
];

interface DataItem {
  value: string;
  key: string;
  id: number;
}

const dataSource = ref([]);
const editableData: UnwrapRef<Record<string, DataItem>> = reactive({});

const editorParam = reactive({
  old: "",
  new_val: "",
});
const edit = (key: string) => {
  editableData[key] = cloneDeep(
    dataSource.value.filter((item) => key === item.key)[0],
  );
};
const save = async (key: string) => {
  var old = dataSource.value.filter((item) => key === item.key)[0];
  var newVal = editableData[key];
  console.log("old", old);
  console.log("new", newVal);

  editorParam.old = old.value;
  editorParam.new_val = newVal.value;
  console.log("11");
  await change_redis_set();
  Object.assign(
    dataSource.value.filter((item) => key === item.key)[0],
    editableData[key],
  );
  delete editableData[key];
  await refresh();
};

const change_redis_set = async () => {
  return await invoke("redis_change_list", {
    param: {
      key_type,
      key_name,
      db_config_id,
      db_index: Number(db_index),
      old: editorParam.old,
      new_val: editorParam.new_val,
    },
  });
};
const cancel = (key: string) => {
  delete editableData[key];
};
const delete_rember = async (key: string) => {
  var old = dataSource.value.filter((item) => key === item.key)[0];
  const modifiedDataSource = dataSource.value.filter(
    (item) => item.key !== key,
  );

  console.log("==========");
  let resp = await invoke("redis_remove_list_member", {
    param: {
      key_type,
      key_name,
      db_config_id,
      db_index: Number(db_index),
      old: old.value,
    },
  });

  dataSource.value = modifiedDataSource;
  console.log("删除");

  await refresh();
};
const confirm_delete = async (e: MouseEvent, key: string) => {
  await delete_rember(key);
};

const cancel_delete = (e: MouseEvent, key) => {
  console.log(e);

  message.error("Click on No");
};

async function refresh() {
  const resp = await invoke("redis_get_list_data", {
    param: {
      key_type,
      key_name,
      db_config_id,
      db_index: Number(db_index),
    },
  });
  vl.value = resp.data.values;
  tol.value = resp.data.total_length;
  const d = resp.data.values.map((item, index) => ({
    key: index,
    value: item.toString(),
    id: index + 1,
  }));
  console.log(d);
  dataSource.value = d;

  console.log("======");
}

onMounted(async () => {
  await refresh();
});
</script>

<template>
  <div>
    <div>数据类型: List</div>
    <div>键：{{ key_name }}</div>
    <a-table :columns="columns" :data-source="dataSource" bordered>
      <template #bodyCell="{ column, text, record }">
        <template v-if="['value'].includes(column.dataIndex)">
          <div>
            <a-input
              v-if="editableData[record.key]"
              v-model:value="editableData[record.key][column.dataIndex]"
              style="margin: -5px 0"
            />
            <template v-else>
              {{ text }}
            </template>
          </div>
        </template>
        <template v-else-if="column.dataIndex === 'operation'">
          <div class="editable-row-operations">
            <span v-if="editableData[record.key]">
              <a-typography-link @click="save(record.key)"
                >Save</a-typography-link
              >
              <a-popconfirm
                title="Sure to cancel?"
                @confirm="cancel(record.key)"
              >
                <a>Cancel</a>
              </a-popconfirm>
            </span>
            <span v-else>
              <a @click="edit(record.key)">Edit</a>

              <a-popconfirm
                cancel-text="No"
                ok-text="Yes"
                title="确认删除？"
                @cancel="cancel_delete"
                @confirm="confirm_delete($event, record.key)"
              >
                <a href="#">Delete</a>
              </a-popconfirm>
            </span>
          </div>
        </template>
      </template>
    </a-table>
  </div>
</template>

<style scoped>
.editable-row-operations a {
  margin-right: 8px;
}
</style>
