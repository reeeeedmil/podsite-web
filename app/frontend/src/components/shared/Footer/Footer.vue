<script setup>
import FooterCalls from './FooterCalls.vue';
</script>

<template>
  <footer class="flex mt-auto ml-0 mr-0 h-20 bg-vanilla-base">
    <Suspense>
      <template #default>
        <FooterCalls />
      </template>
    </Suspense>
  </footer>
</template>
