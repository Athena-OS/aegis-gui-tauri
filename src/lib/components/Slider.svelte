<script lang="ts">
  import resizeIcon from "../../assets/icons/resize-icon.svg";

  export let min = 0;
  export let max = 100;
  export let value = 0;

  $: currentValue = value;
</script>

<div class="w-full px-6 rounded-full overflow-hidden relative">
  <div class="absolute w-6 h-full bg-cyan-400 left-0"></div>
  <div class="absolute w-6 h-full bg-gray-700 right-0 top-0"></div>
  <div class="w-full relative">
    <div class="w-full pointer-events-none bg-gray-700">
      <div style="width: {currentValue}%;" class="h-[50px] bg-cyan-400"></div>
      <div
        style="left: calc({currentValue}% - 20px);"
        class="absolute top-0 bottom-0 my-auto h-10 w-10 p-0.5"
      >
        <div
          class="bg-yellow-500 h-full w-full rounded-full border-2 border-black flex justify-center items-center"
        >
          <img src={resizeIcon} alt="" />
        </div>
      </div>
    </div>
    <input
      class="slider w-full absolute top-0 bottom-0 left-0 right-0"
      type="range"
      {min}
      {max}
      bind:value
    />
  </div>
</div>

<style>
  .slider {
    -webkit-appearance: none;
    appearance: none;
    opacity: 0;
  }

  input::-webkit-slider-thumb {
    -webkit-appearance: none;
    appearance: none;
    width: 1px;
    height: 1px;
  }

  input::-moz-range-thumb {
    width: 1px;
    height: 1px;
  }
</style>
