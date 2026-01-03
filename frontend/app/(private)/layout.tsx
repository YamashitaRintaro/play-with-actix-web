import { redirect } from "next/navigation";
import Link from "next/link";
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
            <Link
              href="/"
              className="text-2xl font-bold text-primary hover:opacity-80 transition-opacity"
            >
              🐦 Twitter Clone
            </Link>
            <div className="flex items-center gap-4">
              <Link
                href={`/profile/${session.user.id}`}
                className="text-muted hover:text-primary transition-colors"
              >
                @{session.user.username}
              </Link>
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
