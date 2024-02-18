<script setup>
  import { ref } from 'vue';
  import { useRouter } from 'vue-router';

  import IPInput from '@c/ui/Input/IPInput.vue';
  import MaskInput from '@c/ui/Input/MaskInput.vue';
  import PrefixInput from '@c/ui/Input/PrefixInput.vue';
  import SubmitButton from '../Button/SubmitButton.vue';
  import { getUrl } from '@c/logic/api.js';


  const router = useRouter();
  const baseAddress = ref()
  const mask = ref()
  const prefixes = ref()
  function save(data) {
    sessionStorage.setItem('data', data);
  }
</script>

<template>
  <main class="bg-nyanza h-fit md:pl-8 md:pr-8 pl-4 pr-4">
    <h1 class="text-3xl md:text-4xl text-center pt-2">
      {{ $t('header.subnetCalculator') }}
    </h1>

    <div class="flex md:flex-row flex-col w-full justify-between">
      <p class="text-smoky lg:text-xl truncate">
        {{ $t('input.baseAddress') }}
      </p>
      <IPInput
        @address-change="(address) => baseAddress = address"
      />
    </div>
    <br>
    <div class="flex md:flex-row flex-col w-full justify-between">
      <p class="text-smoky lg:text-xl truncate">
        {{ $t('input.mask') }}
      </p>
      <MaskInput
        @address-change="(address) => mask = address"
      />
    </div>

    <br>
    <div class="flex md:flex-row flex-col w-full justify-between">
      <p class="text-smoky lg:text-xl truncate">
        {{ $t('input.prefix') }}
      </p>
      <PrefixInput
        @prefixes-change="(changed) => prefixes = changed"
      />
    </div>

    <div class="flex md:flex-row flex-col w-full justify-between">
      <p class="text-smoky lg:text-xl truncate">
        {{ $t('input.parsedPrefixes') }}
      </p>
      <p>{{ prefixes }}</p>
    </div>

    <SubmitButton
      :type="'prefixes'"
      :base-address="baseAddress"
      :mask="mask"
      :input="prefixes"
      @response="(response) => {router.push({path: `/subnet-calculator/net/${response.id}`});}"
    />

  </main>
</template>

<style scoped>
</style>
