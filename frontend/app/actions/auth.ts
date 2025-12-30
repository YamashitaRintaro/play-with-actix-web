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

/** ユーザー登録 */
export async function register(
  _prevState: AuthState | undefined,
  formData: FormData
): Promise<AuthState | undefined> {
  const client = createClient();

  const variables: RegisterMutationVariables = {
    input: {
      username: formData.get("username") as string,
      email: formData.get("email") as string,
      password: formData.get("password") as string,
    },
  };

  const result = await client
    .mutation<RegisterMutation, RegisterMutationVariables>(
      RegisterDocument,
      variables
    )
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

  redirect("/");
}

/** ログイン */
export async function login(
  _prevState: AuthState | undefined,
  formData: FormData
): Promise<AuthState | undefined> {
  const client = createClient();

  const variables: LoginMutationVariables = {
    input: {
      email: formData.get("email") as string,
      password: formData.get("password") as string,
    },
  };

  const result = await client
    .mutation<LoginMutation, LoginMutationVariables>(LoginDocument, variables)
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

  redirect("/");
}

/** ログアウト */
export async function logout() {
  await deleteSession();
  redirect("/login");
}
