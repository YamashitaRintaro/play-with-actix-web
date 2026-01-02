import { NextRequest, NextResponse } from "next/server";
import { verifySessionFromCookie, SESSION_COOKIE_NAME } from "@/lib/session";

const PUBLIC_ROUTES = ["/login", "/register"];
const PROTECTED_ROUTES = ["/"];

function isPublicRoute(pathname: string): boolean {
  return PUBLIC_ROUTES.some(
    (route) => pathname === route || pathname.startsWith(`${route}/`)
  );
}

function isProtectedRoute(pathname: string): boolean {
  if (PROTECTED_ROUTES.includes(pathname)) {
    return true;
  }

  return !isPublicRoute(pathname);
}

export async function middleware(request: NextRequest) {
  const { pathname } = request.nextUrl;

  // 静的ファイルとAPIルートはスキップ
  if (
    pathname.startsWith("/_next") ||
    pathname.startsWith("/api") ||
    pathname.includes(".")
  ) {
    return NextResponse.next();
  }

  // セッションCookieを取得して検証
  const sessionCookie = request.cookies.get(SESSION_COOKIE_NAME)?.value;
  const session = await verifySessionFromCookie(sessionCookie);
  const isAuthenticated = session !== null;

  // 保護されたルートへの未認証アクセス → ログインへリダイレクト
  if (isProtectedRoute(pathname) && !isAuthenticated) {
    const loginUrl = new URL("/login", request.url);
    // ログイン後に元のページに戻れるようにリダイレクト先を保存
    if (pathname !== "/") {
      loginUrl.searchParams.set("redirect", pathname);
    }
    return NextResponse.redirect(loginUrl);
  }

  // 公開ルートへの認証済みアクセス → ホームへリダイレクト
  if (isPublicRoute(pathname) && isAuthenticated) {
    return NextResponse.redirect(new URL("/", request.url));
  }

  return NextResponse.next();
}

// Middlewareを適用するパスを設定
export const config = {
  matcher: [
    /*
     * 以下を除くすべてのパスにマッチ:
     * - api (APIルート)
     * - _next/static (静的ファイル)
     * - _next/image (画像最適化)
     * - favicon.ico (ファビコン)
     */
    "/((?!api|_next/static|_next/image|favicon.ico).*)",
  ],
};
