<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";

async function setupEnvironment() {
  try {
    await invoke("setup_environment");
  } catch (err) {
    console.log("setup failed", err);
  }
}

async function getTailscaleStatus() {
  try {
    const res = await invoke("get_tailscale_status");
    console.log(res);
  } catch (err) {
    console.log("setup failed", err);
  }
}

async function startAgent() {
  try {
    const res = await invoke("start_agent");
    console.log(res);
  } catch (err) {
    console.log("setup failed", err);
  }
}
</script>

<template>
  <div class="w-full max-w-md">
    <div class="mb-10">
      <p class="text-2xl">Installation</p>
      <p class="text-sm text-neutral-400">
        Everything is being setup for you. You will be prompted on an external website to
        login in order to access our private network.
      </p>
    </div>

    <button class="my-4 w-full" @click="setupEnvironment">Start Installation</button>
    <button class="my-4 w-full" @click="getTailscaleStatus">Tailscale Status</button>
    <button class="my-4 w-full" @click="startAgent">Start Agent</button>

    <div class="space-y-2">
      <div
        class="px-4 py-2 bg-neutral-50 dark:bg-neutral-600 rounded-lg flex items-center space-x-4"
      >
        <div
          class="size-4 rounded-full ring-2 ring-black ring-offset-1 ring-offset-neutral-50 dark:ring-offset-neutral-600"
        ></div>
        <p>Downloading VPN</p>
      </div>

      <!-- <div class="px-4 py-2 bg-neutral-50 rounded-lg flex items-center space-x-4">
        <div class="size-4 rounded-full ring-2 ring-black ring-offset-1"></div>
        <p>Initializing VPN</p>
      </div>

      <div class="px-4 py-2 bg-neutral-50 rounded-lg flex items-center space-x-4">
        <div class="size-4 rounded-full ring-2 ring-black ring-offset-1"></div>
        <p>Download service</p>
      </div>

      <div class="px-4 py-2 bg-neutral-50 rounded-lg flex items-center space-x-4">
        <div class="size-4 rounded-full ring-2 ring-black ring-offset-1"></div>
        <p>Starting service</p>
      </div> -->
    </div>
  </div>

  <!-- <div>
		<p
			class="text-3xl text-red-500"
		>
			hello
		</p>
	</div>

 	<div>
		<a href="https://vite.dev" target="_blank">
			<img src="/vite.svg" class="logo" alt="Vite logo" />
		</a>
		<a href="https://vuejs.org/" target="_blank">
			<img src="./assets/vue.svg" class="logo vue" alt="Vue logo" />
		</a>
	</div>
	<HelloWorld msg="Vite + Vue hello" /> -->
</template>

<style scoped>
.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: filter 300ms;
}
.logo:hover {
  filter: drop-shadow(0 0 2em #646cffaa);
}
.logo.vue:hover {
  filter: drop-shadow(0 0 2em #42b883aa);
}
</style>
