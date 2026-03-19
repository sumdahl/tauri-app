import { invoke } from "@tauri-apps/api/core";
import type {
  SigninRequest,
  SigninResponse,
  SignupRequest,
  SignupResponse,
} from "$lib/types/auth";

const isTauri = () => "__TAURI_INTERNALS__" in window;

export async function signinInvoke(
  request: SigninRequest,
): Promise<SigninResponse> {
  if (!isTauri()) {
    // Mock response for browser development only
    console.warn("[DEV] Tauri not available — using mock signin");
    return {
      message: "Mock sign in successful",
      user_id: "dev-user-id",
      token: "dev-mock-token",
    };
  }
  return invoke<SigninResponse>("signin", {
    email: request.email,
    password: request.password,
  });
}

export async function signupInvoke(
  request: SignupRequest,
): Promise<SignupResponse> {
  if (!isTauri()) {
    // Mock response for browser development only
    console.warn("[DEV] Tauri not available — using mock signup");
    return {
      message: "Mock sign up successful",
      user_id: "dev-user-id",
      token: "dev-mock-token",
    };
  }
  return invoke<SignupResponse>("signup", {
    fullname: request.fullname,
    email: request.email,
    password: request.password,
  });
}
