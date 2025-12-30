import "server-only";

import { getSession } from "./session";

const API_BASE = process.env.NEXT_PUBLIC_API_URL || "http://localhost:8080";

export class ApiError extends Error {
  constructor(message: string, public status: number) {
    super(message);
    this.name = "ApiError";
  }
}

interface RequestOptions {
  method?: "GET" | "POST" | "DELETE";
  body?: unknown;
  auth?: boolean;
}

/** API呼び出し共通関数 */
export async function api<T>(
  endpoint: string,
  options: RequestOptions = {}
): Promise<T> {
  const { method = "GET", body, auth = false } = options;

  const headers: HeadersInit = {
    "Content-Type": "application/json",
  };

  if (auth) {
    const session = await getSession();
    if (!session) {
      throw new ApiError("ログインが必要です", 401);
    }
    headers["Authorization"] = `Bearer ${session.token}`;
  }

  const init: RequestInit = { method, headers };
  if (body) {
    init.body = JSON.stringify(body);
  }

  const response = await fetch(`${API_BASE}${endpoint}`, init);

  if (!response.ok) {
    const error = await response.json();
    throw new ApiError(
      error.error || `HTTP ${response.status}`,
      response.status
    );
  }

  if (response.status === 204) {
    return {} as T;
  }

  return response.json();
}

/** 認証なしAPI */
export const publicApi = <T>(
  endpoint: string,
  options?: Omit<RequestOptions, "auth">
) => api<T>(endpoint, { ...options, auth: false });

/** 認証ありAPI */
export const authApi = <T>(
  endpoint: string,
  options?: Omit<RequestOptions, "auth">
) => api<T>(endpoint, { ...options, auth: true });
