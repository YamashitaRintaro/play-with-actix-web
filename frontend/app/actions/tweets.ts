"use server";

import { authApi, ApiError } from "@/lib/fetcher";
import type { Tweet } from "@/lib/types";
import { revalidatePath } from "next/cache";

interface ActionResult {
  error?: string;
  success?: boolean;
}

/** ツイート作成 */
export async function createTweetAction(
  formData: FormData
): Promise<ActionResult> {
  try {
    await authApi<Tweet>("/api/tweets", {
      method: "POST",
      body: { content: formData.get("content") },
    });

    revalidatePath("/");
    return { success: true };
  } catch (e) {
    if (e instanceof ApiError) {
      return { error: e.message };
    }
    return { error: "ツイートの投稿に失敗しました" };
  }
}

/** ツイート削除 */
export async function deleteTweetAction(
  tweetId: string
): Promise<ActionResult> {
  try {
    await authApi(`/api/tweets/${tweetId}`, { method: "DELETE" });

    revalidatePath("/");
    return { success: true };
  } catch (e) {
    if (e instanceof ApiError) {
      return { error: e.message };
    }
    return { error: "ツイートの削除に失敗しました" };
  }
}

/** タイムライン取得 */
export async function getTimelineAction(): Promise<{
  tweets: Tweet[];
  error?: string;
}> {
  try {
    const tweets = await authApi<Tweet[]>("/api/timeline");
    return { tweets };
  } catch (e) {
    if (e instanceof ApiError) {
      return { error: e.message, tweets: [] };
    }
    return { error: "タイムラインの取得に失敗しました", tweets: [] };
  }
}
