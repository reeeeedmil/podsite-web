<script setup>
  import LinkButton from '@c/ui/Button/LinkButton.vue';
</script>

<template>
  <main class="bg-nyanza h-fit md:pl-8 md:pr-8 pl-4 pr-4">
    <h1 class="text-3xl md:text-4xl text-center pt-2">Subnet calculator</h1>
    <LinkButton
      :to="'/subnet-calculator/help'" 
      :text="'Click on me to get help'"
      />
  </main> 
</template>
