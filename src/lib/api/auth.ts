import { invoke } from "@tauri-apps/api/core";
import type {
  SigninRequest,
  SigninResponse,
  SignupRequest,
  SignupResponse,
} from "$lib/types/auth";

export async function signupInvoke(
  request: SignupRequest,
): Promise<SignupResponse> {
  return invoke<SignupResponse>("signup", { request });
}

export async function signinInvoke(
  request: SigninRequest,
): Promise<SigninResponse> {
  return invoke<SigninResponse>("signin", { request });
}
