<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import InstallationItem from "./components/InstallationItem.vue";
import { onMounted, onUnmounted, ref } from "vue";
import { listen } from "@tauri-apps/api/event";
import StatusBar from "./components/StatusBar.vue";
import { useConnectionStore } from "./stores/connection";
import { RefreshCcw } from "lucide-vue-next";
import Button from "./components/Button.vue";

type StepStatus = "pending" | "running" | "success";

type Step = {
  id: string;
  title: string;
  status: StepStatus;
  message: string;
};

const connection = useConnectionStore();

onMounted(() => {
  connection.refreshStatus();
  setInterval(connection.refreshStatus, 5000);
});

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
  await invoke("bootstrap_library");
}

async function terminateConnection() {

  try {
    const success = await invoke("terminate_agent");
    console.log(success);
  } catch (err) {
    console.error(err);
  }

}
</script>

<template>
  <div class="w-full max-w-md">
    <div class="mb-10">
      <p class="text-2xl mb-1">Getting ready to stream</p>
      <p class="text-sm text-neutral-500 dark:text-neutral-300">
        We’re checking your connection and starting the services needed to stream your
        library.
      </p>
    </div>

    <div class="w-full flex items-center space-x-2 my-4">
      <Button
      
        @click="connection.backendRunning ? terminateConnection() : startConnectionSetup()"
        :variant="connection.backendRunning ? 'danger' : 'primary'"
        class="w-full"
      >
        {{ connection.backendRunning ? "Stop Streaming" : "Connect & Stream" }}
      </Button>
      <Button size="icon" @click="startConnectionSetup">
        <RefreshCcw :size="16" :stroke-width="2.5" />
      </Button>
      <Button size="icon" @click="terminateConnection">
        <RefreshCcw :size="16" :stroke-width="2.5" />
      </Button>
    </div>

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

  <StatusBar />
</template>
