#![cfg(feature = "ssr")]
use std::env;
use std::net::SocketAddr;

use dotenvy::dotenv;

pub fn get_bind_address() -> Result<SocketAddr, Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok(); // 加载 .env 文件，如果不存在则忽略
    
    // 从环境变量读取地址，如果未设置则使用默认值
    let bind_addr = env::var("LEPTOS_SITE_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:3000".to_string());
    
    // 解析为 SocketAddr
    bind_addr.parse()
        .map_err(|e| format!("Failed to parse LEPTOS_SITE_ADDR '{}': {}", bind_addr, e).into())
}

pub fn get_log_lev () ->String {
    dotenv().ok(); // 加载 .env 文件
    let log_level = env::var("RUST_LOG");
    println!("Read log level from .env as: {:?}",log_level);
    let log_level = log_level.unwrap_or_else(|_| "info".to_string());
    println!("Return log level as: {}",&log_level);
    log_level
}