import gql from 'graphql-tag';
import * as Urql from 'urql';
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
export type MakeEmpty<T extends { [key: string]: unknown }, K extends keyof T> = { [_ in K]?: never };
export type Incremental<T> = T | { [P in keyof T]?: P extends ' $fragmentName' | '__typename' ? T[P] : never };
export type Omit<T, K extends keyof T> = Pick<T, Exclude<keyof T, K>>;
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: { input: string; output: string; }
  String: { input: string; output: string; }
  Boolean: { input: boolean; output: boolean; }
  Int: { input: number; output: number; }
  Float: { input: number; output: number; }
  UUID: { input: string; output: string; }
};

export type AuthPayload = {
  __typename?: 'AuthPayload';
  token: Scalars['String']['output'];
  user: UserType;
};

export type CommentType = {
  __typename?: 'CommentType';
  content: Scalars['String']['output'];
  createdAt: Scalars['String']['output'];
  id: Scalars['UUID']['output'];
  tweetId: Scalars['UUID']['output'];
  /** コメント投稿者の情報を取得 */
  user?: Maybe<UserType>;
  userId: Scalars['UUID']['output'];
};

/** ログイン入力 */
export type LoginInput = {
  email: Scalars['String']['input'];
  password: Scalars['String']['input'];
};

export type MutationRoot = {
  __typename?: 'MutationRoot';
  /** コメントを作成 */
  createComment: CommentType;
  createTweet: TweetType;
  /** コメントを削除（投稿者のみ） */
  deleteComment: Scalars['Boolean']['output'];
  deleteTweet: Scalars['Boolean']['output'];
  likeTweet: Scalars['Boolean']['output'];
  login: AuthPayload;
  register: AuthPayload;
  unlikeTweet: Scalars['Boolean']['output'];
};


export type MutationRootCreateCommentArgs = {
  content: Scalars['String']['input'];
  tweetId: Scalars['UUID']['input'];
};


export type MutationRootCreateTweetArgs = {
  content: Scalars['String']['input'];
};


export type MutationRootDeleteCommentArgs = {
  id: Scalars['UUID']['input'];
};


export type MutationRootDeleteTweetArgs = {
  id: Scalars['UUID']['input'];
};


export type MutationRootLikeTweetArgs = {
  tweetId: Scalars['UUID']['input'];
};


export type MutationRootLoginArgs = {
  input: LoginInput;
};


export type MutationRootRegisterArgs = {
  input: RegisterInput;
};


export type MutationRootUnlikeTweetArgs = {
  tweetId: Scalars['UUID']['input'];
};

export type QueryRoot = {
  __typename?: 'QueryRoot';
  /** ツイートへのコメント一覧を取得 */
  comments: Array<CommentType>;
  /** 現在のユーザー情報を取得 */
  me?: Maybe<UserType>;
  /** 現在のユーザーのタイムラインを取得 */
  timeline: Array<TweetType>;
  tweet?: Maybe<TweetType>;
};


export type QueryRootCommentsArgs = {
  tweetId: Scalars['UUID']['input'];
};


export type QueryRootTweetArgs = {
  id: Scalars['UUID']['input'];
};

/** 登録入力 */
export type RegisterInput = {
  email: Scalars['String']['input'];
  password: Scalars['String']['input'];
  username: Scalars['String']['input'];
};

export type TweetType = {
  __typename?: 'TweetType';
  content: Scalars['String']['output'];
  createdAt: Scalars['String']['output'];
  hashtags: Array<Scalars['String']['output']>;
  id: Scalars['UUID']['output'];
  isLiked: Scalars['Boolean']['output'];
  likeCount: Scalars['Int']['output'];
  userId: Scalars['UUID']['output'];
};

export type UserType = {
  __typename?: 'UserType';
  email: Scalars['String']['output'];
  id: Scalars['UUID']['output'];
  username: Scalars['String']['output'];
};

export type UserFieldsFragment = { __typename?: 'UserType', id: string, username: string, email: string };

export type TweetFieldsFragment = { __typename?: 'TweetType', id: string, userId: string, content: string, createdAt: string, likeCount: number, isLiked: boolean, hashtags: Array<string> };

