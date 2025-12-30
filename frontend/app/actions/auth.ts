"use server";

import { publicApi, ApiError } from "@/lib/fetcher";
import { createSession, deleteSession } from "@/lib/session";
import type { AuthResponse } from "@/lib/types";
import { redirect } from "next/navigation";

export interface AuthState {
  error?: string;
}

/** ユーザー登録 */
export async function register(
  _prevState: AuthState | undefined,
  formData: FormData
): Promise<AuthState | undefined> {
  try {
    const data = await publicApi<AuthResponse>("/api/register", {
      method: "POST",
      body: {
        username: formData.get("username"),
        email: formData.get("email"),
        password: formData.get("password"),
      },
    });

    await createSession(data.user, data.token);
  } catch (e) {
    if (e instanceof ApiError) {
      return { error: e.message };
    }
    return { error: "登録に失敗しました" };
  }

  redirect("/");
}

/** ログイン */
export async function login(
  _prevState: AuthState | undefined,
  formData: FormData
): Promise<AuthState | undefined> {
  try {
    const data = await publicApi<AuthResponse>("/api/login", {
      method: "POST",
      body: {
        email: formData.get("email"),
        password: formData.get("password"),
      },
    });

    await createSession(data.user, data.token);
  } catch (e) {
    if (e instanceof ApiError) {
      return { error: e.message };
    }
    return { error: "ログインに失敗しました" };
  }

  redirect("/");
}

/** ログアウト */
export async function logout() {
  await deleteSession();
  redirect("/login");
}
