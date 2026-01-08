<script setup lang="ts">
import { computed } from "vue";
import { cn } from "../utils/cn";

type Variant = "primary" | "danger" | "ghost";
type Size = "sm" | "md" | "lg" | "icon";

const props = withDefaults(
  defineProps<{
    variant?: Variant;
    size?: Size;
    loading?: boolean;
    disabled?: boolean;
    class?: string;
    type?: "button" | "submit" | "reset";
  }>(),
  {
    variant: "primary",
    size: "md",
    loading: false,
    disabled: false,
    type: "button",
  }
);

const baseClasses =
  "inline-flex items-center justify-center rounded-lg transition-colors disabled:pointer-events-none";

const variantClasses: Record<Variant, string> = {
  primary: "bg-neutral-100 hover:bg-neutral-200 disabled:text-neutral-300",
  danger: "bg-neutral-100 text-red-600 disabled:text-red-200",
  ghost: "bg-transparent text-neutral-700 hover:bg-neutral-100",
};

const sizeClasses: Record<Size, string> = {
  sm: "h-8 px-3 text-sm",
  md: "h-10 px-4 text-base",
  lg: "h-12 px-6 text-base",
  icon: "size-10 aspect-square",
};

const classes = computed(() =>
  cn(baseClasses, variantClasses[props.variant], sizeClasses[props.size], props.class)
);
</script>

<template>
  <button :type="type" :disabled="disabled || loading" :class="classes">
    <span
      v-if="loading"
      class="h-4 w-4 animate-spin rounded-lg border-2 border-white border-t-transparent text-[1em] font-medium"
    />
    <slot />
  </button>
</template>
