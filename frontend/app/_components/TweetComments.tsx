"use client";

import { useState, useCallback } from "react";
import { Comments } from "./Comments";

interface Props {
  tweetId: string;
  currentUserId: string;
}

export function TweetComments({ tweetId, currentUserId }: Props) {
  const [isOpen, setIsOpen] = useState(false);

  const handleToggle = useCallback(() => {
    setIsOpen((prev) => !prev);
  }, []);

  const handleClose = useCallback(() => {
    setIsOpen(false);
  }, []);

  return (
    <>
      <button
        onClick={handleToggle}
        className="flex items-center gap-2 px-3 py-1.5 rounded-full text-muted hover:bg-blue-50 hover:text-blue-500 transition-colors"
        title="コメント"
      >
        <span className="text-lg">💬</span>
      </button>

      {isOpen && (
        <Comments
          tweetId={tweetId}
          currentUserId={currentUserId}
          onClose={handleClose}
        />
      )}
    </>
  );
}
