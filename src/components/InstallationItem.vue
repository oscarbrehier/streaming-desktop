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
    class="px-4 py-2 bg-neutral-50 dark:bg-neutral-600 rounded-lg flex items-center space-x-4"
  >
    <div
      :class="
        cn(
          'size-4 rounded-full ring-2 ring-offset-1 ring-offset-neutral-50 dark:ring-offset-neutral-600',
          status === 'pending'
            ? 'ring-black bg-neutral-100'
            : status === 'running'
            ? 'ring-blue-200 bg-blue-500 animate-pulse'
            : 'ring-green-200 bg-green-500'
        )
      "
    ></div>

    <div>
      <p>{{ title }}</p>
      <p class="text-sm text-neutral-600">{{ message }}</p>
    </div>
  </div>
</template>
