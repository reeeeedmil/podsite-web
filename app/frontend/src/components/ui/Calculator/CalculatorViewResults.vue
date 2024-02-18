<script setup>
  import { ref, onBeforeMount } from 'vue';
  import { useRoute } from 'vue-router';

  import { api } from "@c/logic/api.js";

  const id = useRoute().params.id;
  const calculated = ref()
  async function getNets() {
    const result = ref(await api("getNets", id))
    console.log(result)
    return await result
  }
  onBeforeMount( async () => {
     calculated.value = await getNets()
  })
</script>

<template>
  <main class="bg-nyanza h-fit md:pl-8 md:pr-8 pl-4 pr-4">
    <h1 class="text-3xl md:text-4xl text-center pt-2">
      subNet
    </h1>
    <h2 class="text-2xl md:text-3xl text-center">
      {{ $t('calculator.created') }}
    </h2>
        <section
          class="m-5 flex flex-wrap"
          v-if="calculated"
          >
        <div
          v-for="(net, key) in calculated.value.nets"
          >
          <div
            v-if="key != '0'"
            class="text-lg p-3 text-smoky"
            >
            <table class="outline-ash outline-3 outline">
              <tr>
              </tr>
              <tr class="bg-nyanza-darker outline outline-ash outline-1">
                <td class="pr-3">Subnet number</td>
                <td class="text-end">{{ key }}</td>
              </tr>
              <tr class="bg-nyanza outline outline-ash outline-1">
                <td class="pr-3">Network address</td>
                <td class="text-end">{{ net.networkAddress }}</td>
              </tr>
              <tr class="bg-nyanza-darker outline outline-ash outline-1">
                <td class="pr-3">Broadcast</td>
                <td class="text-end">{{ net.broadcast }}</td>
              </tr>
              <tr class="bg-nyanza outline outline-ash outline-1">
                <td class="pr-3">Mask</td>
                <td class="text-end">{{ net.mask }}</td>
              </tr>
              <tr class="bg-nyanza-darker outline outline-ash outline-1">
                <td class="pr-3">Wildcard</td>
                <td class="text-end">{{ net.wildcard }}</td>
              </tr>
              <tr class="bg-nyanza outline outline-ash outline-1">
                <td class="pr-3">Prefix</td>
                <td class="text-end">{{ net.prefix }}</td>
              </tr>
            </table>
          </div>

        </div>
        </section>
  </main>
</template>

<style scoped>
</style>
