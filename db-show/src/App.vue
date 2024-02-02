<template>
  <a-layout style="min-height: 100vh">
    <a-layout>
      <a-layout-sider v-model:collapsed="collapsed" collapsible theme="light">
        <div>
          <a-menu
            v-model:selectedKeys="selectedKeys"
            mode="inline"
            theme="light"
          >
            <mysql-menu @cur_type="handler_cur_type" />
            <redis-menu @cur_type="handler_cur_type" />
            <mongo-menu @cur_type="handler_cur_type" />
          </a-menu>
          <div class="button-container">
            <a-button
              class="circle-button"
              shape="circle"
              type="primary"
              @click="handleClick"
            >
              <PlusOutlined />
            </a-button>
          </div>
        </div>
      </a-layout-sider>

      <mysql-view v-show="cur_type == 'MYSQL'" />
      <redis-view v-show="cur_type == 'REDIS'" />
      <mongo-view v-show="cur_type == 'MONGO'" />
    </a-layout>
  </a-layout>

  <add-connection-diag
    :visible="dialogVisible"
    @cancel="handleDialogCancel"
    @ok="handleDialogOk"
    @update:visible="updateDialogVisible"
  />
</template>
<script lang="ts" setup>
import AddConnectionDiag from "./components/AddConnectionDiag.vue"; // 确保路径正确
import MysqlMenu from "./components/mysql/MysqlMenu.vue"; // 确保路径正确
import RedisMenu from "./components/redis/RedisMenu.vue"; // 确保路径正确
import { PlusOutlined } from "@ant-design/icons-vue";
import { onMounted, ref } from "vue";
import MysqlView from "./components/mysql/MysqlView.vue";
import RedisView from "./components/redis/RedisView.vue";
import { useEventbus } from "./ent/events.ts";
import MongoView from "./components/mongo/MongoView.vue";
import MongoMenu from "./components/mongo/MongoMenu.vue";

const dialogVisible = ref(false);

const cur_type = ref("");
const handler_cur_type = (data: any) => {
  cur_type.value = data;
};
const handleClick = () => {
  dialogVisible.value = true;
};

const collapsed = ref<boolean>(false);
const selectedKeys = ref<string[]>(["1"]);
const eventbus = useEventbus();
const updateDialogVisible = (value) => {
  // 手动更新父组件中的 visible
  dialogVisible.value = value;
};
const handleDialogOk = async () => {
  // 处理弹框确定按钮点击事件
  console.log("处理弹框确定按钮点击事件", dialogVisible.value);
};
const handleDialogCancel = () => {
  // 处理弹框取消按钮点击事件
  console.log("处理弹框取消按钮点击事件", dialogVisible.value);
};

onMounted(async () => {});
</script>
<style scoped>
#components-layout-demo-side .logo {
  height: 32px;
  margin: 16px;
  background: rgba(255, 255, 255, 0.3);
}

.site-layout .site-layout-background {
  background: #fff;
}

[data-theme="dark"] .site-layout .site-layout-background {
  background: #141414;
}

.button-container {
  position: fixed;
  bottom: 40px;
  left: 10px;
  z-index: 999; /* Ensure it's above other elements */
}

.circle-button {
  border-radius: 50%;
}
</style>
