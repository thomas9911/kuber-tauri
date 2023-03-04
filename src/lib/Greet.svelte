<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";
  import { emit, listen } from "@tauri-apps/api/event";
  import type { Event } from "@tauri-apps/api/event";

  let ctxs = [];
  let svcs = [];

  let selectedCtx = "";
  let selectedSvc = "";
  // let logText = "Placeholder";
  let messages: Array<string> = [];
  let logElement;

  async function logMessages() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    selectedCtx = await invoke("get_ctx", {});
    messages = [];
    await invoke("log_messages", {});
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

  async function cancelMessages() {
    await invoke("cancel_messages", {});
  }

  async function handleCtxChange() {
    await invoke("set_ctx", { ctx: selectedCtx });
    console.log({ selectedCtx });
  }

  async function handleSvcChange() {
    await invoke("set_svc", { svc: selectedSvc });
    console.log({ selectedSvc });
  }

  const scrollToBottom = async (node) => {
    node.scroll({ top: node.scrollHeight, behavior: "smooth" });
  };

  onMount(async () => {
    ctxs = await invoke("fetch_contexts", {});
    svcs = await invoke("fetch_services", {});
  });
</script>

<div>
  <div class="row">
    <form on:change|preventDefault={handleCtxChange}>
      <select bind:value={selectedCtx} on:change={() => {}}>
        {#each ctxs as ctx}
          <option value={ctx}>
            {ctx}
          </option>
        {/each}
      </select>
    </form>
    <form on:change|preventDefault={handleSvcChange}>
      <select bind:value={selectedSvc} on:change={() => {}}>
        {#each svcs as svc}
          <option value={svc}>
            {svc}
          </option>
        {/each}
      </select>
    </form>
  </div>

  <div class="row">
    <button on:click={logMessages}> Log messages </button>
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
    <button on:click={() => (messages = [])}> Clear messages </button>
    <button on:click={cancelMessages}> Cancel messages </button>
  </div>
</div>
