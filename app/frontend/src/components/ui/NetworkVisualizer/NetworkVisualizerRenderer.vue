<script setup>
  import BlockRender from './BlockRender.vue';
  const props = defineProps({
    inputBasePrefix: Number,
    inputPrefixes: Array,
  })
  if (props.inputBasePrefix == NaN) {props.inputBasePrefix = 0}
  function calculateSize(input) {return input-props.inputBasePrefix}
  function calculateRows() {
    return `grid-template-columns:repeat(${(32-props.inputBasePrefix)*100}, minmax(0, 1fr)); grid-auto-rows: 0px`
  }
</script>

<template>

  <section
    class="bg-vanilla-base"
    >
    <div
      class="min-w-full h-96 overflow-hidden grid grid-rows-[repeat(1000,_minmax(0,_1fr))]"
      :style="calculateRows()"
      >
      <template
        v-for="prefix in props.inputPrefixes"
        >
      <BlockRender
        :size="calculateSize(prefix)"
        :base-prefix="props.inputBasePrefix"
        />
      </template>
    </div>
  </section>

</template>
