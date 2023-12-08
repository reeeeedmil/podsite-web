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
    <router-link :to="props.to" class="m-5">
      <div class="rounded-lg h-20 w-full text-center table" :class="`bg-${props.color}`">
        <h1 
          class="text-3xl md:text-4xl text-center table-cell align-middle"
        >
          {{ props.text }}
        </h1>
      </div>
    </router-link>
</template>
