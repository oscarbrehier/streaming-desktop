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
