/* eslint-disable */
import * as types from './graphql';
import { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';

/**
 * Map of all GraphQL operations in the project.
 *
 * This map has several performance disadvantages:
 * 1. It is not tree-shakeable, so it will include all operations in the project.
 * 2. It is not minifiable, so the string of a GraphQL query will be multiple times inside the bundle.
 * 3. It does not support dead code elimination, so it will add unused operations.
 *
 * Therefore it is highly recommended to use the babel or swc plugin for production.
 * Learn more about it here: https://the-guild.dev/graphql/codegen/plugins/presets/preset-client#reducing-bundle-size
 */
type Documents = {
    "fragment UserFields on UserType {\n  id\n  username\n  email\n  followersCount\n  followingCount\n  isFollowing\n}\n\nfragment TweetFields on TweetType {\n  id\n  userId\n  content\n  createdAt\n  likeCount\n  isLiked\n  hashtags\n  user {\n    id\n    username\n  }\n}\n\nfragment CommentFields on CommentType {\n  id\n  tweetId\n  userId\n  content\n  createdAt\n  user {\n    id\n    username\n  }\n}": typeof types.UserFieldsFragmentDoc,
    "mutation Register($input: RegisterInput!) {\n  register(input: $input) {\n    token\n    user {\n      ...UserFields\n    }\n  }\n}\n\nmutation Login($input: LoginInput!) {\n  login(input: $input) {\n    token\n    user {\n      ...UserFields\n    }\n  }\n}\n\nmutation CreateTweet($content: String!) {\n  createTweet(content: $content) {\n    ...TweetFields\n  }\n}\n\nmutation DeleteTweet($id: UUID!) {\n  deleteTweet(id: $id)\n}\n\nmutation LikeTweet($tweetId: UUID!) {\n  likeTweet(tweetId: $tweetId)\n}\n\nmutation UnlikeTweet($tweetId: UUID!) {\n  unlikeTweet(tweetId: $tweetId)\n}\n\nmutation CreateComment($tweetId: UUID!, $content: String!) {\n  createComment(tweetId: $tweetId, content: $content) {\n    ...CommentFields\n  }\n}\n\nmutation DeleteComment($id: UUID!) {\n  deleteComment(id: $id)\n}\n\nmutation FollowUser($targetId: UUID!) {\n  followUser(targetId: $targetId)\n}\n\nmutation UnfollowUser($targetId: UUID!) {\n  unfollowUser(targetId: $targetId)\n}": typeof types.RegisterDocument,
    "query Me {\n  me {\n    ...UserFields\n  }\n}\n\nquery Timeline {\n  timeline {\n    ...TweetFields\n  }\n}\n\nquery Tweet($id: UUID!) {\n  tweet(id: $id) {\n    ...TweetFields\n  }\n}\n\nquery Comments($tweetId: UUID!) {\n  comments(tweetId: $tweetId) {\n    ...CommentFields\n  }\n}\n\nquery User($id: UUID!) {\n  user(id: $id) {\n    ...UserFields\n  }\n}\n\nquery Followers($userId: UUID!) {\n  followers(userId: $userId) {\n    ...UserFields\n  }\n}\n\nquery Following($userId: UUID!) {\n  following(userId: $userId) {\n    ...UserFields\n  }\n}": typeof types.MeDocument,
};
const documents: Documents = {
    "fragment UserFields on UserType {\n  id\n  username\n  email\n  followersCount\n  followingCount\n  isFollowing\n}\n\nfragment TweetFields on TweetType {\n  id\n  userId\n  content\n  createdAt\n  likeCount\n  isLiked\n  hashtags\n  user {\n    id\n    username\n  }\n}\n\nfragment CommentFields on CommentType {\n  id\n  tweetId\n  userId\n  content\n  createdAt\n  user {\n    id\n    username\n  }\n}": types.UserFieldsFragmentDoc,
    "mutation Register($input: RegisterInput!) {\n  register(input: $input) {\n    token\n    user {\n      ...UserFields\n    }\n  }\n}\n\nmutation Login($input: LoginInput!) {\n  login(input: $input) {\n    token\n    user {\n      ...UserFields\n    }\n  }\n}\n\nmutation CreateTweet($content: String!) {\n  createTweet(content: $content) {\n    ...TweetFields\n  }\n}\n\nmutation DeleteTweet($id: UUID!) {\n  deleteTweet(id: $id)\n}\n\nmutation LikeTweet($tweetId: UUID!) {\n  likeTweet(tweetId: $tweetId)\n}\n\nmutation UnlikeTweet($tweetId: UUID!) {\n  unlikeTweet(tweetId: $tweetId)\n}\n\nmutation CreateComment($tweetId: UUID!, $content: String!) {\n  createComment(tweetId: $tweetId, content: $content) {\n    ...CommentFields\n  }\n}\n\nmutation DeleteComment($id: UUID!) {\n  deleteComment(id: $id)\n}\n\nmutation FollowUser($targetId: UUID!) {\n  followUser(targetId: $targetId)\n}\n\nmutation UnfollowUser($targetId: UUID!) {\n  unfollowUser(targetId: $targetId)\n}": types.RegisterDocument,
    "query Me {\n  me {\n    ...UserFields\n  }\n}\n\nquery Timeline {\n  timeline {\n    ...TweetFields\n  }\n}\n\nquery Tweet($id: UUID!) {\n  tweet(id: $id) {\n    ...TweetFields\n  }\n}\n\nquery Comments($tweetId: UUID!) {\n  comments(tweetId: $tweetId) {\n    ...CommentFields\n  }\n}\n\nquery User($id: UUID!) {\n  user(id: $id) {\n    ...UserFields\n  }\n}\n\nquery Followers($userId: UUID!) {\n  followers(userId: $userId) {\n    ...UserFields\n  }\n}\n\nquery Following($userId: UUID!) {\n  following(userId: $userId) {\n    ...UserFields\n  }\n}": types.MeDocument,
};

/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 *
 *
 * @example
 * ```ts
 * const query = gql(`query GetUser($id: ID!) { user(id: $id) { name } }`);
 * ```
 *
 * The query argument is unknown!
 * Please regenerate the types.
 */
export function gql(source: string): unknown;

/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function gql(source: "fragment UserFields on UserType {\n  id\n  username\n  email\n  followersCount\n  followingCount\n  isFollowing\n}\n\nfragment TweetFields on TweetType {\n  id\n  userId\n  content\n  createdAt\n  likeCount\n  isLiked\n  hashtags\n  user {\n    id\n    username\n  }\n}\n\nfragment CommentFields on CommentType {\n  id\n  tweetId\n  userId\n  content\n  createdAt\n  user {\n    id\n    username\n  }\n}"): (typeof documents)["fragment UserFields on UserType {\n  id\n  username\n  email\n  followersCount\n  followingCount\n  isFollowing\n}\n\nfragment TweetFields on TweetType {\n  id\n  userId\n  content\n  createdAt\n  likeCount\n  isLiked\n  hashtags\n  user {\n    id\n    username\n  }\n}\n\nfragment CommentFields on CommentType {\n  id\n  tweetId\n  userId\n  content\n  createdAt\n  user {\n    id\n    username\n  }\n}"];
/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function gql(source: "mutation Register($input: RegisterInput!) {\n  register(input: $input) {\n    token\n    user {\n      ...UserFields\n    }\n  }\n}\n\nmutation Login($input: LoginInput!) {\n  login(input: $input) {\n    token\n    user {\n      ...UserFields\n    }\n  }\n}\n\nmutation CreateTweet($content: String!) {\n  createTweet(content: $content) {\n    ...TweetFields\n  }\n}\n\nmutation DeleteTweet($id: UUID!) {\n  deleteTweet(id: $id)\n}\n\nmutation LikeTweet($tweetId: UUID!) {\n  likeTweet(tweetId: $tweetId)\n}\n\nmutation UnlikeTweet($tweetId: UUID!) {\n  unlikeTweet(tweetId: $tweetId)\n}\n\nmutation CreateComment($tweetId: UUID!, $content: String!) {\n  createComment(tweetId: $tweetId, content: $content) {\n    ...CommentFields\n  }\n}\n\nmutation DeleteComment($id: UUID!) {\n  deleteComment(id: $id)\n}\n\nmutation FollowUser($targetId: UUID!) {\n  followUser(targetId: $targetId)\n}\n\nmutation UnfollowUser($targetId: UUID!) {\n  unfollowUser(targetId: $targetId)\n}"): (typeof documents)["mutation Register($input: RegisterInput!) {\n  register(input: $input) {\n    token\n    user {\n      ...UserFields\n    }\n  }\n}\n\nmutation Login($input: LoginInput!) {\n  login(input: $input) {\n    token\n    user {\n      ...UserFields\n    }\n  }\n}\n\nmutation CreateTweet($content: String!) {\n  createTweet(content: $content) {\n    ...TweetFields\n  }\n}\n\nmutation DeleteTweet($id: UUID!) {\n  deleteTweet(id: $id)\n}\n\nmutation LikeTweet($tweetId: UUID!) {\n  likeTweet(tweetId: $tweetId)\n}\n\nmutation UnlikeTweet($tweetId: UUID!) {\n  unlikeTweet(tweetId: $tweetId)\n}\n\nmutation CreateComment($tweetId: UUID!, $content: String!) {\n  createComment(tweetId: $tweetId, content: $content) {\n    ...CommentFields\n  }\n}\n\nmutation DeleteComment($id: UUID!) {\n  deleteComment(id: $id)\n}\n\nmutation FollowUser($targetId: UUID!) {\n  followUser(targetId: $targetId)\n}\n\nmutation UnfollowUser($targetId: UUID!) {\n  unfollowUser(targetId: $targetId)\n}"];
/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function gql(source: "query Me {\n  me {\n    ...UserFields\n  }\n}\n\nquery Timeline {\n  timeline {\n    ...TweetFields\n  }\n}\n\nquery Tweet($id: UUID!) {\n  tweet(id: $id) {\n    ...TweetFields\n  }\n}\n\nquery Comments($tweetId: UUID!) {\n  comments(tweetId: $tweetId) {\n    ...CommentFields\n  }\n}\n\nquery User($id: UUID!) {\n  user(id: $id) {\n    ...UserFields\n  }\n}\n\nquery Followers($userId: UUID!) {\n  followers(userId: $userId) {\n    ...UserFields\n  }\n}\n\nquery Following($userId: UUID!) {\n  following(userId: $userId) {\n    ...UserFields\n  }\n}"): (typeof documents)["query Me {\n  me {\n    ...UserFields\n  }\n}\n\nquery Timeline {\n  timeline {\n    ...TweetFields\n  }\n}\n\nquery Tweet($id: UUID!) {\n  tweet(id: $id) {\n    ...TweetFields\n  }\n}\n\nquery Comments($tweetId: UUID!) {\n  comments(tweetId: $tweetId) {\n    ...CommentFields\n  }\n}\n\nquery User($id: UUID!) {\n  user(id: $id) {\n    ...UserFields\n  }\n}\n\nquery Followers($userId: UUID!) {\n  followers(userId: $userId) {\n    ...UserFields\n  }\n}\n\nquery Following($userId: UUID!) {\n  following(userId: $userId) {\n    ...UserFields\n  }\n}"];

export function gql(source: string) {
  return (documents as any)[source] ?? {};
}

export type DocumentType<TDocumentNode extends DocumentNode<any, any>> = TDocumentNode extends DocumentNode<  infer TType,  any>  ? TType  : never;