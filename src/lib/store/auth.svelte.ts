import type { AuthUser } from "$lib/types/auth";

const TOKEN_KEY = "auth_token";
const USER_KEY = "auth_user";

function createAuthStore() {
  let token = $state<string | null>(null);
  let user = $state<AuthUser | null>(null);

  return {
    get token() {
      return token;
    },
    get user() {
      return user;
    },
    get isAuthenticated() {
      return token !== null && user !== null;
    },

    // Call once in root +layout.svelte onMount
    init() {
      const storedToken = localStorage.getItem(TOKEN_KEY);
      const storedUser = localStorage.getItem(USER_KEY);
      if (storedToken && storedUser) {
        token = storedToken;
        user = JSON.parse(storedUser) as AuthUser;
      }
    },

    setAuth(newToken: string, newUser: AuthUser) {
      token = newToken;
      user = newUser;
      localStorage.setItem(TOKEN_KEY, newToken);
      localStorage.setItem(USER_KEY, JSON.stringify(newUser));
    },

    clearAuth() {
      token = null;
      user = null;
      localStorage.removeItem(TOKEN_KEY);
      localStorage.removeItem(USER_KEY);
    },
  };
}

export const authStore = createAuthStore();
