import "server-only";

import { SignJWT, jwtVerify } from "jose";
import { cookies } from "next/headers";
import type { SessionUser } from "./types";

const SECRET_KEY =
  process.env.SESSION_SECRET || "your-secret-key-min-32-chars!!";
const ENCODED_KEY = new TextEncoder().encode(SECRET_KEY);
const COOKIE_NAME = "session";
const SESSION_DURATION = 24 * 60 * 60 * 1000; // 24時間

interface SessionPayload {
  user: SessionUser;
  token: string;
  expiresAt: string;
}

/** セッションを暗号化してJWTを生成 */
async function encrypt(payload: SessionPayload): Promise<string> {
  return new SignJWT({ ...payload })
    .setProtectedHeader({ alg: "HS256" })
    .setIssuedAt()
    .setExpirationTime("1d") // 1 day
    .sign(ENCODED_KEY);
}

/** JWTを復号してセッションペイロードを取得 */
async function decrypt(token: string): Promise<SessionPayload | null> {
  try {
    const { payload } = await jwtVerify<SessionPayload>(token, ENCODED_KEY, {
      algorithms: ["HS256"],
    });
    return payload;
  } catch {
    return null;
  }
}

/** セッションを作成してCookieに保存 */
export async function createSession(
  user: SessionUser,
  token: string
): Promise<void> {
  const expiresAt = new Date(Date.now() + SESSION_DURATION);
  const session = await encrypt({
    user,
    token,
    expiresAt: expiresAt.toISOString(),
  });

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

/** セッションを検証して返す */
export async function verifySession(): Promise<SessionPayload | null> {
  const cookieStore = await cookies();
  const sessionCookie = cookieStore.get(COOKIE_NAME)?.value;

  if (!sessionCookie) {
    return null;
  }

  const payload = await decrypt(sessionCookie);

  if (!payload) {
    return null;
  }

  // 有効期限チェック
  const expiresAt = new Date(payload.expiresAt);
  if (expiresAt < new Date()) {
    return null;
  }

  return payload;
}

/** Middleware用: セッションを検証 */
export async function verifySessionFromCookie(
  sessionCookie: string | undefined
): Promise<SessionPayload | null> {
  if (!sessionCookie) {
    return null;
  }

  const payload = await decrypt(sessionCookie);

  if (!payload) {
    return null;
  }

  // 有効期限チェック
  const expiresAt = new Date(payload.expiresAt);
  if (expiresAt < new Date()) {
    return null;
  }

  return payload;
}

/** Cookie名をエクスポート */
export const SESSION_COOKIE_NAME = COOKIE_NAME;
