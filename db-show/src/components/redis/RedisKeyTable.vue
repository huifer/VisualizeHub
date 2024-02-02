<script lang="ts" setup>
import { useRedisStore } from "../../storage/redis_pina.ts";
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { SmileOutlined } from "@ant-design/icons-vue";
import { useEventbus } from "../../ent/events.ts";
import { message } from "ant-design-vue";
import RedisStringView from "./show/RedisStringView.vue";
import RedisListView from "./show/RedisListView.vue";
import RedisZSetView from "./show/RedisZSetView.vue";
import RedisSetView from "./show/RedisSetView.vue";
import RedisHashView from "./show/RedisHashView.vue";

const [messageApi, contextHolder] = message.useMessage();

const redisStore = useRedisStore();
const observedDbConfigId = ref(redisStore.cur_db_config_id);
const dv = ref([]);
const new_cursor = ref("");
const eventbus = useEventbus();
const cur_row = ref({
  key_type: "",
  key_name: "",
  db_config_id: "",
  db_index: -1,
});

async function refresh() {
  new_cursor.value = "";
  dv.value = [];
  cur_row.value = {
    key_type: "",
    key_name: "",
    db_config_id: "",
    db_index: -1,
  };
  await key_info();
}

eventbus.customOn("redis_db_select_change", async () => {
  await refresh();
});
const key_info = async () => {
  let resp = await invoke("redis_keys_page", {
    param: {
      id: observedDbConfigId.value,
      db_index: redisStore.cur_db_index,
      page: 0,
      page_size: 1,
    },
  });

  // 获取原始数组
  const array = dv.value;

  dv.value = resp.data.keys;
  new_cursor.value = resp.data.new_cursor;
};

const columns = [
  {
    name: "键名",
    dataIndex: "key_name",
    key: "key_name",
  },
  {
    title: "键类型",
    dataIndex: "key_type",
    key: "key_type",
  },
  {
    title: "过期时间",
    dataIndex: "ttl",
    key: "ttl",
  },
  {
    title: "操作",
    key: "action",
  },
];
onMounted(async () => {
  await key_info();
});
const loadMore = async () => {
  if (new_cursor.value == 0) {
    messageApi.info("没有更多数据");
    console.log("==============");
    return;
  }
  let resp = await invoke("redis_keys_page", {
    param: {
      id: observedDbConfigId.value,
      db_index: redisStore.cur_db_index,
      page: new_cursor.value,
      page_size: 1,
    },
  });
  new_cursor.value = resp.data.new_cursor;

  // 获取原始数组
  const array = dv.value;

  // 修改数组（这里使用了解构赋值，确保原数组保持响应式）
  dv.value = [...array, ...resp.data.keys];
};

const open = ref<boolean>(false);

const showModal = (record: any) => {
  open.value = true;
  cur_row.value = {
    key_type: record.key_type,
    key_name: record.key_name,
    db_config_id: observedDbConfigId.value,
    db_index: Number(redisStore.cur_db_index),
  };
};
const open_exp = ref(false);
const exp_time = ref("");

const showExpire = (record: any) => {
  cur_row.value = {
    key_type: record.key_type,
    key_name: record.key_name,
    db_config_id: observedDbConfigId.value,
    db_index: Number(redisStore.cur_db_index),
  };
  open_exp.value = true;
};
const open_del = ref(false);
const handler_del = async () => {
  let resp = await invoke("redis_delete_redis_key", {
    param: {
      key_type: cur_row.value.key_type,
      key_name: cur_row.value.key_name,
      db_config_id: cur_row.value.db_config_id,
      db_index: cur_row.value.db_index,
    },
  });
  await refresh();
  open_del.value = false;
};
const showDel = async (record: any) => {
  cur_row.value = {
    key_type: record.key_type,
    key_name: record.key_name,
    db_config_id: observedDbConfigId.value,
    db_index: Number(redisStore.cur_db_index),
  };

  open_del.value = true;
};

const handleOk = (e: MouseEvent) => {
  open.value = false;
};
const handler_exp = async (e: MouseEvent) => {
  let resp = await invoke("redis_set_redis_key_expire", {
    param: {
      key_type: cur_row.value.key_type,
      key_name: cur_row.value.key_name,
      db_config_id: cur_row.value.db_config_id,
      db_index: cur_row.value.db_index,
      expiration_seconds: Number(exp_time.value),
    },
  });
  open_exp.value = false;
  await refresh();
};
</script>

<template>
  <div>
    <context-holder />
    <a-button type="primary" @click="loadMore">加载更多</a-button>
    <a-button type="primary" @click="key_info">刷新</a-button>

    <a-table :columns="columns" :data-source="dv" :pagination="false">
      <template #headerCell="{ column }">
        <template v-if="column.key === 'key_name'">
          <span>
            <smile-outlined />
            {{ column.name }}
          </span>
        </template>
      </template>

      <template #bodyCell="{ column, record }">
        <template v-if="column.key === 'action'">
          <span>
            <a-space wrap>
              <a-button type="primary" @click="showModal(record)"
                >查看</a-button
              >
              <a-button @click="showExpire(record)">设置过期时间</a-button>
              <a-button danger type="primary" @click="showDel(record)"
                >删除</a-button
              >
            </a-space>
          </span>
        </template>
      </template>
    </a-table>

    <a-modal
      v-model:open="open_del"
      :destroyOnClose="true"
      title="确认删除"
      @ok="handler_del"
      >是否删除此项
    </a-modal>

    <a-modal
      v-model:open="open_exp"
      :destroyOnClose="true"
      title="设置过期时间"
      @ok="handler_exp"
    >
      <a-input-number v-model:value="exp_time" step="1"></a-input-number>
      秒
    </a-modal>

    <a-modal
      v-model:open="open"
      :destroyOnClose="true"
      width="70%"
      title="值信息"
      @ok="handleOk"
    >
      <redis-string-view
        v-if="cur_row.key_type == 'String'"
        :db_config_id="cur_row.db_config_id"
        :db_index="cur_row.db_index"
        :key_name="cur_row.key_name"
        :key_type="cur_row.key_type"
      />

      <redis-list-view
        v-if="cur_row.key_type == 'List'"
        :db_config_id="cur_row.db_config_id"
        :db_index="cur_row.db_index"
        :key_name="cur_row.key_name"
        :key_type="cur_row.key_type"
      />

      <redis-z-set-view
        v-if="cur_row.key_type == 'ZSet'"
        :db_config_id="cur_row.db_config_id"
        :db_index="cur_row.db_index"
        :key_name="cur_row.key_name"
        :key_type="cur_row.key_type"
      ></redis-z-set-view>

      <redis-set-view
        v-if="cur_row.key_type == 'Set'"
        :db_config_id="cur_row.db_config_id"
        :db_index="cur_row.db_index"
        :key_name="cur_row.key_name"
        :key_type="cur_row.key_type"
      />

      <redis-hash-view
        v-if="cur_row.key_type == 'Hash'"
        :db_config_id="cur_row.db_config_id"
        :db_index="cur_row.db_index"
        :key_name="cur_row.key_name"
        :key_type="cur_row.key_type"
      />
    </a-modal>
  </div>
</template>

<style scoped></style>
