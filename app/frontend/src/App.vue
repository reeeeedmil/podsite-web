<script setup>
import {  RouterView } from 'vue-router'
let lang = localStorage.getItem('locale');
if ([null, "null", undefined, "undefined"].includes(lang)) {
  localStorage.setItem('locale', 'EN')
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
