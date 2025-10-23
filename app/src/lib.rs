#![recursion_limit = "512"]
#![allow(dead_code, unused)]
pub mod env;
pub mod api;
pub mod components;
pub mod database;
pub mod db;
pub mod models;
pub mod pages;
//----------------------------------
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}
use tracing::{info, instrument};
fn initialize_frontend_logging() {
    // 设置 tracing-wasm 为全局默认日志处理器。
    // 这会将 tracing 事件重定向到浏览器的 console.log 等 API。
    // 日志级别在这里是固定的，或通过编译时特性控制。
    // 在 release 构建中，你可能希望禁用 debug 和 trace 级别的日志。
    #[cfg(debug_assertions)]
    let max_level = tracing::Level::DEBUG;
    #[cfg(not(debug_assertions))]
    let max_level = tracing::Level::INFO;
    tracing_wasm::set_as_global_default_with_config(
        tracing_wasm::WASMLayerConfigBuilder::new()
            .set_max_level(max_level)
            .build(),
    );
}
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    // 仅在组件首次挂载时运行一次初始化。
    Effect::new(|_| {
        initialize_frontend_logging();
        info!("Frontend logging initialized.");
    });
    view! {
        <Stylesheet id="leptos" href="/pkg/{{project-name}}.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
