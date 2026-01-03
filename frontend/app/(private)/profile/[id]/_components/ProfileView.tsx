"use client";

import {
  useUserQuery,
  useFollowersQuery,
  useFollowingQuery,
  useFollowUserMutation,
  useUnfollowUserMutation,
  UserType,
} from "@/lib/graphql/generated/urql";
import { useState, useCallback, type ReactNode } from "react";
import Link from "next/link";

interface Props {
  userId: string;
  currentUserId: string;
}

type Tab = "followers" | "following";

type UserInfo = Pick<
  UserType,
  "id" | "username" | "followersCount" | "followingCount" | "isFollowing"
>;

const TABS: { key: Tab; label: string }[] = [
  { key: "following", label: "フォロー中" },
  { key: "followers", label: "フォロワー" },
];

export function ProfileView({ userId, currentUserId }: Props) {
  return (
    <PageContainer>
      <ProfileViewContent userId={userId} currentUserId={currentUserId} />
      <div className="mt-6 text-center">
        <Link href="/" className="text-primary hover:underline">
          ← タイムラインに戻る
        </Link>
      </div>
    </PageContainer>
  );
}

function ProfileViewContent({ userId, currentUserId }: Props) {
  const [activeTab, setActiveTab] = useState<Tab>("followers");
  const [
    { data: userData, fetching: userLoading, error: userError },
    refetchUser,
  ] = useUserQuery({ variables: { id: userId } });

  const user = userData?.user;
  const isOwnProfile = userId === currentUserId;

  const handleRefetch = useCallback(() => {
    refetchUser({ requestPolicy: "network-only" });
  }, [refetchUser]);

  if (userLoading) {
    return <p className="text-center text-muted">読み込み中...</p>;
  }

  if (userError || !user) {
    return (
      <Card className="bg-red-50 border-red-200 p-6 text-center text-danger">
        {userError?.message || "ユーザーが見つかりません"}
      </Card>
    );
  }

  return (
    <>
      <ProfileCard user={user}>
        {!isOwnProfile && (
          <FollowAction
            targetId={userId}
            user={user}
            onSuccess={handleRefetch}
          />
        )}
      </ProfileCard>

      <Card className="overflow-hidden">
        <TabList tabs={TABS} activeTab={activeTab} onTabChange={setActiveTab} />
        {activeTab === "followers" && (
          <FollowersTab
            userId={userId}
            currentUserId={currentUserId}
            onFollowChange={handleRefetch}
          />
        )}
        {activeTab === "following" && (
          <FollowingTab
            userId={userId}
            currentUserId={currentUserId}
            onFollowChange={handleRefetch}
          />
        )}
      </Card>
    </>
  );
}

interface TabContentProps {
  userId: string;
  currentUserId: string;
  onFollowChange: () => void;
}

const requestPolicy = "cache-and-network";
const refetcnPoicy = "network-only";

function FollowersTab({
  userId,
  currentUserId,
  onFollowChange,
}: TabContentProps) {
  const [{ data, fetching }, refetch] = useFollowersQuery({
    variables: { userId },
    requestPolicy,
  });

  const handleFollowChange = useCallback(() => {
    onFollowChange();
    refetch({ requestPolicy: refetcnPoicy });
  }, [onFollowChange, refetch]);

  if (fetching && !data) {
    return <p className="p-6 text-center text-muted">読み込み中...</p>;
  }

  const followers = data?.followers ?? [];

  if (followers.length === 0) {
    return (
      <p className="p-6 text-center text-muted">まだフォロワーがいません</p>
    );
  }

  return (
    <div role="tabpanel" className="divide-y divide-border">
      {followers.map((user) => (
        <UserListItem
          key={user.id}
          user={user}
          currentUserId={currentUserId}
          onFollowChange={handleFollowChange}
        />
      ))}
    </div>
  );
}

function FollowingTab({
  userId,
  currentUserId,
  onFollowChange,
}: TabContentProps) {
  const [{ data, fetching }, refetch] = useFollowingQuery({
    variables: { userId },
    requestPolicy,
  });

  const handleFollowChange = useCallback(() => {
    onFollowChange();
    refetch({ requestPolicy: refetcnPoicy });
  }, [onFollowChange, refetch]);

  if (fetching && !data) {
    return <p className="p-6 text-center text-muted">読み込み中...</p>;
  }

  const following = data?.following ?? [];

  if (following.length === 0) {
    return (
      <p className="p-6 text-center text-muted">まだ誰もフォローしていません</p>
    );
  }

  return (
    <div role="tabpanel" className="divide-y divide-border">
      {following.map((user) => (
        <UserListItem
          key={user.id}
          user={user}
          currentUserId={currentUserId}
          onFollowChange={handleFollowChange}
        />
      ))}
    </div>
  );
}

