<script setup>
  import { ref } from 'vue';
  import Renderer from '@c/ui/NetworkVisualizer/NetworkVisualizerRenderer.vue';
  import PrefixInput from '@c/ui/Input/PrefixInput.vue'

  const prefixes = ref()
  const basePrefix = ref()
  function changePrefixes(prefixes) {
    while (prefixes.some((v) => basePrefix.value+1 > v )) {
      prefixes.splice(prefixes.indexOf(Math.min(...prefixes)), 1)
    }
    return prefixes
    }
</script>

<template>
  <main class="bg-nyanza h-fit md:pl-8 md:pr-8 pl-4 pr-4">
    <h1 class="text-3xl md:text-4xl text-center pt-2">
      subNet
    </h1>
    <h2 class="text-2xl md:text-3xl text-center">
      {{ $t('header.networkVisualizer') }}
    </h2>
      <Renderer
        :inputPrefixes="prefixes"
        :inputBasePrefix="basePrefix"
      />
    <br>
    <div
        class="flex md:flex-row flex-col justify-between"
      >
    <p class="text-smoky lg:text-xl truncate">
    {{ $t('input.basePrefix') }}
      </p>
      <select
        v-model="basePrefix"
        class="text-smoky bg-white"
        >
        <option disabled value="">--Please choose an option--</option>
        <option
          v-for="(counter, index) in 33"
          :value="index"
        >
        {{ index }}
        </option>
      </select>
    </div>
    <br>
    <div class="flex md:flex-row flex-col w-full justify-between">
      <p class="text-smoky lg:text-xl truncate">
        {{ $t('input.prefixes') }}
      </p>
      <PrefixInput
      @prefixes-change="(changed) => prefixes = changePrefixes(changed)"

      />
    </div>

  </main>
</template>

<style scoped>
</style>
