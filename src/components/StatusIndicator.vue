<script setup lang="ts">
import { cn } from "../utils/cn";

defineProps<{
  isActive: boolean;
}>();
</script>

<template>
	<div
		:class="
		cn(
			'size-2 rounded-full ring-2 ring-offset-1 ring-offset-neutral-50 dark:ring-offset-neutral-800',
			isActive
			? 'ring-green-200 dark:ring-green-800 bg-green-500 animate-pulse'
			: 'ring-red-200 dark:ring-red-800 bg-red-500 animate-pulse'
		)
		"
	/>
</template>
