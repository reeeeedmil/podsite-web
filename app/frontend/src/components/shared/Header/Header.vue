<script setup>
import { RouterLink } from 'vue-router';
import { onMounted, ref } from 'vue';

import Burger from '@c/ui/Burger/Burger.vue';


const isOpened = ref(false)
onMounted (() => {
  document.getElementById('burger-menu').classList.add('hidden')
})
</script>

<template>
  <header class='bg-vanilla-base h-16 w-screen top-0'>
    <nav class='hidden md:flex h-full'>
      <div class='h-full bg-coyote w-1/6 flex'>
        <router-link to="/" class="text-3xl text-smoky  flex justify-center items-center w-full text-center">
          subNet
        </router-link>
      </div>

      <router-link to="/profile" class="text-3xl text-smoky flex  justify-center items-center w-1/6 text-center">
        {{ $t('header.profile') }}
      </router-link>

      <router-link to="/subnet-calculator" class="text-3xl text-smoky  flex justify-center items-center w-1/3 text-center">
        {{ $t('header.subnetCalculator') }}
      </router-link>

      <router-link to="/network-visualizer" class="text-3xl text-smoky  flex justify-center items-center w-1/3 text-center">
        {{ $t('header.networkVisualizer') }}
      </router-link>

      <button id="locale" class="w-1/12 bg-rufous">
        <img />
      </button>
    </nav>
    <nav class='flex md:hidden bg-vanilla-base w-full h-full justify-between'>

      <div class='h-full bg-coyote w-1/6 flex'>
        <router-link to="/" class="text-3xl text-smoky  flex justify-center items-center w-full text-center">
          subNet
        </router-link>

      </div>
      <div class="w-full"/>
      <button class="w-1/5 bg-rufous h-full justify-end ">
        text
        <img />
      </button>

      <Burger 
        @change-menu='isOpened=!isOpened'
        :class="'md:hidden'"
        />
    </nav>
    <div 
      id="burger-menu"
      class="bg-vanilla-base w-screen md:hidden max-md:h-28 max-sm:h-32 max-[440px]:h-44" 
      :class="isOpened? 'burger-menu-show' : 'burger-menu-hide'"
      >

      <router-link
        to="/profile" 
        class="text-3xl text-smoky  flex justify-center items-center text-center md:hidden"
        :class="isOpened? 'block' : 'hidden'"
        >
        {{ $t('header.profile') }}
      </router-link>

      <router-link 
        to="/subnet-calculator"
        class="text-3xl text-smoky  flex justify-center items-center text-center md:hidden"
        :class="isOpened? 'block' : 'hidden'"
        >
        {{ $t('header.subnetCalculator') }}
      </router-link>

      <router-link
        to="/network-visualizer"
        class="text-3xl text-smoky  flex justify-center items-center text-center md:hidden"
        :class="isOpened? 'block' : 'hidden'"
        >
        {{ $t('header.networkVisualizer') }}
      </router-link>

    </div>

  </header>
</template>

<style scoped>

.burger-menu-show {
    animation: fadeInAnimation ease 0.3s;
    animation-iteration-count: 1;
    animation-fill-mode: forwards;
}
 
@keyframes fadeInAnimation {
    0% {
        opacity: 0;
    }
    100% {
        opacity: 1;
     }
}


.burger-menu-hide {
    animation: fadeOutAnimation ease 0.3s;
    animation-iteration-count: 1;
    animation-fill-mode: forwards;
    animation-direction: reverse;
}
 
@keyframes fadeOutAnimation {
    100% {
        opacity: 1;
     }
    0% {
      opacity: 0;
    }
}
</style>