export type CommentFieldsFragment = { __typename?: 'CommentType', id: string, tweetId: string, userId: string, content: string, createdAt: string, user?: { __typename?: 'UserType', id: string, username: string } | null };

export type RegisterMutationVariables = Exact<{
  input: RegisterInput;
}>;


export type RegisterMutation = { __typename?: 'MutationRoot', register: { __typename?: 'AuthPayload', token: string, user: { __typename?: 'UserType', id: string, username: string, email: string } } };

export type LoginMutationVariables = Exact<{
  input: LoginInput;
}>;


export type LoginMutation = { __typename?: 'MutationRoot', login: { __typename?: 'AuthPayload', token: string, user: { __typename?: 'UserType', id: string, username: string, email: string } } };

export type CreateTweetMutationVariables = Exact<{
  content: Scalars['String']['input'];
}>;


export type CreateTweetMutation = { __typename?: 'MutationRoot', createTweet: { __typename?: 'TweetType', id: string, userId: string, content: string, createdAt: string, likeCount: number, isLiked: boolean, hashtags: Array<string> } };

export type DeleteTweetMutationVariables = Exact<{
  id: Scalars['UUID']['input'];
}>;


export type DeleteTweetMutation = { __typename?: 'MutationRoot', deleteTweet: boolean };

export type LikeTweetMutationVariables = Exact<{
  tweetId: Scalars['UUID']['input'];
}>;


export type LikeTweetMutation = { __typename?: 'MutationRoot', likeTweet: boolean };

export type UnlikeTweetMutationVariables = Exact<{
  tweetId: Scalars['UUID']['input'];
}>;


export type UnlikeTweetMutation = { __typename?: 'MutationRoot', unlikeTweet: boolean };

export type CreateCommentMutationVariables = Exact<{
  tweetId: Scalars['UUID']['input'];
  content: Scalars['String']['input'];
}>;


export type CreateCommentMutation = { __typename?: 'MutationRoot', createComment: { __typename?: 'CommentType', id: string, tweetId: string, userId: string, content: string, createdAt: string, user?: { __typename?: 'UserType', id: string, username: string } | null } };

export type DeleteCommentMutationVariables = Exact<{
  id: Scalars['UUID']['input'];
}>;


export type DeleteCommentMutation = { __typename?: 'MutationRoot', deleteComment: boolean };

export type MeQueryVariables = Exact<{ [key: string]: never; }>;


export type MeQuery = { __typename?: 'QueryRoot', me?: { __typename?: 'UserType', id: string, username: string, email: string } | null };

export type TimelineQueryVariables = Exact<{ [key: string]: never; }>;


export type TimelineQuery = { __typename?: 'QueryRoot', timeline: Array<{ __typename?: 'TweetType', id: string, userId: string, content: string, createdAt: string, likeCount: number, isLiked: boolean, hashtags: Array<string> }> };

export type TweetQueryVariables = Exact<{
  id: Scalars['UUID']['input'];
}>;


export type TweetQuery = { __typename?: 'QueryRoot', tweet?: { __typename?: 'TweetType', id: string, userId: string, content: string, createdAt: string, likeCount: number, isLiked: boolean, hashtags: Array<string> } | null };

export type CommentsQueryVariables = Exact<{
  tweetId: Scalars['UUID']['input'];
}>;


export type CommentsQuery = { __typename?: 'QueryRoot', comments: Array<{ __typename?: 'CommentType', id: string, tweetId: string, userId: string, content: string, createdAt: string, user?: { __typename?: 'UserType', id: string, username: string } | null }> };

export const UserFieldsFragmentDoc = gql`
    fragment UserFields on UserType {
  id
  username
  email
}
    `;
export const TweetFieldsFragmentDoc = gql`
    fragment TweetFields on TweetType {
  id
  userId
  content
  createdAt
  likeCount
  isLiked
  hashtags
}
    `;
export const CommentFieldsFragmentDoc = gql`
    fragment CommentFields on CommentType {
  id
  tweetId
  userId
  content
  createdAt
  user {
    id
    username
  }
}
    `;
export const RegisterDocument = gql`
    mutation Register($input: RegisterInput!) {
  register(input: $input) {
    token
    user {
      ...UserFields
    }
  }
}
    ${UserFieldsFragmentDoc}`;

