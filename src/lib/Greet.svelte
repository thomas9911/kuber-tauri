<script lang="ts">
  import { onMount } from "svelte";
  import { emit, listen } from "@tauri-apps/api/event";
  import type { Event } from "@tauri-apps/api/event";
  import {
    cancelMessages,
    fetchContexts,
    fetchServices,
    getCtx,
    setCtx,
    setSvc,
  } from "./tauri";

  let ctxs = [];
  let svcs = [];

  let selectedCtx = "";
  let selectedSvc = "";
  let messages: Array<string> = [];
  let logElement: HTMLDivElement;

  async function logMessages() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    selectedCtx = await getCtx();
    messages = [];
    await logMessages();
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
    await setCtx(selectedCtx);
    console.log({ selectedCtx });
  }

  async function handleSvcChange() {
    await setSvc(selectedSvc);
    console.log({ selectedSvc });
  }

  const scrollToBottom = async (node) => {
    node.scroll({ top: node.scrollHeight, behavior: "smooth" });
  };

  onMount(async () => {
    ctxs = await fetchContexts();
    svcs = await fetchServices();
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
