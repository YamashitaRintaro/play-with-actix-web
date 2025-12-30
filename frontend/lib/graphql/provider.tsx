"use client";

import { useMemo } from "react";
import { Provider, cacheExchange, createClient, fetchExchange } from "urql";

const API_URL = process.env.NEXT_PUBLIC_API_URL || "http://localhost:8080";

interface GraphQLProviderProps {
  children: React.ReactNode;
  token?: string | null;
}

export function GraphQLProvider({ children, token }: GraphQLProviderProps) {
  const client = useMemo(() => {
    const headers: Record<string, string> = {
      "Content-Type": "application/json",
    };

    if (token) {
      headers["Authorization"] = `Bearer ${token}`;
    }

    return createClient({
      url: `${API_URL}/graphql`,
      exchanges: [cacheExchange, fetchExchange],
      fetchOptions: { headers },
    });
  }, [token]);

  return <Provider value={client}>{children}</Provider>;
}
