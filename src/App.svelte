<script lang="ts">
  import Logger from "./lib/Logger.svelte";
  import Tabs from "./lib/Tabs.svelte";
  import type { TabComponent } from "./lib/tabs";
  import { Tauri } from "./lib/tauri";

  let tabs: TabComponent[] = [
    { label: "Logger", value: 1, component: Logger, props: { backend: Tauri } },
    // { label: "Interactions",
    //  value: 2,
    //  component: Tab2
    // }
  ];
</script>

<main class="container">
  <h1>Welcome to Tauri!</h1>

  <!-- <div class="row">
    <Logger />
  </div> -->

  <Tabs {tabs} />
</main>
