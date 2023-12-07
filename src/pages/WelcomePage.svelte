<script lang="ts">
  import { Link } from "svelte-routing";
  import athenaLogo from "../assets/athena-logo.svg";
  import Button from "../lib/components/Button.svelte";
</script>

<div
  class="relative h-screen flex justify-center flex-col space-y-12 items-center overflow-hidden"
>
  <h1 class="font-semibold">Welcome to AthenaOS</h1>
  <div class="relative circle-surround">
    <img
      src={athenaLogo}
      alt="athena-logo"
      class="w-40 p-2 bg-primary-500 border-solid rounded-full"
    />
  </div>
  <Link to="/keyboard">
    <Button variant="primary">
      <span>Start installation</span><span
        ><i class="ti ti-chevron-right" /></span
      >
    </Button></Link
  >
  <div class="absolute bottom-0 left-0 w-full">
    <div
      class="w-full px-3 py-2 text-sm bg-neutral-800 flex items-center space-x-2 text-white"
    >
      <div class="bg-blue-500 h-3 w-3 rounded-full" />
      <p class="font-medium">
        We'll guide you through a few steps to install AthenaOS on your system.
      </p>
    </div>
  </div>
</div>

<style>
  .circle-surround::before {
    content: "";
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    width: 80rem;
    height: auto;
    aspect-ratio: 1/1;
    z-index: -1;
    background: url("../assets/welcome-radius.svg") no-repeat center center;
    background-size: contain;
  }
</style>
