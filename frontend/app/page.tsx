"use client";

import { useAuth } from "@/lib/auth-context";
import { logout } from "@/lib/api";
import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";
import { useTweets } from "./_hooks/useTweets";

export default function HomePage() {
  const { user, logout: authLogout } = useAuth();
  const router = useRouter();
  const { tweets, content, setContent, error, isSubmitting, submit, remove } =
    useTweets(user?.id);
  const [logoutError, setLogoutError] = useState("");

  useEffect(() => {
    if (!user) router.push("/login");
  }, [user, router]);

  const handleLogout = async () => {
    try {
      await logout();
      authLogout();
      router.push("/login");
    } catch {
      setLogoutError("ログアウトに失敗しました");
    }
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    submit();
  };

  if (!user) return null;

  const displayError = error || logoutError;

  return (
    <main className="min-h-screen py-8">
      <div className="max-w-2xl mx-auto px-4">
        {/* Header */}
        <header className="flex items-center justify-between mb-8">
          <h1 className="text-3xl font-bold text-primary">🐦 Twitter Clone</h1>
          <div className="flex items-center gap-4">
            <span className="text-muted">@{user.username}</span>
            <button
              onClick={handleLogout}
              className="px-4 py-2 bg-danger text-white rounded-full font-medium hover:bg-danger-hover transition-colors"
            >
              ログアウト
            </button>
          </div>
        </header>

        {/* Error */}
        {displayError && (
          <div className="mb-6 p-4 bg-red-50 border border-red-200 rounded-xl text-danger">
            {displayError}
          </div>
        )}

        {/* Tweet Form */}
        <div className="bg-card rounded-2xl shadow-sm border border-border p-6 mb-8">
          <h2 className="text-lg font-semibold mb-4">新しいツイート</h2>
          <form onSubmit={handleSubmit}>
            <textarea
              value={content}
              onChange={(e) => setContent(e.target.value)}
              placeholder="今何してる？"
              maxLength={280}
              required
              className="w-full min-h-[120px] p-4 border border-border rounded-xl resize-none focus:outline-none focus:ring-2 focus:ring-primary/30 focus:border-primary transition-all"
            />
            <div className="flex items-center justify-between mt-4">
              <span className="text-sm text-muted">{content.length}/280</span>
              <button
                type="submit"
                disabled={isSubmitting || !content.trim()}
                className="px-6 py-2 bg-primary text-white rounded-full font-medium hover:bg-primary-hover disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
              >
                {isSubmitting ? "投稿中..." : "ツイート"}
              </button>
            </div>
          </form>
        </div>

        {/* Timeline */}
        <div className="bg-card rounded-2xl shadow-sm border border-border overflow-hidden">
          <h2 className="text-lg font-semibold p-6 border-b border-border">
            タイムライン
          </h2>
          {tweets.length === 0 ? (
            <p className="p-6 text-muted text-center">
              まだツイートがありません。最初のツイートを投稿しましょう！
            </p>
          ) : (
            <ul>
              {tweets.map((tweet) => (
                <li
                  key={tweet.id}
                  className="p-6 border-b border-border last:border-b-0 hover:bg-slate-50 transition-colors"
                >
                  <div className="flex justify-between items-start gap-4">
                    <p className="flex-1 whitespace-pre-wrap break-words">
                      {tweet.content}
                    </p>
                    {tweet.user_id === user.id && (
                      <button
                        onClick={() => remove(tweet.id)}
                        className="text-muted hover:text-danger transition-colors"
                        title="削除"
                      >
                        🗑️
                      </button>
                    )}
                  </div>
                  <time className="text-sm text-muted mt-2 block">
                    {new Date(tweet.created_at).toLocaleString("ja-JP")}
                  </time>
                </li>
              ))}
            </ul>
          )}
        </div>
      </div>
    </main>
  );
}
