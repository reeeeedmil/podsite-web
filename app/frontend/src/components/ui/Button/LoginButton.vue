<script setup>
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import api from '@c/logic/api.js';
const log = ref(false)

const props = defineProps({
  route: {
    type: String,
    default: '/',
  }
})

const router = useRouter();

let form = ref({
	username: "",
	password: "",
})
async function submitForm() {
	let response = await api("getToken", form.value.username, form.value.password)
  if (typeof(response.user_id) == 'number') {
    sessionStorage.setItem('user', response.user_id)
    router.push('/')
  }
  else {
    log.value = true  
  }
}
</script>

<template>
  <form @submit.prevent="submitForm">
    <input id="UsernameAuth" type="text" placeholder="Username" v-model="form.username">
    <input id="PassAuth" type="password" placeholder="Password" v-model="form.password">
    <button class="Login" color="transparent" @click="submitForm">Login</button>
  </form>
  <h1 class="text-rufous text-center text-1xl" :class="log? 'block' : 'hidden'">Invalid login</h1>
</template>

<style scoped>
.logo {
	margin: auto;
	width: 256px;

}

p {
	color: white;
	font-size: 8vw;
}

.welcome {
	margin-top: 40px;
}

input,
ion-button {
	margin: 10px 0;
	width: 80%;
	height: 50px;
	border-radius: 10px;

}

input {
	padding: 0 10px;
	background-color: #242424;
	color: white;
	text-align: left;
}

.Login {
	background-color: #e6e109;
	color: black;
	font-size: 4vw;
	font-weight: bold;

}

.login-by-other {
	font-weight: bold;
	background-color: white;
	color: black;
}

.sign-img {
	width: 40px;
	height: 40px;
	margin: 20px 10px;

}

.sign-up {
	margin-bottom: 50px;
}
</style>
