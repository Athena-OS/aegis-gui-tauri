<script lang="ts">
  import athenaLogo from "../../assets/athena-logo.svg";
  import InfoBar from "../components/InfoBar.svelte";
  import Button from "../components/Button.svelte";
  import currentActive from "../stepsStore";
</script>

<div
  class="relative h-screen flex justify-center flex-col space-y-12 items-center overflow-hidden"
>
  <h1 class="font-semibold">Welcome to AthenaOS</h1>
  <div class="relative circle-surround">
    <img
      src={athenaLogo}
      alt="athena-logo"
      class="w-40 p-2 bg-yellow-500 border-solid rounded-full"
    />
  </div>
  <Button variant="primary" on:click={() => ($currentActive = 1)}>
    <span>Start installation</span><span><i class="ti ti-chevron-right" /></span
    >
  </Button>
  <InfoBar accent="info" />
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
    background: url("../../assets/welcome-radius.svg") no-repeat center center;
    background-size: contain;
  }
</style>
