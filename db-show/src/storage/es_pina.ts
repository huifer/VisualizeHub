import { defineStore } from "pinia";

export const useEsStore = defineStore({
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
