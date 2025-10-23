#![cfg(feature = "ssr")]
static DB: std::sync::OnceLock<sqlx::SqlitePool> = std::sync::OnceLock::new();

#[cfg(feature = "ssr")]
use dotenvy::dotenv;
#[cfg(feature = "ssr")]
use sqlx::{Pool, Sqlite};
use std::env; // 导入必要的类型

// 改进 create_pool，返回 Result

async fn create_pool() -> Result<Pool<Sqlite>, Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok(); // 加载 .env，如果文件不存在则忽略

    let db_url = env::var("DATABASE_URL").map_err(|e| format!("DATABASE_URL not set: {}", e))?; // 使用 ? 运算符处理错误

    let pool = Pool::connect(&db_url)
        .await
        .map_err(|e| format!("Failed to connect to database: {}", e))?;

    // sqlx::migrate!()
    //     .run(&pool)
    //     .await
    //     .map_err(|e| format!("Database migrations failed: {}", e))?;

    Ok(pool)
}

// 改进 init_db，处理 OnceLock::set 的错误和 create_pool 的错误

pub async fn init_db() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let pool = create_pool().await?; // propagate errors from create_pool
    DB.set(pool)
        .map_err(|_| "Database already initialized".into()) // If set fails, it means it's already set
}

// 改进 get_db，返回 &'static sqlx::SqlitePool
pub fn get_db_pool() -> &'static Pool<Sqlite> {
    DB.get()
        .expect("Database not initialized. Call init_db() first.")
}

// 示例用法（在 main.rs 或其他地方）
#[cfg(test)] // 仅在测试时编译和运行此代码
mod tests {
    use super::*;

    // 假设你有 .env 文件，内容类似：DATABASE_URL=sqlite://sqlite.db
    // 并且有一个空的 `migrations` 文件夹或者一些 SQL 文件在里面
    #[tokio::test]
    async fn test_db_initialization_and_usage() {
        // 设置一个临时的 DATABASE_URL，以避免干扰实际数据库或确保测试隔离
        // 注意：这里需要确保你有一个可写的路径
        std::env::set_var("DATABASE_URL", "sqlite::memory:"); // Memory database for testing

        // 确保你已经创建了 migrations 目录，即使它是空的。
        // 或者在 CI/CD 中，这可能是不必要的，并且 `sqlx::migrate!` 宏会寻找 `migrations` 目录。
        // 对于内存数据库，通常不需要真实的迁移文件，但宏仍然会尝试查找。
        // 如果你的项目没有 `migrations` 目录，这一行可能会报错，你可以暂时注释掉 `sqlx::migrate!` 行来测试其他部分。

        init_db().await.unwrap(); // 初始化数据库，并断言成功

        let pool = get_db_pool(); // 获取数据库连接池

        // 示例：执行一个简单查询
        let result = sqlx::query!("SELECT 1 + 1 as sum")
            .fetch_one(pool)
            .await
            .expect("Failed to execute query");

        assert_eq!(result.sum, 2);

        info!("Database initialized and query executed successfully!");
    }
}
