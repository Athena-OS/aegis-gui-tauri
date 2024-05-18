<script lang="ts">
  import { Router, Route } from "svelte-routing";

  import stepsConfig from "./stepsConfig";
  import globalStore from "./lib/stores/globalStore";
  import { listen } from "@tauri-apps/api/event";
  import logStore from "./lib/stores/logStore";

  // listen to log event and save it to log store
  /*listen("log", (event) => {
    console.log("Received tracing event:", event.payload);
    logStore.update((current) => {
      current.logs.push(event.payload as string);
      return current;
    });
  });*/

  // update install progress
  listen("percentage", (event) => {
    $logStore.progress = parseInt(event.payload as string);
  });

  // listeen to install fail event
  listen("install-fail", (event) => {
    console.log("Received install failed event:", event.payload);
    $logStore.installFailed = true
  });
  // listeen to install fail event
  listen("install-success", (event) => {
    console.log("Received install success event:", event.payload);
    $logStore.installSuccess = true
  });
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
