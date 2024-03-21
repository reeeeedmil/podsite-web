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
          class="flex flex-col"
          v-if="calculated"
          >
          <h3
            class="text-2xl md:text-3xl text-center text-smoky pb-3">
            {{ calculated.value.nets[0].name }}
          </h3>
            <table class="outline-ash outline-3 outline text-smoky text-lg self-center">
              <tr class="bg-nyanza-darker outline outline-ash outline-1">
                <td class="pr-3">
                  {{ $t('calculator.table.baseNet') }}
                </td>
                <td></td>
              </tr>
              <tr class="bg-nyanza outline outline-ash outline-1">
                <td class="pr-3">
                  {{ $t('calculator.table.netAddress') }}
                </td>
                <td class="text-end">{{ calculated.value.nets[0].networkAddress }}</td>
              </tr>
              <tr class="bg-nyanza-darker outline outline-ash outline-1">
                <td class="pr-3">
                  {{ $t('calculator.table.broadcast') }}
                </td>
                <td class="text-end">{{ calculated.value.nets[0].broadcast }}</td>
              </tr>
              <tr class="bg-nyanza outline outline-ash outline-1">
                <td class="pr-3">
                  {{ $t('calculator.table.mask') }}
                </td>
                <td class="text-end">{{ calculated.value.nets[0].mask }}</td>
              </tr>
              <tr class="bg-nyanza-darker outline outline-ash outline-1">
                <td class="pr-3">
                  {{ $t('calculator.table.wildcard') }}
                </td>
                <td class="text-end">{{ calculated.value.nets[0].wildcard }}</td>
              </tr>
              <tr class="bg-nyanza outline outline-ash outline-1">
                <td class="pr-3">
                  {{ $t('calculator.table.prefix') }}
                </td>
                <td class="text-end">{{ calculated.value.nets[0].prefix }}</td>
              </tr>
            </table>
        </section>
        <br>

        <section
          class="flex flex-wrap justify-center"
          v-if="calculated"
          >
          <div
            v-for="(net, key) in calculated.value.nets"
            >
            <table
              v-if="key != '0'"
              class="outline-ash outline-3 outline text-lg m-3 text-smoky"
              >
              <tr class="bg-nyanza-darker outline outline-ash outline-1">
                <td class="pr-3">
                  {{ $t('calculator.table.subnetNumber') }}
                </td>
                <td class="text-end">{{ key }}</td>
              </tr>
              <tr class="bg-nyanza outline outline-ash outline-1">
                <td class="pr-3">
                  {{ $t('calculator.table.netAddress') }}
                </td>
                <td class="text-end">{{ net.networkAddress }}</td>
              </tr>
              <tr class="bg-nyanza-darker outline outline-ash outline-1">
                <td class="pr-3">
                  {{ $t('calculator.table.broadcast') }}
                </td>
                <td class="text-end">{{ net.broadcast }}</td>
              </tr>
              <tr class="bg-nyanza outline outline-ash outline-1">
                <td class="pr-3">
                  {{ $t('calculator.table.mask') }}
                </td>
                <td class="text-end">{{ net.mask }}</td>
              </tr>
              <tr class="bg-nyanza-darker outline outline-ash outline-1">
                <td class="pr-3">
                  {{ $t('calculator.table.wildcard') }}
                </td>
                <td class="text-end">{{ net.wildcard }}</td>
              </tr>
              <tr class="bg-nyanza outline outline-ash outline-1">
                <td class="pr-3">
                  {{ $t('calculator.table.prefix') }}
                </td>
                <td class="text-end">{{ net.prefix }}</td>
              </tr>
            </table>
          </div>
        </section>
  </main>
</template>

<style scoped>
</style>
