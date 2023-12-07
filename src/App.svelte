<script>
  import { Router, Route } from "svelte-routing";

  import stepsConfig from "./stepsConfig";
  import globalStore from "./lib/stores/globalStore";

  export let url = "";

  $: console.log($globalStore);
</script>

<Router {url}>
  <div class="h-screen">
    <div class="h-[calc(100%-50px)] pb-6">
      {#each stepsConfig as step}
        <Route path={step.route} component={step.component} />
      {/each}
    </div>
  </div>
</Router>
