import { defineStore } from "pinia";
import { ref } from "vue";
import { isVpsActive } from "../services/tauri/vps";
import { isAgentRunning } from "../services/tauri/agent";

export const useConnectionStore = defineStore("connection", () => {

	const vpsConnected = ref(false);
	const backendRunning = ref(false);
	const checking = ref(false);

	async function refreshStatus() {

		checking.value = true;

		try {
			vpsConnected.value = await isVpsActive();
			backendRunning.value = await isAgentRunning();
		} finally {
			checking.value = false;
		};

	};

	return {
		vpsConnected,
		backendRunning,
		checking,
		refreshStatus
	}

});