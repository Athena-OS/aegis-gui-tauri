<script lang="ts">
  import { Link } from "svelte-routing";
  import Button from "./Button.svelte";

  export let prev: string = "";
  export let next: string = "";
</script>

<div class="flex justify-between mb-0 mt-auto">
  <Link to={prev}>
    <Button>Previous</Button></Link
  >
  <Link to={next}>
    <Button>Next</Button></Link
  >
</div>
