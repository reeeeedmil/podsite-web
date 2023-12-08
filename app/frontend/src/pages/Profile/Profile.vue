<script setup>
import Header from '@c/shared/Header/Header.vue';
import Footer from '@c/shared/Footer/Footer.vue';
import Body from '@c/ui/Profile/ProfileBody.vue';
  
</script>

<template>
  <Header />
  <Suspense>
    <template #default>
      <Body />
    </template>
    <template #fallback>
      ahoj
    </template>
  </Suspense>
  <Footer />
</template>
