"use client";

import {
  useTimelineQuery,
  useCreateTweetMutation,
  useDeleteTweetMutation,
  useLikeTweetMutation,
  useUnlikeTweetMutation,
} from "@/lib/graphql/generated/urql";
import { useState, useCallback, type FormEvent } from "react";
import { TweetComments } from "./TweetComments";

interface Props {
  userId: string;
}

export function Timeline({ userId }: Props) {
  const [{ data, fetching, error: queryError }, reexecuteQuery] =
    useTimelineQuery();
  const [{ fetching: isCreating, error: createError }, createTweet] =
    useCreateTweetMutation();
  const [content, setContent] = useState("");

  const refetch = useCallback(() => {
    reexecuteQuery({ requestPolicy: "network-only" });
  }, [reexecuteQuery]);

  const handleSubmit = useCallback(
    async (e: FormEvent) => {
      e.preventDefault();
      if (!content.trim()) return;

      const result = await createTweet({ content });

      if (!result.error) {
        setContent("");
        refetch();
      }
    },
    [content, createTweet, refetch]
  );

  const tweets = data?.timeline ?? [];
  const error = queryError || createError;

  return (
    <div className="py-8">
      <div className="max-w-2xl mx-auto px-4">
        {error && (
          <div className="mb-6 p-4 bg-red-50 border border-red-200 rounded-xl text-danger">
            {error.message}
          </div>
        )}

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
                    {tweet.userId === userId && (
                      <DeleteButton tweetId={tweet.id} onSuccess={refetch} />
                    )}
                  </div>

                  <HashtagList hashtags={tweet.hashtags ?? []} />

                  <div className="flex items-center justify-between mt-3">
                    <time className="text-sm text-muted">
                      {new Date(tweet.createdAt).toLocaleString("ja-JP")}
                    </time>
                    <div className="flex items-center gap-2">
                      <TweetComments
                        tweetId={tweet.id}
                        currentUserId={userId}
                      />
                      <LikeButton
                        tweetId={tweet.id}
                        isLiked={tweet.isLiked}
                        likeCount={tweet.likeCount}
                        onSuccess={refetch}
                      />
                    </div>
                  </div>
                </li>
              ))}
            </ul>
          )}
        </div>
      </div>
    </div>
  );
}

function LikeButton({
  tweetId,
  isLiked,
  likeCount,
  onSuccess,
}: {
  tweetId: string;
  isLiked: boolean;
  likeCount: number;
  onSuccess: () => void;
}) {
  const [{ fetching: isLiking }, likeTweet] = useLikeTweetMutation();
  const [{ fetching: isUnliking }, unlikeTweet] = useUnlikeTweetMutation();

  const handleToggle = useCallback(async () => {
    const result = isLiked
      ? await unlikeTweet({ tweetId })
      : await likeTweet({ tweetId });

    if (!result.error) {
      onSuccess();
    }
  }, [tweetId, isLiked, likeTweet, unlikeTweet, onSuccess]);

  const isPending = isLiking || isUnliking;

  return (
    <button
      onClick={handleToggle}
      disabled={isPending}
      className={`flex items-center gap-2 px-3 py-1.5 rounded-full transition-colors disabled:opacity-50 disabled:cursor-not-allowed ${
        isLiked
          ? "text-red-500 hover:bg-red-50"
          : "text-muted hover:bg-gray-100"
      }`}
      title={isLiked ? "いいねを解除" : "いいね"}
    >
      <span className="text-lg">{isLiked ? "❤️" : "🤍"}</span>
      {likeCount > 0 && (
        <span className="text-sm font-medium">{likeCount}</span>
      )}
    </button>
  );
}

function HashtagList({ hashtags }: { hashtags: string[] }) {
  if (hashtags.length === 0) return null;

  return (
    <div className="flex flex-wrap gap-2 mt-3">
      {hashtags.map((tag) => (
        <span
          key={tag}
          className="px-2 py-1 bg-primary/10 text-primary text-xs font-medium rounded-full"
        >
          #{tag}
        </span>
      ))}
    </div>
  );
}

function DeleteButton({
  tweetId,
  onSuccess,
}: {
  tweetId: string;
  onSuccess: () => void;
}) {
  const [{ fetching }, deleteTweet] = useDeleteTweetMutation();

  const handleDelete = useCallback(async () => {
    if (!confirm("このツイートを削除しますか？")) return;

    const result = await deleteTweet({ id: tweetId });

    if (!result.error) {
      onSuccess();
    }
  }, [tweetId, deleteTweet, onSuccess]);

  return (
    <button
      onClick={handleDelete}
      disabled={fetching}
      className="text-muted hover:text-danger transition-colors disabled:opacity-50"
      title="削除"
    >
      🗑️
    </button>
  );
}
