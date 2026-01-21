<script setup lang="ts">
import { useConnectionStore } from "../stores/connection";
import StatusIndicator from "./StatusIndicator.vue";

const connection = useConnectionStore();

</script>

<template>
  <div
    class="fixed bottom-0 left-0 w-full bg-neutral-100 dark:bg-neutral-800 py-2 px-4 flex items-center space-x-10"
  >
    <div class="flex items-center space-x-3">
      <StatusIndicator :is-active="connection.vpsConnected" />
      <p class="text-xs">Secure Access</p>
    </div>

    <div class="flex items-center space-x-3">
      <StatusIndicator :is-active="connection.backendRunning" />
      <p class="text-xs">Library Connection</p>
    </div>
  </div>
</template>
