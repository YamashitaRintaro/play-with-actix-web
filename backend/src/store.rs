use sea_orm::{ConnectionTrait, Database, DatabaseConnection, DbErr, Statement};

pub type Db = DatabaseConnection;

/// データベース接続を作成し、テーブルを初期化する
pub async fn init_db() -> Result<DatabaseConnection, DbErr> {
    // SQLiteデータベースファイルへの接続
    let db = Database::connect("sqlite:./app.db?mode=rwc").await?;

    // テーブルの作成
    let backend = sea_orm::DatabaseBackend::Sqlite;

    // ユーザーテーブルの作成
    let stmt = Statement::from_sql_and_values(
        backend,
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY NOT NULL,
            username TEXT NOT NULL UNIQUE,
            email TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL,
            created_at TEXT NOT NULL
        )
        "#,
        [],
    );
    db.execute(stmt).await?;

    // ツイートテーブルの作成
    let stmt = Statement::from_sql_and_values(
        backend,
        r#"
        CREATE TABLE IF NOT EXISTS tweets (
            id TEXT PRIMARY KEY NOT NULL,
            user_id TEXT NOT NULL,
            content TEXT NOT NULL,
            created_at TEXT NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users(id)
        )
        "#,
        [],
    );
    db.execute(stmt).await?;

    // インデックスの作成
    let stmt = Statement::from_sql_and_values(
        backend,
        "CREATE INDEX IF NOT EXISTS idx_tweets_user_id ON tweets(user_id)",
        [],
    );
    db.execute(stmt).await?;

    let stmt = Statement::from_sql_and_values(
        backend,
        "CREATE INDEX IF NOT EXISTS idx_tweets_created_at ON tweets(created_at)",
        [],
    );
    db.execute(stmt).await?;

    // いいねテーブルの作成
    let stmt = Statement::from_sql_and_values(
        backend,
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
        [],
    );
    db.execute(stmt).await?;

    // いいねテーブルのインデックス作成
    let stmt = Statement::from_sql_and_values(
        backend,
        "CREATE INDEX IF NOT EXISTS idx_likes_tweet_id ON likes(tweet_id)",
        [],
    );
    db.execute(stmt).await?;

    let stmt = Statement::from_sql_and_values(
        backend,
        "CREATE INDEX IF NOT EXISTS idx_likes_user_id ON likes(user_id)",
        [],
    );
    db.execute(stmt).await?;

    Ok(db)
}
