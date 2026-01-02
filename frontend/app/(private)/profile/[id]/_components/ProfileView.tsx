"use client";

import {
  useUserQuery,
  useFollowersQuery,
  useFollowingQuery,
  useFollowUserMutation,
  useUnfollowUserMutation,
  UserType,
} from "@/lib/graphql/generated/urql";
import { useState, useCallback } from "react";
import Link from "next/link";

interface Props {
  userId: string;
  currentUserId: string;
}

type Tab = "followers" | "following";

const TABS: { key: Tab; label: string; emptyMessage: string }[] = [
  {
    key: "followers",
    label: "フォロワー",
    emptyMessage: "まだフォロワーがいません",
  },
  {
    key: "following",
    label: "フォロー中",
    emptyMessage: "まだ誰もフォローしていません",
  },
];

export function ProfileView({ userId, currentUserId }: Props) {
  const [activeTab, setActiveTab] = useState<Tab>("followers");
  const [
    { data: userData, fetching: userLoading, error: userError },
    refetchUser,
  ] = useUserQuery({ variables: { id: userId } });
  const [{ data: followersData, fetching: followersLoading }] =
    useFollowersQuery({
      variables: { userId },
      pause: activeTab !== "followers",
    });
  const [{ data: followingData, fetching: followingLoading }] =
    useFollowingQuery({
      variables: { userId },
      pause: activeTab !== "following",
    });

  const [{ fetching: isFollowing }, followUser] = useFollowUserMutation();
  const [{ fetching: isUnfollowing }, unfollowUser] = useUnfollowUserMutation();

  const user = userData?.user;
  const isOwnProfile = userId === currentUserId;

  const handleFollowToggle = useCallback(async () => {
    if (!user) return;

    const result = user.isFollowing
      ? await unfollowUser({ userId })
      : await followUser({ userId });

    if (!result.error) {
      refetchUser({ requestPolicy: "network-only" });
    }
  }, [user, userId, followUser, unfollowUser, refetchUser]);

  if (userLoading) {
    return (
      <div className="py-8">
        <div className="max-w-2xl mx-auto px-4 text-center text-muted">
          読み込み中...
        </div>
      </div>
    );
  }

  if (userError || !user) {
    return (
      <div className="py-8">
        <div className="max-w-2xl mx-auto px-4">
          <div className="bg-red-50 border border-red-200 rounded-xl p-6 text-center text-danger">
            {userError?.message || "ユーザーが見つかりません"}
          </div>
        </div>
      </div>
    );
  }

  const tabData = {
    followers: {
      users: followersData?.followers ?? [],
      loading: followersLoading,
    },
    following: {
      users: followingData?.following ?? [],
      loading: followingLoading,
    },
  } satisfies Record<Tab, { users: UserType[]; loading: boolean }>;
  const { users, loading } = tabData[activeTab];
  const currentTab = TABS.find((t) => t.key === activeTab)!;

  return (
    <div className="py-8">
      <div className="max-w-2xl mx-auto px-4">
        <ProfileCard
          user={user}
          isOwnProfile={isOwnProfile}
          isPending={isFollowing || isUnfollowing}
          activeTab={activeTab}
          onFollowToggle={handleFollowToggle}
          onTabChange={setActiveTab}
        />

        {/* タブパネル */}
        <div className="bg-card rounded-2xl shadow-sm border border-border overflow-hidden">
          <div role="tablist" className="flex border-b border-border">
            {TABS.map((tab) => (
              <button
                key={tab.key}
                role="tab"
                aria-selected={activeTab === tab.key}
                onClick={() => setActiveTab(tab.key)}
                className={`flex-1 py-4 font-medium transition-colors ${
                  activeTab === tab.key
                    ? "text-primary border-b-2 border-primary"
                    : "text-muted hover:text-primary"
                }`}
              >
                {tab.label}
              </button>
            ))}
          </div>

          <div role="tabpanel" className="divide-y divide-border">
            {loading ? (
              <p className="p-6 text-center text-muted">読み込み中...</p>
            ) : users.length === 0 ? (
              <p className="p-6 text-center text-muted">
                {currentTab.emptyMessage}
              </p>
            ) : (
              users.map((u) => (
                <UserListItem
                  key={u.id}
                  user={u}
                  currentUserId={currentUserId}
                />
              ))
            )}
          </div>
        </div>

        <div className="mt-6 text-center">
          <Link href="/" className="text-primary hover:underline">
            ← タイムラインに戻る
          </Link>
        </div>
      </div>
    </div>
  );
}

