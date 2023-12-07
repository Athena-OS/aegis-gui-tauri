<script>
  import CardGroup from "../CardGroup.svelte";
</script>

<CardGroup
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
