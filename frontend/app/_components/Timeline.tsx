"use client";

import { logout } from "@/app/actions/auth";
import {
  useTimelineQuery,
  useCreateTweetMutation,
  useDeleteTweetMutation,
  useLikeTweetMutation,
  useUnlikeTweetMutation,
  type UserType,
} from "@/lib/graphql/generated/urql";
import { useState, useCallback, type FormEvent } from "react";

interface Props {
  user: Pick<UserType, "id" | "username" | "email">;
}

export function Timeline({ user }: Props) {
  const [{ data, fetching, error: queryError }, reexecuteQuery] =
    useTimelineQuery();
  const [{ fetching: isCreating, error: createError }, createTweet] =
    useCreateTweetMutation();
  const [{ fetching: isDeleting, error: deleteError }, deleteTweet] =
    useDeleteTweetMutation();
  const [{ fetching: isLiking }, likeTweet] = useLikeTweetMutation();
  const [{ fetching: isUnliking }, unlikeTweet] = useUnlikeTweetMutation();
  const [content, setContent] = useState("");

  const handleSubmit = useCallback(
    async (e: FormEvent) => {
      e.preventDefault();
      if (!content.trim()) return;

      const result = await createTweet({ content });

      if (!result.error) {
        setContent("");
        reexecuteQuery({ requestPolicy: "network-only" });
      }
    },
    [content, createTweet, reexecuteQuery]
  );

  const handleDelete = useCallback(
    async (tweetId: string) => {
      if (!confirm("このツイートを削除しますか？")) return;

      const result = await deleteTweet({ id: tweetId });

      if (!result.error) {
        reexecuteQuery({ requestPolicy: "network-only" });
      }
    },
    [deleteTweet, reexecuteQuery]
  );

  const handleLike = useCallback(
    async (tweetId: string, isLiked: boolean) => {
      const result = isLiked
        ? await unlikeTweet({ tweetId })
        : await likeTweet({ tweetId });

      if (!result.error) {
        reexecuteQuery({ requestPolicy: "network-only" });
      }
    },
    [likeTweet, unlikeTweet, reexecuteQuery]
  );

  const tweets = data?.timeline ?? [];
  const error = queryError || createError || deleteError;
  const isLikeActionPending = isLiking || isUnliking;

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
            {error.message}
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
                disabled={isCreating || !content.trim()}
                className="px-6 py-2 bg-primary text-white rounded-full font-medium hover:bg-primary-hover disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
              >
                {isCreating ? "投稿中..." : "ツイート"}
              </button>
            </div>
          </form>
        </div>

        {/* Timeline */}
        <div className="bg-card rounded-2xl shadow-sm border border-border overflow-hidden">
          <h2 className="text-lg font-semibold p-6 border-b border-border">
            タイムライン
          </h2>
          {fetching ? (
            <p className="p-6 text-muted text-center">読み込み中...</p>
          ) : tweets.length === 0 ? (
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
                    {tweet.userId === user.id && (
                      <button
                        onClick={() => handleDelete(tweet.id)}
                        disabled={isDeleting}
                        className="text-muted hover:text-danger transition-colors disabled:opacity-50"
                        title="削除"
                      >
                        🗑️
                      </button>
                    )}
                  </div>
                  <div className="flex items-center justify-between mt-3">
                    <time className="text-sm text-muted">
                      {new Date(tweet.createdAt).toLocaleString("ja-JP")}
                    </time>
                    <button
                      onClick={() => handleLike(tweet.id, tweet.isLiked)}
                      disabled={isLikeActionPending}
                      className={`flex items-center gap-2 px-3 py-1.5 rounded-full transition-colors disabled:opacity-50 disabled:cursor-not-allowed ${
                        tweet.isLiked
                          ? "text-red-500 hover:bg-red-50"
                          : "text-muted hover:bg-gray-100"
                      }`}
                      title={tweet.isLiked ? "いいねを解除" : "いいね"}
                    >
                      <span className="text-lg">
                        {tweet.isLiked ? "❤️" : "🤍"}
                      </span>
                      <span className="text-sm font-medium">
                        {tweet.likeCount > 0 && tweet.likeCount}
                      </span>
                    </button>
                  </div>
                </li>
              ))}
            </ul>
          )}
        </div>
      </div>
    </main>
  );
}
