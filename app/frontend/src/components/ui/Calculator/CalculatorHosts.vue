<script setup>
  import { ref } from 'vue';
  import { useRouter } from 'vue-router';

  import IPInput from '@c/ui/Input/IPInput.vue';
  import MaskInput from '@c/ui/Input/MaskInput.vue';
  import HostInput from '@c/ui/Input/HostInput.vue';
  import SubmitButton from '@c/ui/Button/SubmitButton.vue';
  import TitleInput from '@c/ui/Input/TitleInput.vue';


  const router = useRouter();
  const baseAddress = ref()
  const mask = ref()
  const hosts = ref()
  const name = ref()
  const error = ref()
  function checkResponse(response) {
    if (response.error == "Invalid name") {
      error.value = "Invalid name - try a different one"
    }
    else {
      if (response.name != "" && response.name != undefined && response.name != null) {
        router.push(
              {path: `/subnet-calculator/net/${response.name}`}
            )
      }

      else {
        router.push(
              {path: `/subnet-calculator/net/${response.id}`}
            )
        }
    }
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
        {{ $t('input.hosts') }}
      </p>
      <HostInput
        @hosts-change="(changed) => hosts = changed"
      />
    </div>

    <div class="flex md:flex-row flex-col w-full justify-between">
      <p class="text-smoky lg:text-xl truncate">
        {{ $t('input.parsedHosts') }}
      </p>
      <p>{{ hosts }}</p>
    </div>


    <br>
    <div class="flex md:flex-row flex-col w-full justify-between">
      <p class="text-smoky lg:text-xl truncate">
        {{ $t('input.name') }}
      </p>
      <TitleInput
        @name-change="(changed) => name = changed"
      />
    </div>
    <br>
    <SubmitButton
      :type="'hosts'"
      :base-address="baseAddress"
      :mask="mask"
      :input="hosts"
      :name="name"
      @response="(response) => {checkResponse(response)}"
    />

    <div
      v-if="error"
      class="flex md:flex-row flex-col w-full justify-between"
      >
      <p class="text-smoky lg:text-xl truncate">
        {{ $t('error.invalidName') }}
      </p>
    </div>
  </main>
</template>

<style scoped>
</style>
