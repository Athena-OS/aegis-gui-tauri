<script>
  import { createTabs } from "svelte-headlessui";
  import InputBox from "../InputBox.svelte";

  import accountsStore from "../../stores/accountsStore";

  let tabs = createTabs({ selected: "CUSTOM" });
  let keys = ["Same as ROOT", "CUSTOM"];
  if (
    $accountsStore.users.length <= 0 ||
    $accountsStore.users.filter((item) => item.hasroot === true).length <= 0
  ) {
    tabs = createTabs({ selected: "CUSTOM" });
    keys = ["CUSTOM"];
  }

  $: $tabs.selected, onTabsSelected();
  function onTabsSelected() {
    if ($tabs.selected === "CUSTOM") {
      $accountsStore.createNewUserTemp.passwordSameAsRoot = false;
    } else {
      $accountsStore.createNewUserTemp.passwordSameAsRoot = true;
    }
  }
</script>

<div class="flex w-full flex-col items-center justify-center">
  <div class="w-full space-y-2">
    {#if $accountsStore.users.filter((item) => item.hasroot === true).length > 0}
      <div class="font-medium text-neutral-400">Password Options</div>
      <div
        use:tabs.list
        class="flex space-x-4 rounded-full p-2 bg-gray-700 h-[50px] text-sm"
      >
        {#each keys as value}
          {@const active = $tabs.active === value}
          <button
            class:active
            class="h-full rounded-full w-full font-medium"
            use:tabs.tab={{ value }}>{value}</button
          >
        {/each}
      </div>
    {/if}
    <div>
      {#if $accountsStore.users.filter((item) => item.hasroot === true).length > 0}
        {#each keys as value}
          <div use:tabs.panel={{}}>
            {#if $tabs.selected === value && $tabs.selected === "CUSTOM"}
              <div class="space-y-4">
                <InputBox
                  bind:value={$accountsStore.createNewUserTemp.password}
                  inputType="password"
                  label="Password"
                  placeholderText="Enter your password"
                />
                <InputBox
                  bind:value={$accountsStore.createNewUserTemp.confirmPassword}
                  inputType="password"
                  label="Confirm Password"
                  placeholderText="Confirm your password"
                />
              </div>
            {/if}
          </div>
        {/each}
      {:else}
        <div class="space-y-4">
          <InputBox
            bind:value={$accountsStore.createNewUserTemp.password}
            inputType="password"
            label="Password"
            placeholderText="Enter your password"
          />
          <InputBox
            bind:value={$accountsStore.createNewUserTemp.confirmPassword}
            inputType="password"
            label="Confirm Password"
            placeholderText="Confirm your password"
          />
        </div>
      {/if}
    </div>
  </div>
</div>

<style>
  .active {
    @apply bg-yellow-500 text-black;
  }
</style>
