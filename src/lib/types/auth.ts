// Shared frontend auth payloads and session types.

export interface SignupRequest {
  fullname: string;
  email: string;
  password: string;
}

export interface SignupResponse {
  message: string;
  user_id: string;
  token: string;
}

export interface SigninRequest {
  email: string;
  password: string;
}

export interface SigninResponse {
  message: string;
  user_id: string;
  token: string;
}

export interface AuthUser {
  id: string;
  fullname: string;
  email: string;
}

export interface AuthState {
  user: AuthUser | null;
  token: string | null;
  isAuthenticated: boolean;
}
