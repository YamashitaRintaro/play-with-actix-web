"use client";

import { logout } from "@/app/actions/auth";
import { createTweetAction, deleteTweetAction } from "@/app/actions/tweets";
import type { Tweet, User } from "@/lib/types";
import { useState, useTransition } from "react";

interface Props {
  user: User;
  initialTweets: Tweet[];
}

export function Timeline({ user, initialTweets }: Props) {
  const [tweets, setTweets] = useState(initialTweets);
  const [error, setError] = useState("");
  const [isPending, startTransition] = useTransition();

  const handleCreateTweet = async (formData: FormData) => {
    const result = await createTweetAction(formData);
    if (result?.error) {
      setError(result.error);
    } else {
      // ページをリロードして最新のツイートを取得
      window.location.reload();
    }
  };

  const handleDelete = (tweetId: string) => {
    if (!confirm("このツイートを削除しますか？")) return;

    startTransition(async () => {
      const result = await deleteTweetAction(tweetId);
      if (result?.error) {
        setError(result.error);
      } else {
        setTweets(tweets.filter((t) => t.id !== tweetId));
      }
    });
  };

  return (
    <main className="min-h-screen py-8">
      <div className="max-w-2xl mx-auto px-4">
        {/* Header */}
        <header className="flex items-center justify-between mb-8">
          <h1 className="text-3xl font-bold text-primary">🐦 Twitter Clone</h1>
          <div className="flex items-center gap-4">
            <span className="text-muted">@{user.username}</span>
            <form action={logout}>
              <button
                type="submit"
                className="px-4 py-2 bg-danger text-white rounded-full font-medium hover:bg-danger-hover transition-colors"
              >
                ログアウト
              </button>
            </form>
          </div>
        </header>

        {/* Error */}
        {error && (
          <div className="mb-6 p-4 bg-red-50 border border-red-200 rounded-xl text-danger">
            {error}
          </div>
        )}

        {/* Tweet Form */}
        <div className="bg-card rounded-2xl shadow-sm border border-border p-6 mb-8">
          <h2 className="text-lg font-semibold mb-4">新しいツイート</h2>
          <form action={handleCreateTweet}>
            <textarea
              name="content"
              placeholder="今何してる？"
              maxLength={280}
              required
              className="w-full min-h-[120px] p-4 border border-border rounded-xl resize-none focus:outline-none focus:ring-2 focus:ring-primary/30 focus:border-primary transition-all"
            />
            <div className="flex items-center justify-end mt-4">
              <button
                type="submit"
                disabled={isPending}
                className="px-6 py-2 bg-primary text-white rounded-full font-medium hover:bg-primary-hover disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
              >
                {isPending ? "投稿中..." : "ツイート"}
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
                        onClick={() => handleDelete(tweet.id)}
                        disabled={isPending}
                        className="text-muted hover:text-danger transition-colors disabled:opacity-50"
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
