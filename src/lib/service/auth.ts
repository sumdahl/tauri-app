import { signinInvoke, signupInvoke } from "$lib/api/auth";
import { authStore } from "$lib/store/auth.svelte";
import type { SigninRequest, SignupRequest } from "$lib/types/auth";
import { goto } from "$app/navigation";

export async function signup(data: SignupRequest): Promise<void> {
  const response = await signupInvoke(data);
  authStore.setAuth(response.token, {
    id: response.user_id,
    fullname: data.fullname,
    email: data.email,
  });
  await goto("/dashboard");
}

export async function signin(data: SigninRequest): Promise<void> {
  const response = await signinInvoke(data);
  // Decode fullname from token or store minimal info
  // For now we store what we have — extend when get_me command is added
  authStore.setAuth(response.token, {
    id: response.user_id,
    fullname: "",
    email: data.email,
  });
  await goto("/dashboard");
}

export async function signout(): Promise<void> {
  authStore.clearAuth();
  await goto("/");
}
