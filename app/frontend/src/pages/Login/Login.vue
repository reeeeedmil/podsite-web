<script setup>
import Header from '@c/shared/Header/Header.vue';
import Footer from '@c/shared/Footer/Footer.vue';
import LoginButton from '@c/ui/Button/LoginButton.vue';
</script>


<template>
  <Header />
  <LoginButton />
  <Footer />
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
