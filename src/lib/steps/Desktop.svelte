<script lang="ts">
    import { invoke } from "@tauri-apps/api";
  
    export let switchView: (viewName: string) => void;
</script>
  
<h2>Environment</h2>

<div class="container">
    <button on:click={() => switchView("keyboard")}>Back</button>
    <button on:click={() => switchView("initial")}>Next</button>
</div>

