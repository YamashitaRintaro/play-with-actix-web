/**
 * Data Access Layer (DAL)
 *
 * Server Components 用のデータアクセス関数。
 * React の cache() により、同一リクエスト内での重複呼び出しがメモ化され、
 * JWT復号が1回のみになります。
 *
 * 使用例:
 *   Layout: const session = await getSession();
 *   Page:   const user = await getCurrentUser(); // キャッシュヒット
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
