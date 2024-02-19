<script setup>
  import { ref } from "vue";
  const name = ref()
</script>

<template>
  <input
    v-model="name"
    type="text"
    class="text-smoky address-input md:w-1/2"
    @input="$emit('nameChange', name)"
  >
</template>
<style scoped>
  .address-input:focus {
    background-color: rgb(213, 228, 195);
  }
</style>
