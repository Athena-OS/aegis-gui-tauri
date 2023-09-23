<script lang="ts">
    import { invoke } from "@tauri-apps/api";
  
    export let switchView: (viewName: string) => void;
</script>
  
<h2>Keyboard layout</h2>

<div class="container">
    <button on:click={() => switchView("initial")}>Back</button>
    <button on:click={() => switchView("desktop")}>Next: Dektop Environment</button>
</div>

