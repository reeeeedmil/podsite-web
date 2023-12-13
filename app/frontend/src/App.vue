<script setup>
import {  RouterView } from 'vue-router'
import { getCookie } from '@s/utils/utils.js';
let cookie
try {
  cookie = getCookie('locale');
} catch (error) {
  document.cookie('locale=EN')
  cookie = 'EN';
} finally {
  const locale = `locale=${cookie}`;
  document.cookie = locale;
  sessionStorage.setItem("locale", cookie);
}


</script>

<template>
  <div class='min-h-screen w-screen bg-nyanza flex flex-col'>
    <RouterView />
  </div>
</template>

<style>
 #app {
  padding: 0;
  margin: 0;
  font-family: skyhook;
  background-color: #d5ecd4;
 }
 h1, h2, h3, h4, h5, h6, p, a {
  color: #090c02;
 }
</style>
