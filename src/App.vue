<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import InstallationItem from "./components/InstallationItem.vue";
import { onMounted, onUnmounted, ref } from "vue";
import { listen } from "@tauri-apps/api/event";

type StepStatus = "pending" | "running" | "success";

type Step = {
  id: string;
  title: string;
  status: StepStatus;
  message: string;
};

let unlisten: (() => void) | null = null;

onMounted(async () => {
  unlisten = await listen<Step>("setup-progress", (event) => {
    const payload = event.payload;

    const existing = steps.value.find((step) => step.id === payload.id);

    if (existing) {
      Object.assign(existing, payload);
    } else {
      steps.value.push({
        ...payload,
        status: payload.status ?? "pending",
      });
    }
  });
});

onUnmounted(() => {
  if (unlisten) {
    unlisten();
  }
});

const steps = ref<Step[]>([]);

async function startConnectionSetup() {
  await invoke("run_setup");
}
</script>

<template>
  <div class="w-full max-w-md">
    <div class="mb-10">
      <p class="text-2xl">Preparing your library</p>
      <p class="text-sm text-neutral-400">
        We’re getting everything ready so you can start streaming. This ensures your
        connection is secure and all necessary services are running.
      </p>
    </div>

    <button class="my-4 w-full" @click="startConnectionSetup">Connect & Stream</button>

    <div class="space-y-2">
      <InstallationItem
        v-for="step in steps"
        :key="step.id"
        :title="step.title"
        :message="step.message"
        :status="step.status"
      />
    </div>
  </div>
</template>
