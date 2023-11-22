<script>
  import promotionImage from "../assets/promotion.png";
  import arrowDown from "../assets/icons/arrow-down-white.svg";

  import { Splide, SplideSlide } from "@splidejs/svelte-splide";
  import "@splidejs/svelte-splide/css";
  import Button from "../lib/components/Button.svelte";

  let consoleOpen = true;

  let progress = 80; // in percentage
</script>

<main
  class="h-full p-4 space-y-4 absolute top-0 left-0 right-0 overflow-scroll bg-gradient-to-tr from-blue-700 to-indigo-700"
>
  <div class="text-center">
    <h2 class="text-center py-2">Installing</h2>
    <div>Sit back and enjoy while we install AthenaOS for you</div>
  </div>
  <div class="h-[calc(100%-96px)] space-y-4 flex flex-col">
    <div class="w-full relative">
      <div
        class="px-4 py-1 bg-white text-black uppercase text-sm font-medium rounded-full absolute top-2 right-2 z-20"
      >
        Promotions
      </div>
      <Splide
        class="w-full rounded-xl overflow-hidden"
        aria-label="My Favorite Images"
        options={{ type: "loop" }}
      >
        <SplideSlide>
          <div class="w-full h-full relative">
            <img
              class="object-cover w-full aspect-[38/9]"
              src={promotionImage}
              alt="Image1"
            />
            <div
              class="absolute top-0 bottom-0 left-0 right-0 h-full w-full flex p-4 bg-gradient-to-t from-black to-transparent"
            >
              <Button>Visit Sponser</Button>
            </div>
          </div>
        </SplideSlide>
        <SplideSlide>
          <div class="w-full h-full relative">
            <img
              class="object-cover w-full aspect-[38/9]"
              src={promotionImage}
              alt="Image1"
            />
            <div
              class="absolute top-0 bottom-0 left-0 right-0 h-full w-full flex p-4 bg-gradient-to-t from-black to-transparent"
            >
              <Button>Visit Sponser</Button>
            </div>
          </div>
        </SplideSlide>
      </Splide>
    </div>
    <div class="flex space-x-4">
      <button
        on:click={() => (consoleOpen = !consoleOpen)}
        class="flex items-center justify-center bg-gray-800 h-10 px-4 rounded-xl"
        ><img src={arrowDown} alt="" /></button
      >
      <div
        class="h-10 bg-gray-800 rounded-xl w-full flex items-center px-4 py-2"
      >
        <div class="relative h-2 w-full rounded-full overflow-hidden">
          <div
            class="absolute top-0 bottom-0 left-0 my-auto w-full bg-gray-700"
          />
          <div
            style="width: {progress}%;"
            class="absolute top-0 bottom-0 left-0 my-auto bg-primary-500"
          />
        </div>
      </div>
    </div>
    {#if consoleOpen}
      <div class="grow overflow-scroll bg-gray-800 rounded-xl w-full px-4 py-2">
        <pre class="w-full whitespace-pre-line">
        > 
        vite
        
        Forced re-optimization of dependencies
        
          VITE v4.4.9  ready in 1421 ms
        
          ➜  Local:   http://localhost:5173/
          ➜  Network: use --host to expose
          11:25:31 PM [vite-plugin-svelte] /Users/alken/Desktop/All/athena-installer/src/pages/PackagesPage.svelte:16:11 PackagesPage has unused export property 'switchView'. If it is for external reference only, please consider using `export const switchView`
          11:25:31 PM [vite-plugin-svelte] /Users/alken/Desktop/All/athena-installer/src/lib/components/Dropdown.svelte:5:11 Dropdown has unused export property 'additionalIcons'. If it is for external reference only, please consider using `export const additionalIcons`
    </pre>
      </div>
    {/if}
  </div>
</main>
