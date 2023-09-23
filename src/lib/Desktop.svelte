<script lang="ts">
    import { invoke } from "@tauri-apps/api";
  
    export let switchView: (viewName: string) => void;
</script>
  
<h2>Environment</h2>

<button on:click={() => switchView("initial")}>Go back</button>

