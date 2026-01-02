import { getCurrentUser } from "@/lib/session";
import { Timeline } from "./_components/Timeline";
import { redirect } from "next/navigation";

export default async function HomePage() {
  const user = await getCurrentUser();

  if (!user) {
    redirect("/login");
  }

  return <Timeline userId={user.id} />;
}
