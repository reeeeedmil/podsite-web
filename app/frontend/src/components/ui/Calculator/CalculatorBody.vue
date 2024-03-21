<script setup>
  import LinkButton from '@c/ui/Button/LinkButton.vue';
</script>

<template>
  <main class="bg-nyanza h-fit md:pl-8 md:pr-8 pl-4 pr-4">
    <h1 class="text-3xl md:text-4xl text-center pt-2">
      subNet
    </h1>
    <h2 class="text-2xl md:text-3xl text-center">
      {{ $t('homeSubtitle') }}
    </h2>
    <LinkButton
      :to="'/subnet-calculator/prefixes'"
      :text="this.$t('buttons.calculator.prefixes')"
    />
    <LinkButton
      :to="'/subnet-calculator/hosts'"
      :text="this.$t('buttons.calculator.hosts')"
    />
  </main>
</template>

<style scoped>
</style>
