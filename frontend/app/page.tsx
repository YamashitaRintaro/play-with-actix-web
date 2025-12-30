import { redirect } from "next/navigation";
import { authApi } from "@/lib/fetcher";
import { getCurrentUser } from "@/lib/session";
import type { Tweet } from "@/lib/types";
import { Timeline } from "./_components/Timeline";

export default async function HomePage() {
  const user = await getCurrentUser();

  if (!user) {
    redirect("/login");
  }

  const tweets: Tweet[] = await authApi<Tweet[]>("/api/timeline");

  return <Timeline user={user} initialTweets={tweets} />;
}
