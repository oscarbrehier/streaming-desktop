import { invoke } from "@tauri-apps/api/core";

export async function startAgent(): Promise<{ data?: any, error: string | null }> {

	try {

		const res = await invoke("start_agent");
		return { data: res, error: null };

	} catch (err) {

		const message = err instanceof Error ? err.message : "Unknown error";
		return { error: message }

	};

};


export async function isAgentRunning(): Promise<boolean> {

	try {

		const res = await invoke<boolean>("get_backend_status");
		return res ?? false;

	} catch (err) {
		return false;
	};

};