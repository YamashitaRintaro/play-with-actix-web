import { Client, cacheExchange, fetchExchange } from "@urql/core";

const API_URL = process.env.NEXT_PUBLIC_API_URL || "http://localhost:8080";

/**
 * urqlクライアントを作成（Server Components用）
 * 認証トークンを受け取ってヘッダーに設定
 */
export function createClient(token?: string | null) {
  const headers: Record<string, string> = {
    "Content-Type": "application/json",
  };

  if (token) {
    headers["Authorization"] = `Bearer ${token}`;
  }

  return new Client({
    url: `${API_URL}/graphql`,
    exchanges: [cacheExchange, fetchExchange],
    fetchOptions: () => ({ method: "POST", headers }),
  });
}
