import { redirect } from "next/navigation";
import { getSession } from "@/lib/dal";
import { GraphQLProvider } from "@/lib/graphql/provider";
import { logout } from "@/app/actions/auth";

export default async function PrivateLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  const session = await getSession();

  if (!session) {
    redirect("/login");
  }

  return (
    <GraphQLProvider token={session.token}>
      <div className="min-h-screen">
        <header className="sticky top-0 z-10 bg-white/80 backdrop-blur-sm border-b border-border">
          <div className="max-w-2xl mx-auto px-4 py-4 flex items-center justify-between">
            <h1 className="text-2xl font-bold text-primary">
              🐦 Twitter Clone
            </h1>
            <div className="flex items-center gap-4">
              <span className="text-muted">@{session.user.username}</span>
              <form action={logout}>
                <button
                  type="submit"
                  className="px-4 py-2 bg-danger text-white rounded-full font-medium hover:bg-danger-hover transition-colors"
                >
                  ログアウト
                </button>
              </form>
            </div>
          </div>
        </header>
        <main>{children}</main>
      </div>
    </GraphQLProvider>
  );
}
