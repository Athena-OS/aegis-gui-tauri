<script lang="ts">
    import { invoke } from "@tauri-apps/api";
  
    export let switchView: (viewName: string) => void;
</script>
  
<h1>Environment</h1>
<button on:click={() => switchView("initial")}>Go back</button>
  