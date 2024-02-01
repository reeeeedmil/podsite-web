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
    return prefixes.sort(function(a, b){return a-b});
  }
</script>

<template>
    <input
      class="text-smoky address-input md:w-1/2"
      v-model="prefixesInput"
      @input="prefixes = handleInput(prefixesInput); $emit('prefixesChange', prefixes)"
      >
</template>
