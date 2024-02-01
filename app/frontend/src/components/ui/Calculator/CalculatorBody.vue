<script setup>
  import { ref } from 'vue';
  import IPInput from '@c/ui/Input/IPInput.vue';
  import MaskInput from '@c/ui/Input/MaskInput.vue';
  import PrefixInput from '@c/ui/Input/PrefixInput.vue';
  const baseAddress = ref()
  const mask = ref()
  const prefixes = ref()
</script>

<template>
  <main class="bg-nyanza h-fit md:pl-8 md:pr-8 pl-4 pr-4">
    <h1 class="text-3xl md:text-4xl text-center pt-2">Subnet calculator</h1>

    <div class="flex md:flex-row flex-col w-full justify-between">
      <p class="text-smoky lg:text-xl truncate">Base network address:</p>
      <IPInput
        @address-change="(address) => baseAddress = address"
        />
    </div>
    <br>
    <div class="flex md:flex-row flex-col w-full justify-between">
      <p class="text-smoky lg:text-xl truncate">Mask:</p>
      <MaskInput
        @address-change="(address) => mask = address"
        />
    </div>

    <br>
    <div class="flex md:flex-row flex-col w-full justify-between">
      <p class="text-smoky lg:text-xl truncate">Prefix input: </p>
      <PrefixInput
        @prefixes-change="(changed) => prefixes = changed"
        />
    </div>

    <div class="flex md:flex-row flex-col w-full justify-between">
      <p class="text-smoky lg:text-xl truncate">Parsed prefixes: </p>
      <p>{{ prefixes }}</p>
    </div>

  </main>
</template>

<style scoped>
</style>
