<template>
  <main>
    <h1>quantum-lab</h1>
    <p v-if="sim.ready">
      simulator core loaded — qsv-wasm v{{ sim.wasmVersion }}
    </p>
    <p v-else-if="sim.error">simulator failed to load: {{ sim.error }}</p>
    <p v-else>loading simulator…</p>
  </main>
</template>

<script setup lang="ts">
import { sim } from "@/modules/sim";
</script>

<style scoped>
main {
  max-width: 40rem;
  margin: 4rem auto;
  padding: 0 1rem;
}
</style>
