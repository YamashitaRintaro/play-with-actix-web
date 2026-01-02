import { redirect } from "next/navigation";
import { getCurrentUser } from "@/lib/dal";
import { Timeline } from "./_components/Timeline";

export default async function HomePage() {
  const user = await getCurrentUser();

  if (!user) {
    redirect("/login");
  }

  return <Timeline userId={user.id} />;
}
