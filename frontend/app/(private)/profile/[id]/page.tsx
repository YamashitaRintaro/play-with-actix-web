import { getCurrentUser } from "@/lib/dal";
import { redirect } from "next/navigation";
import { ProfileView } from "./_components/ProfileView";

interface Props {
  params: Promise<{ id: string }>;
}

export default async function ProfilePage({ params }: Props) {
  const { id } = await params;
  const currentUser = await getCurrentUser();

  if (!currentUser) {
    redirect("/login");
  }

  return <ProfileView userId={id} currentUserId={currentUser.id} />;
}

