"use client";

import { register } from "@/app/actions/auth";
import Link from "next/link";
import { useActionState } from "react";

export default function RegisterPage() {
  const [state, action, pending] = useActionState(register, undefined);

  return (
    <main className="min-h-screen flex items-center justify-center py-12 px-4">
      <div className="w-full max-w-md">
        <div className="bg-card rounded-2xl shadow-lg border border-border p-8">
          <h1 className="text-3xl font-bold text-primary text-center mb-8">
            🐦 Twitter Clone
          </h1>

          {state?.error && (
            <div className="mb-6 p-4 bg-red-50 border border-red-200 rounded-xl text-danger text-sm">
              {state.error}
            </div>
          )}

          <form action={action} className="space-y-5">
            <div>
              <input
                type="text"
                name="username"
                placeholder="ユーザー名"
                required
                className="w-full px-4 py-3 border border-border rounded-xl focus:outline-none focus:ring-2 focus:ring-primary/30 focus:border-primary transition-all"
              />
            </div>
            <div>
              <input
                type="email"
                name="email"
                placeholder="メールアドレス"
                required
                className="w-full px-4 py-3 border border-border rounded-xl focus:outline-none focus:ring-2 focus:ring-primary/30 focus:border-primary transition-all"
              />
            </div>
            <div>
              <input
                type="password"
                name="password"
                placeholder="パスワード"
                required
                className="w-full px-4 py-3 border border-border rounded-xl focus:outline-none focus:ring-2 focus:ring-primary/30 focus:border-primary transition-all"
              />
            </div>
            <button
              type="submit"
              disabled={pending}
              className="w-full py-3 bg-primary text-white rounded-full font-semibold hover:bg-primary-hover disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
            >
              {pending ? "登録中..." : "新規登録"}
            </button>
          </form>

          <p className="mt-6 text-center text-muted">
            すでにアカウントをお持ちの方は{" "}
            <Link
              href="/login"
              className="text-primary hover:underline font-medium"
            >
              ログイン
            </Link>
          </p>
        </div>
      </div>
    </main>
  );
}