export function useRegisterMutation() {
  return Urql.useMutation<RegisterMutation, RegisterMutationVariables>(RegisterDocument);
};
export const LoginDocument = gql`
    mutation Login($input: LoginInput!) {
  login(input: $input) {
    token
    user {
      ...UserFields
    }
  }
}
    ${UserFieldsFragmentDoc}`;

export function useLoginMutation() {
  return Urql.useMutation<LoginMutation, LoginMutationVariables>(LoginDocument);
};
export const CreateTweetDocument = gql`
    mutation CreateTweet($content: String!) {
  createTweet(content: $content) {
    ...TweetFields
  }
}
    ${TweetFieldsFragmentDoc}`;

export function useCreateTweetMutation() {
  return Urql.useMutation<CreateTweetMutation, CreateTweetMutationVariables>(CreateTweetDocument);
};
export const DeleteTweetDocument = gql`
    mutation DeleteTweet($id: UUID!) {
  deleteTweet(id: $id)
}
    `;

export function useDeleteTweetMutation() {
  return Urql.useMutation<DeleteTweetMutation, DeleteTweetMutationVariables>(DeleteTweetDocument);
};
export const LikeTweetDocument = gql`
    mutation LikeTweet($tweetId: UUID!) {
  likeTweet(tweetId: $tweetId)
}
    `;

export function useLikeTweetMutation() {
  return Urql.useMutation<LikeTweetMutation, LikeTweetMutationVariables>(LikeTweetDocument);
};
export const UnlikeTweetDocument = gql`
    mutation UnlikeTweet($tweetId: UUID!) {
  unlikeTweet(tweetId: $tweetId)
}
    `;

export function useUnlikeTweetMutation() {
  return Urql.useMutation<UnlikeTweetMutation, UnlikeTweetMutationVariables>(UnlikeTweetDocument);
};
export const CreateCommentDocument = gql`
    mutation CreateComment($tweetId: UUID!, $content: String!) {
  createComment(tweetId: $tweetId, content: $content) {
    ...CommentFields
  }
}
    ${CommentFieldsFragmentDoc}`;

export function useCreateCommentMutation() {
  return Urql.useMutation<CreateCommentMutation, CreateCommentMutationVariables>(CreateCommentDocument);
};
export const DeleteCommentDocument = gql`
    mutation DeleteComment($id: UUID!) {
  deleteComment(id: $id)
}
    `;

export function useDeleteCommentMutation() {
  return Urql.useMutation<DeleteCommentMutation, DeleteCommentMutationVariables>(DeleteCommentDocument);
};
export const MeDocument = gql`
    query Me {
  me {
    ...UserFields
  }
}
    ${UserFieldsFragmentDoc}`;

export function useMeQuery(options?: Omit<Urql.UseQueryArgs<MeQueryVariables>, 'query'>) {
  return Urql.useQuery<MeQuery, MeQueryVariables>({ query: MeDocument, ...options });
};
export const TimelineDocument = gql`
    query Timeline {
  timeline {
    ...TweetFields
  }
}
    ${TweetFieldsFragmentDoc}`;

export function useTimelineQuery(options?: Omit<Urql.UseQueryArgs<TimelineQueryVariables>, 'query'>) {
  return Urql.useQuery<TimelineQuery, TimelineQueryVariables>({ query: TimelineDocument, ...options });
};
export const TweetDocument = gql`
    query Tweet($id: UUID!) {
  tweet(id: $id) {
    ...TweetFields
  }
}
    ${TweetFieldsFragmentDoc}`;

export function useTweetQuery(options: Omit<Urql.UseQueryArgs<TweetQueryVariables>, 'query'>) {
  return Urql.useQuery<TweetQuery, TweetQueryVariables>({ query: TweetDocument, ...options });
};
export const CommentsDocument = gql`
    query Comments($tweetId: UUID!) {
  comments(tweetId: $tweetId) {
    ...CommentFields
  }
}
    ${CommentFieldsFragmentDoc}`;

export function useCommentsQuery(options: Omit<Urql.UseQueryArgs<CommentsQueryVariables>, 'query'>) {
  return Urql.useQuery<CommentsQuery, CommentsQueryVariables>({ query: CommentsDocument, ...options });
};