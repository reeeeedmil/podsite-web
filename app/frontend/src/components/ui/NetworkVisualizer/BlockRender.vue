<script setup>
  import { ref } from 'vue';

  const props = defineProps({
    size: Number,
    basePrefix: Number,
  })
  function rowSpan() {
    let height = ((((props.size-((props.size+1)%2))%(32-props.basePrefix))-1)/2) + ((props.size+1)%2)
    return parseInt(Math.pow(2, -1*(height))*1000)
  }
  function calculateSize() {
    return `grid-row:span ${rowSpan()}; grid-column:span ${getWidth()}`
  }
  function getWidth() {
    let width = (((props.size-((props.size+1)%2) )%(32-props.basePrefix+1))+1)/2
    return parseInt((32-props.basePrefix)*Math.pow(2, -1*(width))*100)
    }
</script>

<template>

  <div
    class="bg-rufous w-full overflow-hidden text-nyanza-darker border-ash border"
    :style="calculateSize()"
    >{{ props.basePrefix+props.size }}
  </div>


</template>
