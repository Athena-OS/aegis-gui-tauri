<script>
  import { createTabs } from "svelte-headlessui";
  import InputBox from "../InputBox.svelte";

  const tabs = createTabs({ selected: "Same as ROOT" });
  const keys = ["Same as ROOT", "CUSTOM"];
</script>

<div class="flex w-full flex-col items-center justify-center mt-8">
  <div class="w-full space-y-2">
    <div class="font-medium text-neutral-400">Password Options</div>
    <div use:tabs.list class="flex space-x-4 rounded-full p-2 bg-neutral-700">
      {#each keys as value}
        {@const active = $tabs.active === value}
        <button
          class:active
          class="h-[50px] rounded-full w-full font-medium"
          use:tabs.tab={{ value }}>{value}</button
        >
      {/each}
    </div>
    <div>
      {#each keys as value}
        <div use:tabs.panel={{}}>
          {#if $tabs.selected === value && $tabs.selected === "Same as ROOT"}
            <ul class="p-2 bg-neutral-700 rounded-xl text-neutral-400">
              Continue
            </ul>
          {:else if $tabs.selected === value && $tabs.selected === "CUSTOM"}
            <div class="space-y-4">
              <InputBox
                label="Password"
                placeholderText="Enter your password"
              />
              <InputBox
                label="Confirm Password"
                placeholderText="Confirm your password"
              />
            </div>
          {/if}
        </div>
      {/each}
    </div>
  </div>
</div>

<style>
  .active {
    @apply bg-yellow-500 text-black;
  }
</style>
