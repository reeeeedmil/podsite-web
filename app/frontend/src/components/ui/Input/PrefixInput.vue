<script setup>
  import { ref } from "vue";
  const prefixes = ref([0])
  const prefixesInput = ref("")
  function handleInput(prefixes) {
    prefixes = [...prefixes.matchAll(/\d+/g)].map((num) => parseInt(num));

    while (prefixes.some((v) => 0 > v || v > 32)) {
      if (prefixes.some((v) => v > 32)) {
        prefixes.splice(prefixes.indexOf(Math.max(...prefixes)), 1)
      }
      if (prefixes.some((v) => 0 > v)) {
        prefixes.splice(prefixes.indexOf(Math.min(...prefixes)), 1)
      }
    }
    if (prefixes.length > 100) {
      prefixes = prefixes.slice(0, 100)
    }
    return prefixes.sort(function(a, b){return a-b});
  }
</script>

<template>
  <input
    v-model="prefixesInput"
    class="text-smoky address-input md:w-1/2"
    @input="prefixes = handleInput(prefixesInput); $emit('prefixesChange', prefixes)"
  >
</template>
<style scoped>
  .address-input:focus {
    background-color: rgb(213, 228, 195);
  }
</style>
