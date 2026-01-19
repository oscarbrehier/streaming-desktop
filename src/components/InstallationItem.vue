<script setup lang="ts">
import { cn } from "../utils/cn";

defineProps<{
  title: string;
  status?: "pending" | "running" | "success";
  message?: string;
}>();
</script>

<template>
  <div
    class="px-4 py-2 bg-neutral-50 dark:bg-neutral-800 rounded-lg flex items-center space-x-4"
  >
    <div
      :class="
        cn(
          'size-4 rounded-full ring-2 ring-offset-1 ring-offset-neutral-50 dark:ring-offset-neutral-800',
          status === 'pending'
            ? 'ring-black bg-neutral-100'
            : status === 'running'
            ? 'ring-blue-200 dark:ring-blue-800 bg-blue-500 animate-pulse'
            : 'ring-green-200 dark:ring-green-800 bg-green-500'
        )
      "
    />

    <div>
      <p>{{ title }}</p>
      <p class="text-sm text-neutral-600 dark:text-neutral-300">{{ message }}</p>
    </div>
  </div>
</template>
