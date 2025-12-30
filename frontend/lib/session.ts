import "server-only";

import { SignJWT, jwtVerify } from "jose";
import { cookies } from "next/headers";
import type { User } from "./types";

const SECRET_KEY =
  process.env.SESSION_SECRET || "your-secret-key-min-32-chars!!";
const ENCODED_KEY = new TextEncoder().encode(SECRET_KEY);
const COOKIE_NAME = "session";

interface SessionPayload {
  user: User;
  token: string;
  expiresAt: Date;
  [key: string]: unknown; // JWTPayload互換のためのインデックスシグネチャ
}

/** セッションを暗号化 */
export async function encrypt(payload: SessionPayload): Promise<string> {
  return new SignJWT(payload)
    .setProtectedHeader({ alg: "HS256" })
    .setIssuedAt()
    .setExpirationTime("1d")
    .sign(ENCODED_KEY);
}

/** セッションを復号 */
export async function decrypt(session: string): Promise<SessionPayload | null> {
  try {
    const { payload } = await jwtVerify<SessionPayload>(session, ENCODED_KEY, {
      algorithms: ["HS256"],
    });
    return payload;
  } catch {
    return null;
  }
}

/** セッションを作成してCookieに保存 */
export async function createSession(user: User, token: string): Promise<void> {
  const expiresAt = new Date(Date.now() + 24 * 60 * 60 * 1000);
  const session = await encrypt({ user, token, expiresAt });

  const cookieStore = await cookies();
  cookieStore.set(COOKIE_NAME, session, {
    httpOnly: true,
    secure: process.env.NODE_ENV === "production",
    expires: expiresAt,
    sameSite: "lax",
    path: "/",
  });
}

/** セッションを削除 */
export async function deleteSession(): Promise<void> {
  const cookieStore = await cookies();
  cookieStore.delete(COOKIE_NAME);
}

/** 現在のセッションを取得 */
export async function getSession(): Promise<SessionPayload | null> {
  const cookieStore = await cookies();
  const session = cookieStore.get(COOKIE_NAME)?.value;
  if (!session) return null;
  return decrypt(session);
}

/** 現在のユーザーを取得 */
export async function getCurrentUser(): Promise<User | null> {
  const session = await getSession();
  return session?.user ?? null;
}

/** APIトークンを取得 */
export async function getApiToken(): Promise<string | null> {
  const session = await getSession();
  return session?.token ?? null;
}
