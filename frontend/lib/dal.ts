/**
 * Data Access Layer (DAL)
 * Server Components 用のデータアクセス関数。
 */
import "server-only";

import { cache } from "react";
import { verifySession } from "./session";
import type { SessionUser } from "./types";

/** セッション全体を取得 */
export const getSession = cache(async () => {
  return verifySession();
});

/** ユーザー情報のみ取得 */
export const getCurrentUser = cache(async (): Promise<SessionUser | null> => {
  const session = await getSession();
  return session?.user ?? null;
});
