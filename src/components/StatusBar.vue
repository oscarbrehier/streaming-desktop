<script setup lang="ts">
import { ref } from "vue";
import { cn } from "../utils/cn";
import { isVpsActive } from "../services/tauri/vps";
import { isAgentRunning } from "../services/tauri/agent";

const vpsConnected = ref(false);
const agentRunning = ref(false);

async function checkVpsConnection() {
  const res = await isVpsActive();
  console.log(res);
  vpsConnected.value = res;
}

async function checkAgentConnection() {
  agentRunning.value = await isAgentRunning();
}

setInterval(checkVpsConnection, 5000);
setInterval(checkAgentConnection, 5000);
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
            vpsConnected
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
            agentRunning
              ? 'ring-green-200 bg-green-500 animate-pulse'
              : 'ring-red-200 bg-red-500 animate-pulse'
          )
        "
      />
      <p class="text-xs">Library Connection</p>
    </div>
  </div>
</template>
