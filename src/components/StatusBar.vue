<script setup lang="ts">
import { cn } from "../utils/cn";
import { useConnectionStore } from "../stores/connection";

const connection = useConnectionStore();

</script>

<template>
  <div
    class="fixed bottom-0 left-0 w-full bg-neutral-100 py-2 px-4 flex items-center space-x-10"
  >
    <div class="flex items-center space-x-3">
      <div
        :class="
          cn(
            'size-2 rounded-full ring-2 ring-offset-1 ring-offset-neutral-50 dark:ring-offset-neutral-600',
            connection.vpsConnected
              ? 'ring-green-200 bg-green-500 animate-pulse'
              : 'ring-red-200 bg-red-500 animate-pulse'
          )
        "
      />
      <p class="text-xs">Secure Access</p>
    </div>

    <div class="flex items-center space-x-3">
      <div
        :class="
          cn(
            'size-2 rounded-full ring-2 ring-offset-1 ring-offset-neutral-50 dark:ring-offset-neutral-600',
            connection.backendRunning
              ? 'ring-green-200 bg-green-500 animate-pulse'
              : 'ring-red-200 bg-red-500 animate-pulse'
          )
        "
      />
      <p class="text-xs">Library Connection</p>
    </div>
  </div>
</template>
