<script lang="ts">
  import { Link } from "svelte-routing";
  import athenaLogo from "../assets/athena-logo.svg";
  import Button from "../lib/components/Button.svelte";

  let app_element = document.getElementById("app");
  if (app_element) {
    app_element.style.background =
      "linear-gradient(180deg, #002358 0%, #6790EB 160.63%)";
  }
</script>

<div
  class="z-10 relative h-screen flex justify-center flex-col space-y-12 items-center overflow-hidden"
>
  <h1 class="font-semibold">Welcome to AthenaOS</h1>
  <div class="relative circle-surround">
    <img
      src={athenaLogo}
      alt="athena-logo"
      class="w-64 rounded-full border-primary-500 border-solid border-[20px]"
    />
  </div>
  <Link
    on:click={() => {
      let app_element = document.getElementById("app");
      if (app_element) {
        app_element.style.background = "";
      }
    }}
    to="/base"
    class="w-1/6"
  >
    <Button variant="primary" fullWidth={true}>
      <span>Let's Get Started</span>
      <span>
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="15"
          height="16"
          viewBox="0 0 15 24"
          fill="none"
        >
          <path
            d="M2 2L12 12L2 22"
            stroke="black"
            stroke-width="4"
            stroke-linecap="round"
          />
        </svg>
      </span>
    </Button>
  </Link>
  <div class="absolute bottom-3 left-0 w-full px-4">
    <div
      class="rounded-full w-full px-3 py-2 text-sm bg-white flex items-center space-x-2 text-neutral-800"
    >
      <div class="bg-blue-500 h-4 w-4 rounded-full" />
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
