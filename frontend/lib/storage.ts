import type { User } from "./types";

const TOKEN_KEY = "token";
const USER_KEY = "user";

const isBrowser = typeof window !== "undefined";

export function getToken(): string | null {
  return isBrowser ? localStorage.getItem(TOKEN_KEY) : null;
}

export function saveAuth(user: User, token: string): void {
  if (!isBrowser) return;
  localStorage.setItem(TOKEN_KEY, token);
  localStorage.setItem(USER_KEY, JSON.stringify(user));
}

export function clearAuth(): void {
  if (!isBrowser) return;
  localStorage.removeItem(TOKEN_KEY);
  localStorage.removeItem(USER_KEY);
}

export function getStoredUser(): User | null {
  if (!isBrowser) return null;

  const token = localStorage.getItem(TOKEN_KEY);
  const json = localStorage.getItem(USER_KEY);

  if (!token || !json) return null;

  try {
    return JSON.parse(json);
  } catch {
    clearAuth();
    return null;
  }
}
