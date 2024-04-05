<script setup>
  import LinkButton from '@c/ui/Button/LinkButton.vue';
  import SubnetTextAnimation from '@c/animations/SubnetTextAnimation/SubnetTextAnimation.vue';
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
      :to="'/subnet-calculator'"
      :text="this.$t('buttons.subnetCalculator')"
    />
    <LinkButton
      :to="'/network-visualizer'"
      :text="this.$t('buttons.networkVisualizer')"
    />
    <SubnetTextAnimation />
  </main>
</template>
