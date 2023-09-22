<script lang="ts">
  import InitialView from './lib/Welcome.svelte';
  import GreetingView from './lib/InstallingOne.svelte';

  let view = "initial";

  function switchView(targetView: string): void {
    view = targetView;
  }
</script>

{#if view === "initial"}
  <InitialView {switchView} />
{:else if view === "greeting"}
  <GreetingView {switchView} />
{/if}