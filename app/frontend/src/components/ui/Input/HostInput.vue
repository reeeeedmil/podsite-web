<script setup>
  import { ref } from "vue";
  const hosts = ref([0])
  const hostsInput = ref("")
  function handleInput(hosts) {
    hosts = [...hosts.matchAll(/\d+/g)].map((num) => parseInt(num));

    while (hosts.some((v) => 0 > v || v > Math.pow(2, 32))) {
      if (hosts.some((v) => v > 2**32)) {
        hosts.splice(hosts.indexOf(Math.max(...hosts)), 1)
      }
      if (hosts.some((v) => 0 > v)) {
        hosts.splice(prefixes.indexOf(Math.min(...hosts)), 1)
      }
    }
    if (hosts.length > 100) {
      hosts = hosts.slice(0, 100)
    }
    hosts.forEach(normalize)
    return hosts.sort(function(a, b){return b-a});
  }



function normalize(hosts, index, arr) {
  arr[index] = Math.pow(2, (hosts+1).toString(2).length)
}

</script>

<template>
  <input
    v-model="hostsInput"
    class="text-smoky address-input md:w-1/2"
    @input="hosts = handleInput(hostsInput); $emit('hostsChange', hosts)"
  >
</template>
<style scoped>
  .address-input:focus {
    background-color: rgb(213, 228, 195);
  }
</style>
