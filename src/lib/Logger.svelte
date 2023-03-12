<script lang="ts">
  import { onMount } from "svelte";
  import { emit, listen } from "@tauri-apps/api/event";
  import type { Event } from "@tauri-apps/api/event";
  import type { TauriBackend } from "./tauri";

  export let backend: TauriBackend;
  let ctxs = [];
  let svcs = [];

  let selectedCtx = "";
  let selectedSvc = "";
  let messages: string[] = [];
  let logElement: HTMLDivElement;

  export async function onMountHandle() {
    ctxs = await backend.fetchContexts();
    svcs = await backend.fetchServices();
  }

  export function introspectState() {
    return {
      ctxs,
      svcs,
      selectedCtx,
      selectedSvc,
      messages,
    };
  }

  async function subscribeToLogs() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    selectedCtx = await backend.getCtx();
    messages = [];
    await backend.logMessages();
    const unlisten = await listen("onMessage", (event: Event<string>) => {
      // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
      // event.payload is the payload object
      messages.push(
        ...event.payload
          .split("\n")
          .map((x) => x.trimEnd())
          .filter((x) => x)
      );
      messages = messages;
      scrollToBottom(logElement);
    });
  }

  async function handleCtxChange() {
    await backend.cancelMessages();
    await backend.setCtx(selectedCtx);
    console.log({ selectedCtx });
  }

  async function handleSvcChange() {
    await backend.cancelMessages();
    await backend.setSvc(selectedSvc);
    console.log({ selectedSvc });
  }

  function clearMessages() {
    messages = [];
  }

  const scrollToBottom = async (node: HTMLDivElement) => {
    node.scroll({ top: node.scrollHeight, behavior: "smooth" });
  };

  onMount(onMountHandle);
</script>

<div>
  <div class="row">
    <form on:change|preventDefault={handleCtxChange}>
      <select
        bind:value={selectedCtx}
        on:change={() => {}}
        data-testid="logger-ctx"
      >
        {#each ctxs as ctx}
          <option value={ctx}>
            {ctx}
          </option>
        {/each}
      </select>
    </form>
    <form on:change|preventDefault={handleSvcChange}>
      <select
        bind:value={selectedSvc}
        on:change={() => {}}
        data-testid="logger-svc"
      >
        {#each svcs as svc}
          <option value={svc}>
            {svc}
          </option>
        {/each}
      </select>
    </form>
  </div>

  <div class="row">
    <button on:click={subscribeToLogs}> Log messages </button>
  </div>
  <div class="row">
    <code>{selectedCtx}</code>
  </div>
  <div
    bind:this={logElement}
    style="height:500px;overflow:auto;text-align:left;"
  >
    {#each messages as message}
      <div class="row">
        <pre>{message}</pre>
      </div>
    {/each}
  </div>
  <div class="row">
    <button on:click={clearMessages}> Clear messages </button>
    <button on:click={backend.cancelMessages}> Cancel messages </button>
  </div>
</div>
