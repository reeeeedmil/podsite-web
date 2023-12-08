<script setup>
import api from '@c/logic/api.js';
let user
const userId = sessionStorage.getItem("user")
  user = await api('getUser', userId)
console.log(user)
</script>

<template>
    <h1 v-if="user.username != 'not logged in'" class="text-center text-xl ">Logged in as {{ user.username }}</h1>
    <h1 v-else class="text-center text-xl ">Not logged in</h1>
</template>
