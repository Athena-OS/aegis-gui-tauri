<script lang="ts">
  import { Link, useLocation } from "svelte-routing";
  import Button from "./Button.svelte";
  import stepsConfig from "../../stepsConfig";

  export let prev: string = "";
  export let next: string = "";

  let location = useLocation();

  let index = stepsConfig.findIndex((x) => x.route === $location.pathname);
</script>

<div
  class="flex justify-between items-center absolute bottom-0 left-0 right-0 w-full p-6"
>
  <div class="w-40">
    <Link to={prev}>
      <Button fullWidth>Previous</Button></Link
    >
  </div>
  <div class="flex items-center space-x-2">
    <div class="h-2 w-2 bg-gray-500 rounded-full"></div>
    <div class="h-4 w-4 bg-primary-500 rounded-full"></div>
    <div class="h-2 w-2 bg-gray-500 rounded-full"></div>
    <div class="h-2 w-2 bg-gray-500 rounded-full"></div>
  </div>
  <div
    class="flex flex-col items-center justify-center space-y-2 w-40 relative"
  >
    <div class="text-primary-500 font-medium absolute bottom-14">
      Step {index} of 9
    </div>
    <Link class="w-full" to={next}>
      <Button fullWidth>Next</Button></Link
    >
  </div>
</div>
