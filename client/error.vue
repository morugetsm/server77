<script setup lang="ts">
const props = defineProps({
  error: Object
});

const handleError = () => clearError({ redirect: '/' })
</script>

<template>
  <div>
    404
    <button @click="handleError">home</button>
  </div>
</template>