import {invoke} from "@tauri-apps/api";

export async function initSystemTray() {
   const result = await invoke('init_sys_tray')
   console.log(result)
   return result
}