function ProfileCard({
  user,
  isOwnProfile,
  isPending,
  activeTab,
  onFollowToggle,
  onTabChange,
}: {
  user: UserType;
  isOwnProfile: boolean;
  isPending: boolean;
  activeTab: Tab;
  onFollowToggle: () => void;
  onTabChange: (tab: Tab) => void;
}) {
  return (
    <div className="bg-card rounded-2xl shadow-sm border border-border p-6 mb-6">
      <div className="flex items-start justify-between">
        <div>
          <h1 className="text-2xl font-bold">@{user.username}</h1>
          <p className="text-muted mt-1">{user.email}</p>
        </div>
        {!isOwnProfile && (
          <FollowButton
            isFollowing={user.isFollowing}
            isPending={isPending}
            onClick={onFollowToggle}
          />
        )}
      </div>

      <div className="flex gap-6 mt-6">
        <StatButton
          count={user.followingCount}
          label="フォロー中"
          isActive={activeTab === "following"}
          onClick={() => onTabChange("following")}
        />
        <StatButton
          count={user.followersCount}
          label="フォロワー"
          isActive={activeTab === "followers"}
          onClick={() => onTabChange("followers")}
        />
      </div>
    </div>
  );
}

function FollowButton({
  isFollowing,
  isPending,
  onClick,
}: {
  isFollowing: boolean;
  isPending: boolean;
  onClick: () => void;
}) {
  return (
    <button
      onClick={onClick}
      disabled={isPending}
      className={`px-6 py-2 rounded-full font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed ${
        isFollowing
          ? "bg-gray-200 text-gray-800 hover:bg-gray-300"
          : "bg-primary text-white hover:bg-primary-hover"
      }`}
    >
      {isPending ? "処理中..." : isFollowing ? "フォロー解除" : "フォローする"}
    </button>
  );
}

/** フォロー数/フォロワー数の表示ボタン */
function StatButton({
  count,
  label,
  isActive,
  onClick,
}: {
  count: number;
  label: string;
  isActive: boolean;
  onClick: () => void;
}) {
  return (
    <button
      onClick={onClick}
      className={`text-center transition-colors ${
        isActive ? "text-primary" : "hover:text-primary"
      }`}
    >
      <span className="text-xl font-bold">{count}</span>
      <span className="text-muted block text-sm">{label}</span>
    </button>
  );
}

interface UserListItemProps {
  user: {
    id: string;
    username: string;
    followersCount: number;
    followingCount: number;
    isFollowing: boolean;
  };
  currentUserId: string;
}

function UserListItem({ user, currentUserId }: UserListItemProps) {
  const [{ fetching: isFollowing }, followUser] = useFollowUserMutation();
  const [{ fetching: isUnfollowing }, unfollowUser] = useUnfollowUserMutation();
  const [isFollowingLocal, setIsFollowingLocal] = useState(user.isFollowing);

  const isOwnProfile = user.id === currentUserId;

  const handleFollowToggle = useCallback(async () => {
    const result = isFollowingLocal
      ? await unfollowUser({ userId: user.id })
      : await followUser({ userId: user.id });

    if (!result.error) {
      setIsFollowingLocal(!isFollowingLocal);
    }
  }, [user.id, isFollowingLocal, followUser, unfollowUser]);

  return (
    <div className="flex items-center justify-between p-4 hover:bg-slate-50 transition-colors">
      <Link href={`/profile/${user.id}`} className="flex-1">
        <div className="font-semibold text-primary hover:underline">
          @{user.username}
        </div>
        <div className="text-sm text-muted">
          {user.followersCount} フォロワー · {user.followingCount} フォロー中
        </div>
      </Link>
      {!isOwnProfile && (
        <button
          onClick={handleFollowToggle}
          disabled={isFollowing || isUnfollowing}
          className={`px-4 py-1.5 rounded-full text-sm font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed ${
            isFollowingLocal
              ? "bg-gray-200 text-gray-800 hover:bg-gray-300"
              : "bg-primary text-white hover:bg-primary-hover"
          }`}
        >
          {isFollowing || isUnfollowing
            ? "..."
            : isFollowingLocal
            ? "フォロー解除"
            : "フォロー"}
        </button>
      )}
    </div>
  );
}
