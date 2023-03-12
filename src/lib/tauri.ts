import { invoke } from "@tauri-apps/api/tauri";

/**
 * Wrapper for tauri invoke functions
 */

export abstract class TauriBackend {
  setSvc: (selectedSvc: string) => Promise<void>;
  setCtx: (selectedCtx: string) => Promise<void>;
  getCtx: () => Promise<string>;
  cancelMessages: () => Promise<void>;
  fetchContexts: () => Promise<string[]>;
  fetchServices: () => Promise<string[]>;
  logMessages: () => Promise<void>;
}

export class Tauri extends TauriBackend {
  static async setSvc(selectedSvc: string): Promise<void> {
    await invoke("set_svc", { svc: selectedSvc });
  }

  static async setCtx(selectedCtx: string): Promise<void> {
    await invoke("set_ctx", { ctx: selectedCtx });
  }

  static async getCtx(): Promise<string> {
    return invoke("get_ctx", {});
  }

  static async cancelMessages(): Promise<void> {
    await invoke("cancel_messages", {});
  }

  static async fetchContexts(): Promise<Array<string>> {
    return invoke("fetch_contexts", {});
  }

  static async fetchServices(): Promise<Array<string>> {
    return invoke("fetch_services", {});
  }

  static async logMessages(): Promise<void> {
    await invoke("log_messages", {});
  }
}
