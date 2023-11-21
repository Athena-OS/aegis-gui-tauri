<script>
  import promotionImage from "../assets/promotion.png";
  import arrowDown from "../assets/icons/arrow-down-white.svg";
</script>

<main
  class="p-4 space-y-4 absolute top-0 left-0 right-0 overflow-hidden bg-gradient-to-tr from-blue-700 to-indigo-700 h-screen"
>
  <div class="flex flex-col items-center mx-auto w-fit space-x-6">
    <h2 class="text-center py-2">Installing</h2>
    <div class="text-lg">
      Sit back and enjoy while we install AthenaOS for you
    </div>
  </div>
  <div class="h-[calc(100%-140px)] pb-10 space-y-4">
    <div class="aspect-[38/9] rounded-xl overflow-hidden">
      <img class="object-cover h-full w-full" src={promotionImage} alt="" />
    </div>
    <div class="flex items-center space-x-2 w-full">
      <button class="px-4 py-2 bg-gray-800 rounded-lg border-gray-700 border-2"
        ><img src={arrowDown} alt="" /></button
      >
      <div class="p-4 bg-gray-800 rounded-lg border-gray-700 border-2 w-full">
        <div class="relative h-2 w-full rounded-full overflow-hidden">
          <div
            class="absolute top-0 bottom-0 left-0 my-auto w-full bg-gray-700"
          />
          <div
            class="absolute top-0 bottom-0 left-0 my-auto w-1/2 bg-primary-500"
          />
        </div>
      </div>
    </div>
    <div class="bg-gray-800 rounded-lg border-gray-700 h-full w-full" />
  </div>
</main>
