<script setup>
const props = defineProps({
  buttonType: {
    type: String,
    default: 'base',
  },
  color: {
    type: String,
    default: 'vanilla-light',
  },
  to: {
    type: String,
    default: '/',
    required: true,
  },
  text: {
    type: String,
    default: 'button-text',
    required: true,
  },
})
</script>

<template>
  <router-link
    :to="props.to"
    class="m-5"
  >
    <div class="rounded-lg h-20 w-full text-center table">
      <h1
        class="text-3xl md:text-4xl text-center table-cell align-middle"
      >
        {{ props.text }}
      </h1>
    </div>
  </router-link>
</template>

<style scoped>
div {
  background-color: rgb(213, 228, 195);
  transition: ease 0.3s;
}
h1 {
  color: rgb(9, 12, 2);
  transition: ease 0.2s;
}
div:hover {
  background-color: rgb(126, 101, 81);
  transition: ease 0.3s;
}
h1:hover {
  color: rgb(255, 255, 255);
  transition: ease 0.2s;
}
</style>
