<script setup>
  import cz from '@a/flags/cz.vue';
  import en from '@a/flags/en.vue';
  let locale = localStorage.getItem("locale")
  function changeLanguage() {
    if (localStorage.getItem("locale") == "CS") {
      localStorage.setItem("locale", "EN")
    }
    else if (localStorage.getItem("locale") == "EN"){
      localStorage.setItem("locale", "CS")
    }
    location.reload()
  }
</script>

<template>
  <button class="content-center w-12" @click="changeLanguage">
    <cz v-if="locale == 'CS'"/>
    <en v-else />
  </button>
</template>
