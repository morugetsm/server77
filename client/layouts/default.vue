<script setup lang="ts">
</script>

<template>
  <div id="app-container">
    <header>
      <h3>77번에 오신 것을 환영합니다</h3>
      <ul>
        <li>
          <a href="/api/houu">/houu</a>
        </li>
        <li>
          <a href="/api/json">/json</a>
        </li>
        <li>
          <a href="/api/download/atoz.txt">/download/atoz.txt</a>
        </li>
      </ul>
    </header>
    <main>
      <slot />
    </main>
  </div>
</template>

<style scoped lang="scss">
header {
  margin: 0 30vw;
  padding: 1rem 0;

  display: flex;
  flex-flow: column nowrap;
  align-items: flex-start;
  gap: 0.5rem;

  >ul {
    margin: 0;
    display: flex;
    flex-flow: column nowrap;
    align-items: flex-start;
    gap: 0.25rem;

    >li {
      padding: 0.25rem 0.5rem;
      border-radius: 0.25rem;
      background-color: cyan;
    }
  }
}

main {
  margin: 0 30vw;
}
</style>