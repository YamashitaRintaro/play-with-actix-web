use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

pub type Db = SqlitePool;

/// データベース接続プールを作成し、テーブルを初期化する
pub async fn init_db() -> Result<SqlitePool, sqlx::Error> {
    // SQLiteデータベースファイルへの接続プールを作成
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:./app.db?mode=rwc")
        .await?;

    // ユーザーテーブルの作成
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY NOT NULL,
            username TEXT NOT NULL UNIQUE,
            email TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            created_at TEXT NOT NULL
        )
        "#,
    )
    .execute(&pool)
    .await?;

    // ツイートテーブルの作成
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS tweets (
            id TEXT PRIMARY KEY NOT NULL,
            user_id TEXT NOT NULL,
            content TEXT NOT NULL,
            created_at TEXT NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users(id)
        )
        "#,
    )
    .execute(&pool)
    .await?;

    // いいねテーブルの作成
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS likes (
            user_id TEXT NOT NULL,
            tweet_id TEXT NOT NULL,
            created_at TEXT NOT NULL,
            PRIMARY KEY (user_id, tweet_id),
            FOREIGN KEY (user_id) REFERENCES users(id),
            FOREIGN KEY (tweet_id) REFERENCES tweets(id)
        )
        "#,
    )
    .execute(&pool)
    .await?;

    // インデックスの作成
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_tweets_user_id ON tweets(user_id)")
        .execute(&pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_tweets_created_at ON tweets(created_at)")
        .execute(&pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_likes_tweet_id ON likes(tweet_id)")
        .execute(&pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_likes_user_id ON likes(user_id)")
        .execute(&pool)
        .await?;

    Ok(pool)
}
