import type { ComponentType, SvelteComponentTyped } from "svelte";

export interface TabComponent {
  label: string;
  value: number;
  component: ComponentType<SvelteComponentTyped>;
  props?: object;
}
