import { onUnmounted } from "vue";
import mitt from "mitt";

type IUseEventbus = {
  customEmit: (eventName: string) => void;
  customOn: (eventName: string, callback: (data: any) => void) => void;
};

const emitter: mitt.Emitter = mitt();

/**
 * @description: 自定义触发器
 * @param {*} eventName 名称
 */
const customEmit = (eventName: string, args: any) => {
  emitter.emit(eventName, args);
};

/**
 * @description: 自定义接收器
 * @param {*} name 名称
 * @param {*} callback 回调的函数
 */
const customOn = (eventName: string, callback: (data: any) => void) => {
  emitter.on(eventName, (data: any) => callback(data));
};

/**
 * @description: 导出useEventbus
 */
export const useEventbus = (): IUseEventbus => {
  // 销毁的事件
  onUnmounted(() => {
    // 清空所有的事件，避免多组件互相清理
    emitter.all.clear();
  });

  return {
    customEmit,
    customOn,
  };
};
