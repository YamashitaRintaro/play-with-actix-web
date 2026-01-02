import { getCurrentUser } from "@/lib/session";
import { Timeline } from "./_components/Timeline";

export default async function HomePage() {
  const user = await getCurrentUser();

  return <Timeline user={user!} />;
}
