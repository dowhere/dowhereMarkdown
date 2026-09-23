use std::error::Error;
use std::fs;
use std::path::Path;
use std::time::Duration;

use serde::Serialize;

// 将文本内容写入指定路径（导出 .md / .doc / .html）
#[tauri::command]
fn save_text_file(path: &str, contents: &str) -> Result<(), String> {
    fs::write(Path::new(path), contents).map_err(|e| e.to_string())
}

// 将二进制内容写入指定路径（导出 .png）
#[tauri::command]
fn save_binary_file(path: &str, bytes: Vec<u8>) -> Result<(), String> {
    fs::write(Path::new(path), &bytes).map_err(|e| e.to_string())
}

// 网页抓取结果：html 为页面源码，final_url 为重定向后的最终地址（用于解析相对链接）
#[derive(Serialize)]
struct FetchResult {
    html: String,
    final_url: String,
}

// 直接抓取网页：桌面端不受浏览器 CORS 限制，无需 Python 本地代理或公共代理
#[tauri::command]
async fn fetch_web_page(url: String) -> Result<FetchResult, String> {
    let client = reqwest::Client::builder()
        .user_agent(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 \
             (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36",
        )
        .timeout(Duration::from_secs(30))
        .redirect(reqwest::redirect::Policy::limited(10))
        .build()
        .map_err(|e| format!("HTTP 客户端初始化失败：{e}"))?;

    let response = client.get(&url).send().await.map_err(|e| {
        // 拼接完整错误链，便于区分 DNS / TCP / TLS / 协议层问题
        let mut detail = e.to_string();
        let mut src = e.source();
        while let Some(s) = src {
            detail += &format!(" <- {s}");
            src = s.source();
        }
        if e.is_timeout() {
            format!("请求超时（30 秒），请检查网络或系统代理设置（{detail}）")
        } else if e.is_connect() {
            format!("无法连接到服务器：{detail}")
        } else {
            detail
        }
    })?;

    let final_url = response.url().to_string();
    let status = response.status();
    if !status.is_success() {
        return Err(format!("服务器返回 HTTP {status}"));
    }

    // resp.text() 会按 Content-Type 的 charset 解码（GBK 等页面也能正确处理）
    let html = response
        .text()
        .await
        .map_err(|e| format!("读取网页内容失败：{e}"))?;
    if html.trim().is_empty() {
        return Err("网页内容为空".to_string());
    }

    Ok(FetchResult { html, final_url })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            save_text_file,
            save_binary_file,
            fetch_web_page
        ])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
