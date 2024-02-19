<script setup>
  import { ref } from 'vue';
  import { postApi } from "@c/logic/api.js";
  import slugify from 'slugify'
  const props = defineProps({
    type: String,
    baseAddress: Array,
    mask: Array,
    input: Array,
    name: String,
  })
  const emit = defineEmits(["response"])
function createData() {
let data = new Object;
  data.baseAddress = props.baseAddress;
  data.mask = props.mask;

  if (props.name != "" && props.name != undefined && props.name != null) {
    data.name = slugify(props.name);
    if (props.name == undefined || props.name == null) {
      data.name == ""
    }
  }
  else {
    data.name = props.name
  }

  if (props.type == "prefixes") {
      data.prefixes = props.input
    } else {
      data.hosts = props.input
      }
  if (data.baseAddress == undefined) {
    data.baseAddress = [0, 0, 0, 0]
  }
  if (data.mask == undefined) {
    data.mask = [0, 0, 0, 0]
  }
  return data
  }
</script>

<template>
  <button
    v-if="props.type == 'prefixes'"
    class="p-1 outline-black submit-button rounded-lg h-20 w-full text-center text-3xl md:text-4xl"
    @click="postApi('prefixes', createData()).then((data) => {$emit('response', data)});"
  >
    {{ $t('buttons.postPrefixes') }}
  </button>
  <button
    v-if="props.type == 'hosts'"
    class="p-1 outline-black submit-button rounded-lg h-20 w-full text-center text-3xl md:text-4xl"
    @click="postApi('hosts', createData()).then((data) => {$emit('response', data)});"
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
