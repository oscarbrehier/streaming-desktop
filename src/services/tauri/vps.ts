import { invoke } from "@tauri-apps/api/core";

export async function isVpsActive(): Promise<boolean> {

	try {

		const res = await invoke<boolean>("get_vps_status");
		return res ?? false;

	} catch (err) {
		return false;
	};

};