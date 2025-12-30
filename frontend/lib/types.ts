// GraphQL codegenで生成された型を再エクスポート
export type {
  UserType as User,
  TweetType as Tweet,
  AuthPayload,
  LoginInput,
  RegisterInput,
} from "./graphql/generated/urql";

// セッション用のシンプルなUser型（GraphQL型から抽出）
export interface SessionUser {
  id: string;
  username: string;
  email: string;
}
