"use client";

import { useCallback, useEffect, useState } from "react";
import { createTweet, deleteTweet, getTimeline } from "../../lib/api";
import type { Tweet } from "../../lib/types";

export function useTweets(userId: string | undefined) {
  const [tweets, setTweets] = useState<Tweet[]>([]);
  const [content, setContent] = useState("");
  const [error, setError] = useState("");
  const [isSubmitting, setIsSubmitting] = useState(false);

  const loadTimeline = useCallback(async () => {
    try {
      const data = await getTimeline();
      setTweets(data);
    } catch {
      setError("タイムラインの取得に失敗しました");
    }
  }, []);

  useEffect(() => {
    if (userId) loadTimeline();
  }, [userId, loadTimeline]);

  const submit = useCallback(async () => {
    if (!content.trim() || isSubmitting) return;

    setIsSubmitting(true);
    setError("");

    try {
      const newTweet = await createTweet({ content });
      setTweets((prev) => [newTweet, ...prev]);
      setContent("");
    } catch (err) {
      setError(
        err instanceof Error ? err.message : "ツイートの投稿に失敗しました"
      );
    } finally {
      setIsSubmitting(false);
    }
  }, [content, isSubmitting]);

  const remove = useCallback(async (id: string) => {
    if (!confirm("このツイートを削除しますか？")) return;

    try {
      await deleteTweet(id);
      setTweets((prev) => prev.filter((t) => t.id !== id));
    } catch {
      setError("ツイートの削除に失敗しました");
    }
  }, []);

  return {
    tweets,
    content,
    setContent,
    error,
    isSubmitting,
    submit,
    remove,
  };
}
