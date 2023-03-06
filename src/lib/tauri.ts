import { invoke } from "@tauri-apps/api/tauri";

/**
 * Wrapper for tauri invoke functions
 */

export const setSvc = async (selectedSvc: string): Promise<void> => {
  await invoke("set_svc", { svc: selectedSvc });
};

export const setCtx = async (selectedCtx: string): Promise<void> => {
  await invoke("set_ctx", { ctx: selectedCtx });
};

export const getCtx = async (): Promise<string> => {
  return invoke("get_ctx", {});
};

export const cancelMessages = async (): Promise<void> => {
  await invoke("cancel_messages", {});
};

export const fetchContexts = async (): Promise<Array<string>> => {
  return invoke("fetch_contexts", {});
};

export const fetchServices = async (): Promise<Array<string>> => {
  return invoke("fetch_services", {});
};

export const logMessages = async (): Promise<void> => {
  await invoke("log_messages", {});
};
