import { redirect } from "next/navigation";
import { getApiToken, getCurrentUser } from "@/lib/session";
import { GraphQLProvider } from "@/lib/graphql/provider";
import { Timeline } from "./_components/Timeline";

export default async function HomePage() {
  const user = await getCurrentUser();
  const token = await getApiToken();

  if (!user) {
    redirect("/login");
  }

  return (
    <GraphQLProvider token={token}>
      <Timeline user={user} />
    </GraphQLProvider>
  );
}
