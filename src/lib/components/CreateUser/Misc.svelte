<script>
  import Switch from "../Switch.svelte";
  import accountsStore from "../../stores/accountsStore";
</script>

<div class="space-y-2">
  <div class="text-red-400 font-medium">Configure Permissions</div>
  <ul class=" divide-y divide-neutral-700">
    <li>
      <span>Allow ROOT Access</span>
      <Switch bind:value={$accountsStore.createNewUserTemp.hasroot} />
    </li>
    <!--li>
      <span>Permission 2</span>
      <Switch />
    </li>
    <li>
      <span>Permission 3</span>
      <Switch />
    </li>
    <li>
      <span>Permission 4</span>
      <Switch />
    </li>
    <li>
      <span>Permission 5</span>
      <Switch />
    </li-->
  </ul>
</div>

<style>
  li {
    @apply flex items-center justify-between py-2;
  }
</style>
