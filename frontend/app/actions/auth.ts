"use server";

import { createClient } from "@/lib/graphql/client";
import {
  LoginDocument,
  RegisterDocument,
  type LoginMutation,
  type LoginMutationVariables,
  type RegisterMutation,
  type RegisterMutationVariables,
} from "@/lib/graphql/generated/graphql";
import { createSession, deleteSession } from "@/lib/session";
import { redirect } from "next/navigation";

export interface AuthState {
  error?: string;
}

// 型定義
interface RegisterInput {
  username: string;
  email: string;
  password: string;
}

interface LoginInput {
  email: string;
  password: string;
}

// 内部関数（型付き引数、テスト可能）
async function registerUser(
  input: RegisterInput
): Promise<AuthState | undefined> {
  const client = createClient();

  const result = await client
    .mutation<RegisterMutation, RegisterMutationVariables>(RegisterDocument, {
      input,
    })
    .toPromise();

  if (result.error) {
    return { error: result.error.message };
  }

  if (!result.data) {
    return { error: "登録に失敗しました" };
  }

  const { token, user } = result.data.register;
  await createSession(
    { id: user.id, username: user.username, email: user.email },
    token
  );

  return undefined;
}

async function loginUser(input: LoginInput): Promise<AuthState | undefined> {
  const client = createClient();

  const result = await client
    .mutation<LoginMutation, LoginMutationVariables>(LoginDocument, { input })
    .toPromise();

  if (result.error) {
    return { error: result.error.message };
  }

  if (!result.data) {
    return { error: "ログインに失敗しました" };
  }

  const { token, user } = result.data.login;
  await createSession(
    { id: user.id, username: user.username, email: user.email },
    token
  );

  return undefined;
}

/** ユーザー登録 */
export async function register(
  _prevState: AuthState | undefined,
  formData: FormData
): Promise<AuthState | undefined> {
  const username = formData.get("username")?.toString().trim();
  const email = formData.get("email")?.toString().trim();
  const password = formData.get("password")?.toString();

  if (!username || !email || !password) {
    return { error: "すべての項目を入力してください" };
  }

  const result = await registerUser({ username, email, password });

  if (result?.error) {
    return result;
  }

  redirect("/");
}

/** ログイン */
export async function login(
  _prevState: AuthState | undefined,
  formData: FormData
): Promise<AuthState | undefined> {
  const email = formData.get("email")?.toString().trim();
  const password = formData.get("password")?.toString();

  if (!email || !password) {
    return { error: "メールアドレスとパスワードを入力してください" };
  }

  const result = await loginUser({ email, password });

  if (result?.error) {
    return result;
  }

  redirect("/");
}

/** ログアウト */
export async function logout() {
  await deleteSession();
  redirect("/login");
}
