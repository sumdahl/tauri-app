//we will extend this more, this is just sample api.ts.

//based on backend models type, we create a seperate folder with types
//and there will be type

import { invoke } from "@tauri-apps/api/core";

export interface GreetResponse {
  message: string;
  greeting_count: number;
}

export interface AppInfo {
  name: string;
  version: string;
  greeting_count: number;
  initialized: boolean;
}

export async function greet(name: string): Promise<GreetResponse> {
  return invoke<GreetResponse>("greet", { name });
}

export async function getAppInfo(): Promise<AppInfo> {
  return invoke<AppInfo>("get_app_info");
}

export async function initializeApp(): Promise<AppInfo> {
  return invoke<AppInfo>("initialize_app");
}

export async function resetGreetingCount(): Promise<AppInfo> {
  return invoke<AppInfo>("reset_greeting_count");
}
