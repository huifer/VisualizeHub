import { defineStore } from "pinia";

export const useMysqlStore = defineStore({
  id: "mysqlStore",
  state: () => ({
    mysqlData: [] as string[],
    // 当前选中的数据库配置id
    cur_db_config_id: "",
    // 当前选中的数据库名称
    cur_db_name: "",
  }),
  actions: {
    addMysqlData(data: string) {
      this.mysqlData.push(data);
    },
    removeMysqlData(index: number) {
      if (index >= 0 && index < this.mysqlData.length) {
        this.mysqlData.splice(index, 1);
      }
    },
    clearMysqlData() {
      this.mysqlData = [];
    },
    setMysqlDataArray(data: string[]) {
      this.mysqlData = data;
    },
    setDbConfigId(dbId: string) {
      this.cur_db_config_id = dbId;
    },
    setDbName(dbName: string) {
      this.cur_db_name = dbName;
    },
  },
});
