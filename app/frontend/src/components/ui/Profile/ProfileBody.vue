<script setup>
import api from '@c/logic/api.js';
import { useRouter } from 'vue-router';
import LinkButton from '@c/ui/Button/LinkButton.vue';
let response = await api("confirmAuth",)
let user
const router = useRouter();
if (typeof(response.log) == 'number') {
  user = await api("getUser", response.log)
  sessionStorage.setItem("username", user.username)
  sessionStorage.setItem("id", user.id)
}
else {
  router.push({
    path: '/login',
  })
}

</script>

<template>
  <h1
    class="text-center text-3xl text-smoky"
    >
    {{ user.username }}
  </h1>
  <LinkButton 
    :to="'/profile/my-networks'"
    :text="'My networks'"
  />
</template>
