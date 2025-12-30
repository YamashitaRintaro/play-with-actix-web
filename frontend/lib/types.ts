// API レスポンス型

export interface User {
  id: string;
  username: string;
  email: string;
}

export interface Tweet {
  id: string;
  user_id: string;
  content: string;
  created_at: string;
}

export interface AuthResponse {
  token: string;
  user: User;
}

// API リクエスト型

export interface RegisterRequest {
  username: string;
  email: string;
  password: string;
}

export interface LoginRequest {
  email: string;
  password: string;
}

export interface CreateTweetRequest {
  content: string;
}
