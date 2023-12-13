<script setup>
</script>

<template>
  <div class="w-full text-center">
    <ul  class="list-none w-full text-center">
      <span
        class="text-xl hook-1 pl-3 text-smoky"
        >s
      </span>
      <span
        class="text-xl hook-2 pl-3 text-smoky"
        >u
      </span>
      <span
        class="text-xl hook-3 pl-3 text-smoky"
        >b
      </span>
      <span
        class="text-xl hook-4 pl-3 text-smoky"
        >N
      </span>
      <span
        class="text-xl hook-5 pl-3 text-smoky"
        >e
      </span>
      <span
        class="text-xl hook-6 pl-3 text-smoky"
        >t
      </span>
    </ul>
  </div>
</template>

<style scoped>
.hook-1 {
  animation: hook-1-animation 1s;
  animation-iteration-count: infinite;
  animation-direction: reverse;
}
@keyframes hook-1-animation {
    100% {
      font-family: skyhook;
      font-size: 1.875rem;
    }
    80%, 60%, 40%, 20%, 0% {
      font-family: unibody8;
      font-size: 1.25rem;
    }
}
.hook-2 {
  animation: hook-2-animation 1s;
  animation-iteration-count: infinite;
  animation-direction: reverse;
}
@keyframes hook-2-animation {
    80% {
      font-family: skyhook;
      font-size: 1.875rem;
    }
    100%, 60%, 40%, 20%, 0% {
      font-family: unibody8;
      font-size: 1.25rem;
    }
}
.hook-3 {
  animation: hook-3-animation 1s;
  animation-iteration-count: infinite;
  animation-direction: reverse;
}
@keyframes hook-3-animation {
    60% {
      font-family: skyhook;
      font-size: 1.875rem;
    }
    100%, 80%, 40%, 20%, 0% {
      font-family: unibody8;
      font-size: 1.25rem;
    }
}
.hook-4 {
  animation: hook-4-animation 1s;
  animation-iteration-count: infinite;
  animation-direction: reverse;
}
@keyframes hook-4-animation {
    40% {
      font-family: skyhook;
      font-size: 1.875rem;
    }
    100%, 80%, 60%, 20%, 0% {
      font-family: unibody8;
      font-size: 1.25rem;
    }
}

.hook-5 {
  animation: hook-5-animation 1s;
  animation-iteration-count: infinite;
  animation-direction: reverse;
}
@keyframes hook-5-animation {
    20% {
      font-family: skyhook;
      font-size: 1.875rem;
    }
    100%, 80%, 60%, 40%, 0% {
      font-family: unibody8;
      font-size: 1.25rem;
    }
}

.hook-6 {
  animation: hook-6-animation 1s;
  animation-iteration-count: infinite;
  animation-direction: reverse;
}
@keyframes hook-6-animation {
    0% {
      font-family: skyhook;
      font-size: 1.875rem;
    }
    100%, 80%, 60%, 40%, 20% {
      font-family: unibody8;
      font-size: 1.25rem;
    }
}
</style>
