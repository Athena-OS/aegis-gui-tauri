<script lang="ts">
  import athenaLogo from "../assets/athena-logo.svg";
  import doneIcon from "../assets/icons/done.svg";

  import Button from "../lib/components/Button.svelte";

  let app_element = document.getElementById("app");
  if (app_element) {
    app_element.style.background =
      "linear-gradient(180deg, #002358 0%, #6790EB 160.63%)";
  }
</script>

<div class="z-10 relative h-screen flex flex-col items-center overflow-hidden">
  <div class="h-full flex flex-col justify-center items-center space-y-12">
    <div class="flex space-x-2 items-center justify-center">
      <h2 class="text-center text-5xl py-2">Installation Complete</h2>
      <img class="h-10 w-10" src={doneIcon} alt="" />
    </div>

    <div class="relative circle-surround">
      <img
        src={athenaLogo}
        alt="athena-logo"
        class="w-64 rounded-full border-primary-500 border-solid border-[20px]"
      />
    </div>
    <div class="flex space-x-4 w-80">
      <Button variant="bordered" fullWidth>
        <span>Close</span><span><i class="ti ti-chevron-right" /></span>
      </Button>
      <Button variant="primary" fullWidth>
        <span>Reboot</span><span><i class="ti ti-chevron-right" /></span>
      </Button>
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
