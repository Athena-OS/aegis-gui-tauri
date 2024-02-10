<script>
  import CardGroup from "../CardGroup.svelte";
  import partitionStore from "../../stores/partitionStore";
</script>

<CardGroup
  on:change={(e) => {
    // $partitionStore.newPartition.type = e.detail.target.value;
  }}
  title="Method of Partition"
  warning={false}
  cards={[
    {
      title: "MBR",
      desc: "For legacy operating systems. Ex: Windows XP",
      value: "mbr",
      checked: true,
    },
    {
      title: "GPT",
      desc: "Recommended in most cases.",
      value: "gpt",
    },
  ]}
/>
