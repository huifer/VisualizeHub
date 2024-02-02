import { defineStore } from "pinia";

export const useRedisStore = defineStore({
  id: "redisStore",
  state: () => ({
    cur_db_config_id: "",
    cur_db_index: "",
  }),
  actions: {
    setDbConfigId(dbId: string) {
      this.cur_db_config_id = dbId;
    },

    setDbIndexId(dbIndex: string) {
      this.cur_db_index = dbIndex;
    },
  },
});
