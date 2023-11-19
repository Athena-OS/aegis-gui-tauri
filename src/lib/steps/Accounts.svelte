<script lang="ts">
  import userIcon from "../../assets/icons/user-yellow.svg";
  import plusIcon from "../../assets/icons/plus-yellow-icon.svg";
  import StepWrapper from "../components/StepWrapper.svelte";
</script>

<StepWrapper
  title="Select Keyboard"
  dialogTitle="Header Here"
  dialogContent="Your text here"
>
  <div class="flex flex-col items-center">
    <div
      class="flex flex-col item-center bg-[#1A1A1A] w-[32em] h-[22em] rounded-2xl mt-16 p-4"
    >
      <div class="flex flex-row items-center justify-between">
        <div class="flex flex-row items-center gap-2">
          <img src={userIcon} alt="user" />
          <h3 class="font-semibold">Accounts</h3>
        </div>
        <img src={plusIcon} alt="plus" />
      </div>
      <h4
        class="text-center mx-auto my-auto text-4xl max-w-md text-[#B0B0B0] font-medium"
      >
        Create Users to Continue...
      </h4>
    </div>
  </div>
</StepWrapper>
