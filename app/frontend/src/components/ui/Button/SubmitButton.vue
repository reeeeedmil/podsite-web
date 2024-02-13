<script setup>
  import { postApi } from "@c/logic/api.js";
  const props = defineProps({
    type: String,
    baseAddress: Array,
    mask: Array,
    input: Array,
  })

function createData() {
let data = new Object;
  data.baseAddress = props.baseAddress
  data.mask = props.mask
  if (props.type == "prefixes") {
      data.prefixes = props.input
    } else {
      data.hosts = props.input
      }
  return data
  }
</script>

<template>
  <button
    v-if="props.type == 'prefixes'"
    class="p-1 outline-black submit-button rounded-lg h-20 w-full text-center text-3xl md:text-4xl"
    @click="postApi('prefixes', createData())"
  >
    {{ $t('buttons.postPrefixes') }}
  </button>
  <button
    v-if="props.type == 'hosts'"
    class="p-1 outline-black submit-button rounded-lg h-20 w-full text-center text-3xl md:text-4xl"
    @click="postApi('hosts', createData())"
  >
    {{ $t('buttons.postHosts') }}
  </button>
</template>

<style scoped>
button {
  background-color: rgb(213, 228, 195);
  transition: ease-out 0.3s;
}
button {
  color: rgb(9, 12, 2);
  transition: ease-out 0.2s;
}
button:hover {
  background-color: rgb(126, 101, 81);
  transition: ease-out 0.3s;
}
button:hover {
  color: rgb(255, 255, 255);
  transition: ease-out 0.2s;
}
</style>
