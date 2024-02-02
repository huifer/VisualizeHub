<template>
  <a-modal
    :visible="visible"
    title="新增链接"
    @cancel="handleCancel"
    @ok="handleOk"
  >
    <a-form
      :label-col="{ span: 8 }"
      :model="form_data"
      :wrapper-col="{ span: 16 }"
      autocomplete="off"
      name="basic"
      @finish="onFinish"
      @finishFailed="onFinishFailed"
    >
      <a-form-item label="数据库类型" name="type">
        <a-select v-model:value="form_data.type">
          <a-select-opt-group>
            <template #label>
              <span>
                <user-outlined />
                关系型数据库
              </span>
            </template>
            <a-select-option value="MySQL">MySQL</a-select-option>
          </a-select-opt-group>
          <a-select-opt-group label="缓存">
            <a-select-option value="Redis">Redis</a-select-option>
          </a-select-opt-group>
          <a-select-opt-group label="文档数据库">
            <a-select-option value="mongo">Mongo</a-select-option>
          </a-select-opt-group>
        </a-select>
      </a-form-item>
      <div v-if="form_data.type == 'Redis'">
        <a-form-item label="链接名称" name="name">
          <a-input v-model:value="form_data.redis.name" />
        </a-form-item>
        <a-form-item label="主机地址" name="mysql.host">
          <a-input v-model:value="form_data.redis.host" />
        </a-form-item>
        <a-form-item label="端口" name="port">
          <a-input v-model:value="form_data.redis.port" />
        </a-form-item>
        <a-form-item label="用户名" name="username">
          <a-input v-model:value="form_data.redis.username" />
        </a-form-item>
        <a-form-item label="密码" name="password">
          <a-input-password v-model:value="form_data.redis.password" />
        </a-form-item>
      </div>

      <div v-if="form_data.type == 'MySQL'">
        <a-form-item label="链接名称" name="name">
          <a-input v-model:value="form_data.mysql.name" />
        </a-form-item>
        <a-form-item label="主机地址" name="mysql.host">
          <a-input v-model:value="form_data.mysql.host" />
        </a-form-item>
        <a-form-item label="端口" name="port">
          <a-input v-model:value="form_data.mysql.port" />
        </a-form-item>
        <a-form-item label="用户名" name="username">
          <a-input v-model:value="form_data.mysql.username" />
        </a-form-item>
        <a-form-item label="密码" name="password">
          <a-input-password v-model:value="form_data.mysql.password" />
        </a-form-item>
      </div>

      <div v-if="form_data.type == 'mongo'">
        <a-form-item label="链接名称" name="name">
          <a-input v-model:value="form_data.mongo.name" />
        </a-form-item>
        <a-form-item label="主机地址" name="mysql.host">
          <a-input v-model:value="form_data.mongo.host" />
        </a-form-item>
        <a-form-item label="端口" name="port">
          <a-input v-model:value="form_data.mongo.port" />
        </a-form-item>
        <a-form-item label="用户名" name="username">
          <a-input v-model:value="form_data.mongo.username" />
        </a-form-item>
        <a-form-item label="密码" name="password">
          <a-input-password v-model:value="form_data.mongo.password" />
        </a-form-item>
      </div>
    </a-form>
  </a-modal>
</template>

<script lang="ts">
import { Modal } from "ant-design-vue";
import { invoke } from "@tauri-apps/api/tauri";
import { useEventbus } from "../ent/events.ts";

const eventbus = useEventbus();
export default {
  components: {
    AModal: Modal,
  },
  props: {
    visible: Boolean,
  },
  data() {
    return {
      form_data: {
        type: "",
        mysql: {
          username: "",
          password: "",
          host: "",
          port: 3306,
          name: "",
        },
        redis: {
          username: "",
          password: "",
          host: "",
          port: 6379,
          name: "",
        },
        mongo: {
          username: "",
          password: "",
          host: "",
          port: 27017,
          name: "",
        },
      },
    };
  },
  methods: {
    onFinishFailed(errorInfo: any) {
      console.log("Failed:", errorInfo);
    },
    onFinish(values: any) {
      console.log("Success:", values);
    },
    async handleOk() {
      this.form_data.mysql.port = Number(this.form_data.mysql?.port);
      this.form_data.redis.port = Number(this.form_data.redis?.port);
      this.form_data.mongo.port = Number(this.form_data.mongo?.port);
      let resp = null;
      let dt = this.form_data.type;
      if (this.form_data.type === "MySQL") {
        resp = await invoke("add_mysql_config", {
          param: this.form_data.mysql,
        });
      } else if (this.form_data.type === "Redis") {
        resp = await invoke("add_redis_config", {
          param: this.form_data.redis,
        });
      } else if (this.form_data.type === "mongo") {
        resp = await invoke("add_mongo_config", {
          param: this.form_data.mongo,
        });
      }
      if (resp.status_code === 20000) {
        eventbus.customEmit(dt + ":created");
      }

      console.log(resp);
      // 处理确定按钮点击事件
      this.$emit("ok");
      this.$emit("update:visible", false); // 手动更新 visible
      console.log("处理确定按钮点击事件", this.visible);
    },
    handleCancel() {
      // 处理取消按钮点击事件
      this.$emit("cancel");
      this.$emit("update:visible", false); // 手动更新 visible

      console.log("处理取消按钮点击事件", this.visible);
    },
  },
};
</script>

<style scoped>
/* 在这里可以添加样式 */
</style>
