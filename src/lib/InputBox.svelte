<script>
    export let label = "Test Keyboard"; // default label text
    export let placeholderText = "Type to test.."; // default input placeholder text
</script>

<div class="flex w-full flex-col items-center justify-center">
    <div class="w-[28em]">
        <p class="text-[#B0B0B0] text-left font-semibold mb-2">{label}</p>
        <input class="flex flex-row items-center cursor-pointer text-left gap-4 relative w-full rounded-full bg-[#1A1A1A] py-4 pl-4 text-[#B0B0B0] font-medium border-2 border-[#2F2F2F] focus:outline-none focus:ring-2 focus:ring-[#FFB800] focus:shadow-none" placeholder="{placeholderText}" />
    </div>
</div>
