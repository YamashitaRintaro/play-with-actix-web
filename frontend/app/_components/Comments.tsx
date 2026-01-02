"use client";

import {
  useCommentsQuery,
  useCreateCommentMutation,
  useDeleteCommentMutation,
} from "@/lib/graphql/generated/urql";
import { useState, useCallback, type FormEvent } from "react";

interface Props {
  tweetId: string;
  currentUserId: string;
  onClose: () => void;
}

export function Comments({ tweetId, currentUserId, onClose }: Props) {
  const [{ data, fetching, error: queryError }, reexecuteQuery] =
    useCommentsQuery({ variables: { tweetId } });
  const [{ fetching: isCreating, error: createError }, createComment] =
    useCreateCommentMutation();
  const [{ fetching: isDeleting, error: deleteError }, deleteComment] =
    useDeleteCommentMutation();
  const [content, setContent] = useState("");

  const handleSubmit = useCallback(
    async (e: FormEvent) => {
      e.preventDefault();
      if (!content.trim()) return;

      const result = await createComment({ tweetId, content });

      if (!result.error) {
        setContent("");
        reexecuteQuery({ requestPolicy: "network-only" });
      }
    },
    [content, tweetId, createComment, reexecuteQuery]
  );

  const handleDelete = useCallback(
    async (commentId: string) => {
      if (!confirm("このコメントを削除しますか？")) return;

      const result = await deleteComment({ id: commentId });

      if (!result.error) {
        reexecuteQuery({ requestPolicy: "network-only" });
      }
    },
    [deleteComment, reexecuteQuery]
  );

  const comments = data?.comments ?? [];
  const error = queryError || createError || deleteError;

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
      <div className="bg-card rounded-2xl shadow-xl w-full max-w-lg max-h-[80vh] flex flex-col">
        {/* Header */}
        <header className="flex items-center justify-between p-4 border-b border-border">
          <h2 className="text-lg font-semibold">💬 コメント</h2>
          <button
            onClick={onClose}
            className="p-2 hover:bg-slate-100 rounded-full transition-colors"
            title="閉じる"
          >
            ✕
          </button>
        </header>

        {/* Error */}
        {error && (
          <div className="mx-4 mt-4 p-3 bg-red-50 border border-red-200 rounded-lg text-danger text-sm">
            {error.message}
          </div>
        )}

        {/* Comments List */}
        <div className="flex-1 overflow-y-auto p-4">
          {fetching ? (
            <p className="text-muted text-center py-8">読み込み中...</p>
          ) : comments.length === 0 ? (
            <p className="text-muted text-center py-8">
              まだコメントがありません。最初のコメントを投稿しましょう！
            </p>
          ) : (
            <ul className="space-y-4">
              {comments.map((comment) => (
                <li
                  key={comment.id}
                  className="p-4 bg-slate-50 rounded-xl border border-border"
                >
                  <div className="flex items-start justify-between gap-3">
                    <div className="flex-1 min-w-0">
                      <div className="flex items-center gap-2 mb-2">
                        <span className="font-medium text-sm">
                          @{comment.user?.username ?? "unknown"}
                        </span>
                        <time className="text-xs text-muted">
                          {new Date(comment.createdAt).toLocaleString("ja-JP")}
                        </time>
                      </div>
                      <p className="whitespace-pre-wrap break-words text-sm">
                        {comment.content}
                      </p>
                    </div>
                    {comment.userId === currentUserId && (
                      <button
                        onClick={() => handleDelete(comment.id)}
                        disabled={isDeleting}
                        className="text-muted hover:text-danger transition-colors disabled:opacity-50 flex-shrink-0"
                        title="削除"
                      >
                        🗑️
                      </button>
                    )}
                  </div>
                </li>
              ))}
            </ul>
          )}
        </div>

        {/* Comment Form */}
        <form
          onSubmit={handleSubmit}
          className="p-4 border-t border-border bg-slate-50 rounded-b-2xl"
        >
          <div className="flex gap-3">
            <input
              type="text"
              value={content}
              onChange={(e) => setContent(e.target.value)}
              placeholder="コメントを入力..."
              maxLength={280}
              className="flex-1 px-4 py-2 border border-border rounded-full focus:outline-none focus:ring-2 focus:ring-primary/30 focus:border-primary transition-all"
            />
            <button
              type="submit"
              disabled={isCreating || !content.trim()}
              className="px-5 py-2 bg-primary text-white rounded-full font-medium hover:bg-primary-hover disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
            >
              {isCreating ? "..." : "送信"}
            </button>
          </div>
          <p className="text-xs text-muted mt-2 text-right">
            {content.length}/280
          </p>
        </form>
      </div>
    </div>
  );
}
