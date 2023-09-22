<script lang="ts">
    import { onMount } from 'svelte';
    import { invoke } from "@tauri-apps/api";
  
    export let switchView: (viewName: string) => void;
  
    let greetMessage = " ";
    
    async function fetchGreeting() {
      greetMessage = await invoke("greet", { name: "AthenaOS" });
    }
    
    onMount(fetchGreeting);
</script>
  
<h1>Greeting</h1>
<p>{greetMessage}</p>
<button on:click={() => switchView("initial")}>Go back</button>
  