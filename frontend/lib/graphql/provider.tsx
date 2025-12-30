"use client";

import { UrqlProvider } from "@urql/next";
import { cacheExchange, createClient, fetchExchange, ssrExchange } from "urql";
import { useMemo } from "react";

const API_URL = process.env.NEXT_PUBLIC_API_URL || "http://localhost:8080";

interface GraphQLProviderProps {
  children: React.ReactNode;
  token?: string | null;
}

export function GraphQLProvider({ children, token }: GraphQLProviderProps) {
  const [client, ssr] = useMemo(() => {
    const ssr = ssrExchange({
      isClient: typeof window !== "undefined",
    });

    const headers: Record<string, string> = {
      "Content-Type": "application/json",
    };

    if (token) {
      headers["Authorization"] = `Bearer ${token}`;
    }

    const client = createClient({
      url: `${API_URL}/graphql`,
      exchanges: [cacheExchange, ssr, fetchExchange],
      fetchOptions: { headers },
      suspense: true,
    });

    return [client, ssr];
  }, [token]);

  return (
    <UrqlProvider client={client} ssr={ssr}>
      {children}
    </UrqlProvider>
  );
}