function PageContainer({ children }: { children: ReactNode }) {
  return (
    <div className="py-8">
      <div className="max-w-2xl mx-auto px-4">{children}</div>
    </div>
  );
}

function Card({
  children,
  className = "",
}: {
  children: ReactNode;
  className?: string;
}) {
  return (
    <div
      className={`bg-card rounded-2xl shadow-sm border border-border ${className}`}
    >
      {children}
    </div>
  );
}

function ProfileCard({
  user,
  children,
}: {
  user: UserType;
  children: ReactNode;
}) {
  return (
    <Card className="p-6 mb-6">
      <div className="flex items-start justify-between">
        <UserHeader username={user.username} email={user.email} />
        {children}
      </div>

      <div className="flex gap-6 mt-6">
        <Stat count={user.followingCount} label="フォロー中" />
        <Stat count={user.followersCount} label="フォロワー" />
      </div>
    </Card>
  );
}

function UserHeader({ username, email }: { username: string; email: string }) {
  return (
    <div>
      <h1 className="text-2xl font-bold">@{username}</h1>
      {email && <p className="text-muted mt-1">{email}</p>}
    </div>
  );
}

function Stat({ count, label }: { count: number; label: string }) {
  return (
    <div className="text-center">
      <span className="text-xl font-bold">{count}</span>
      <span className="text-muted block text-sm">{label}</span>
    </div>
  );
}

function TabList({
  tabs,
  activeTab,
  onTabChange,
}: {
  tabs: typeof TABS;
  activeTab: Tab;
  onTabChange: (tab: Tab) => void;
}) {
  return (
    <div role="tablist" className="flex border-b border-border">
      {tabs.map((tab) => (
        <button
          key={tab.key}
          role="tab"
          aria-selected={activeTab === tab.key}
          onClick={() => onTabChange(tab.key)}
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
  );
}

type Size = "default" | "small";
const sizeStyles: Record<Size, string> = {
  default: "px-6 py-2 text-base",
  small: "px-4 py-1.5 text-sm",
};

function FollowButton({
  isFollowing,
  isPending,
  onClick,
  size = "default",
}: {
  isFollowing: boolean;
  isPending: boolean;
  onClick: () => void;
  size?: Size;
}) {
  const label = (() => {
    if (isPending) return size === "small" ? "..." : "処理中...";
    if (isFollowing) return "フォロー解除";
    return size === "small" ? "フォロー" : "フォローする";
  })();

  return (
    <button
      onClick={onClick}
      disabled={isPending}
      className={`rounded-full font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed ${
        sizeStyles[size]
      } ${
        isFollowing
          ? "bg-gray-200 text-gray-800 hover:bg-gray-300"
          : "bg-primary text-white hover:bg-primary-hover"
      }`}
    >
      {label}
    </button>
  );
}

function FollowAction({
  targetId,
  user,
  onSuccess,
  size = "default",
}: {
  targetId: string;
  user: UserInfo;
  onSuccess: () => void;
  size?: Size;
}) {
  const [{ fetching: isFollowPending }, followUser] = useFollowUserMutation();
  const [{ fetching: isUnfollowPending }, unfollowUser] =
    useUnfollowUserMutation();

  const isPending = isFollowPending || isUnfollowPending;

  const handleToggle = useCallback(async () => {
    const result = user.isFollowing
      ? await unfollowUser({ targetId })
      : await followUser({ targetId });

    if (!result.error) {
      onSuccess();
    }
  }, [targetId, user.isFollowing, followUser, unfollowUser, onSuccess]);

  return (
    <FollowButton
      isFollowing={user.isFollowing}
      isPending={isPending}
      onClick={handleToggle}
      size={size}
    />
  );
}

function UserListItem({
  user,
  currentUserId,
  onFollowChange,
}: {
  user: UserInfo;
  currentUserId: string;
  onFollowChange: () => void;
}) {
  const isOwnProfile = user.id === currentUserId;

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
        <FollowAction
          targetId={user.id}
          user={user}
          onSuccess={onFollowChange}
          size="small"
        />
      )}
    </div>
  );
}
