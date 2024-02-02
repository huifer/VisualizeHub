import { defineStore } from "pinia";

export const useMongoStore = defineStore({
  id: "mongoStore",
  state: () => ({
    cur_db_config_id: "",
  }),
  actions: {
    setDbConfigId(dbId: string) {
      this.cur_db_config_id = dbId;
    },
  },
});
