<script setup>
import { ref } from 'vue';
const isOpened = ref(false)
</script>

<template>

  <div class="menu-btn" :class="isOpened? 'open' : ''" @click="isOpened = !isOpened; $emit('changeMenu')">
    <div class="menu-btn__burger"></div>
  </div>

</template>

<style scoped>

.menu-btn {
  padding-right: 2%;
  position: relative;
  display: flex;
  justify-content: center;
  align-items: center;
  width: 30%px;
  height: 100%;
  transition: all .5s ease-in-out;
  /* border: 3px solid #fff; */
}
.menu-btn__burger {
  padding-right: 2%;
  width: 40px;
  height: 6px;
  background: #fff;
  border-radius: 5px;
  box-shadow: 0 2px 5px rgba(255,101,47,.2);
  transition: all .5s ease-in-out;
}
.menu-btn__burger::before,
.menu-btn__burger::after {
  padding-right: 2%;
  content: '';
  position: absolute;
  width: 40px;
  height: 6px;
  background: #fff;
  border-radius: 5px;
  box-shadow: 0 2px 5px rgba(255,101,47,.2);
  transition: all .5s ease-in-out;
}
.menu-btn__burger::before {
  transform: translateY(-16px);
}
.menu-btn__burger::after {
  transform: translateY(16px);
}
/* ANIMATION */
.menu-btn.open .menu-btn__burger {
  transform: translateX(-50px);
  background: transparent;
  box-shadow: none;
}
.menu-btn.open .menu-btn__burger::before {
  transform: rotate(45deg) translate(35px, -35px);
}
.menu-btn.open .menu-btn__burger::after {
  transform: rotate(-45deg) translate(35px, 35px);
}

</style>
