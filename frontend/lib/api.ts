import { clearAuth, getToken, saveAuth } from "./storage";
import type {
  AuthResponse,
  CreateTweetRequest,
  LoginRequest,
  RegisterRequest,
  Tweet,
} from "./types";

const API_BASE = process.env.NEXT_PUBLIC_API_URL || "http://localhost:8080";

async function request<T>(
  endpoint: string,
  options: RequestInit = {}
): Promise<T> {
  const token = getToken();

  const response = await fetch(`${API_BASE}${endpoint}`, {
    ...options,
    headers: {
      "Content-Type": "application/json",
      ...(token && { Authorization: `Bearer ${token}` }),
      ...options.headers,
    },
  });

  if (!response.ok) {
    const body = await response.json().catch(() => ({}));
    throw new Error(body.error || `HTTP ${response.status}`);
  }

  if (response.status === 204) return {} as T;

  return response.json();
}

// 認証 API
export async function register(data: RegisterRequest): Promise<AuthResponse> {
  const res = await request<AuthResponse>("/api/register", {
    method: "POST",
    body: JSON.stringify(data),
  });
  saveAuth(res.user, res.token);
  return res;
}

export async function login(data: LoginRequest): Promise<AuthResponse> {
  const res = await request<AuthResponse>("/api/login", {
    method: "POST",
    body: JSON.stringify(data),
  });
  saveAuth(res.user, res.token);
  return res;
}

export async function logout(): Promise<void> {
  await request("/api/logout", { method: "POST" });
  clearAuth();
}

// ツイート API
export async function createTweet(data: CreateTweetRequest): Promise<Tweet> {
  return request<Tweet>("/api/tweets", {
    method: "POST",
    body: JSON.stringify(data),
  });
}

export async function getTweet(id: string): Promise<Tweet> {
  return request<Tweet>(`/api/tweets/${id}`);
}

export async function deleteTweet(id: string): Promise<void> {
  await request(`/api/tweets/${id}`, { method: "DELETE" });
}

export async function getTimeline(): Promise<Tweet[]> {
  return request<Tweet[]>("/api/timeline");
}
