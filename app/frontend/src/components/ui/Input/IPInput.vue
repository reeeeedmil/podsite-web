<script setup>
  import { ref } from 'vue';
  const bytes = ref([0, 0, 0, 0])
  function checkByte(byte){
    if (byte < 0) {
      return 0
    }
    else if (byte > 255) {
      return 255
    }
    return byte
  }
</script>

<template>
  <div
    class="flex flex-row"
  >
    <input
      v-model="bytes[0]"
      class="text-smoky address-input w-16 sm:w-24 md:w-32"
      type="number"
      min="0"
      max="255"
      @input="bytes[0] = checkByte(bytes[0]); $emit('addressChange', bytes);"
    >
    <p>.</p>
    <input
      v-model="bytes[1]"
      class="text-smoky address-input w-16 sm:w-24 md:w-32"
      type="number"
      min="0"
      max="255"
      @input="bytes[1] = checkByte(bytes[1]); $emit('addressChange', bytes);"
    >
    <p>.</p>
    <input
      v-model="bytes[2]"
      class="text-smoky address-input w-16 sm:w-24 md:w-32"
      type="number"
      min="0"
      max="255"
      @input="bytes[2] = checkByte(bytes[2]); $emit('addressChange', bytes);"
    >
    <p>.</p>
    <input
      v-model="bytes[3]"
      class="text-smoky address-input w-16 sm:w-24 md:w-32"
      type="number"
      min="0"
      max="255"
      @input="bytes[3] = checkByte(bytes[3]); $emit('addressChange', bytes);"
    >
  </div>
</template>

<style scoped>
  .address-input:focus {
    background-color: rgb(213, 228, 195);
  }
</style>
