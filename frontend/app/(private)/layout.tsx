import { redirect } from "next/navigation";
import { getApiToken, getCurrentUser } from "@/lib/session";
import { GraphQLProvider } from "@/lib/graphql/provider";

export default async function PrivateLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  const user = await getCurrentUser();
  const token = await getApiToken();

  if (!user) {
    redirect("/login");
  }

  return <GraphQLProvider token={token}>{children}</GraphQLProvider>;
}

