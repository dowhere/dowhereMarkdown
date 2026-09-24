use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

use encoding_rs::{Encoding, UTF_8};
use serde::Serialize;

// 按指定编码（IANA 标签，None 或无法识别时为 UTF-8）将文本写入文件
// 注：encoding_rs 对目标编码无法表示的字符会输出 ASCII 数字字符实体（&#..;），不丢内容
#[tauri::command]
fn save_text_file(path: &str, contents: &str, encoding: Option<String>) -> Result<(), String> {
    let enc = encoding
        .as_deref()
        .and_then(|label| Encoding::for_label(label.as_bytes()))
        .unwrap_or(UTF_8);
    let (bytes, _enc, _had_errors) = enc.encode(contents);
    fs::write(Path::new(path), &bytes).map_err(|e| e.to_string())
}

// 将二进制内容写入指定路径（导出 .png）
#[tauri::command]
fn save_binary_file(path: &str, bytes: Vec<u8>) -> Result<(), String> {
    fs::write(Path::new(path), &bytes).map_err(|e| e.to_string())
}

// 图片保存结果：path 为绝对路径（分隔符统一为 /，便于直接作为 Markdown 链接）
#[derive(Serialize)]
struct ImageSaveResult {
    path: String,
}

// 按时间戳自动命名保存图片到指定目录：image-{毫秒时间戳}.{ext}，
// 冲突时追加 -1、-2…；目录不存在自动创建
#[tauri::command]
fn save_image_file(dir: &str, ext: &str, bytes: Vec<u8>) -> Result<ImageSaveResult, String> {
    fs::create_dir_all(dir).map_err(|e| format!("创建图片目录失败: {e}"))?;
    let millis = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_millis();
    // 扩展名只保留字母数字，防止路径穿越
    let safe_ext: String = ext
        .chars()
        .take(8)
        .filter(|c| c.is_ascii_alphanumeric())
        .collect();
    let safe_ext = if safe_ext.is_empty() { "png".to_string() } else { safe_ext };

    let base = PathBuf::from(dir);
    let mut candidate = base.join(format!("image-{millis}.{safe_ext}"));
    let mut suffix = 1;
    while candidate.exists() {
        candidate = base.join(format!("image-{millis}-{suffix}.{safe_ext}"));
        suffix += 1;
    }
    fs::write(&candidate, &bytes).map_err(|e| e.to_string())?;
    Ok(ImageSaveResult {
        path: candidate.to_string_lossy().replace('\\', "/"),
    })
}

// 按指定编码读取文本文件（打开/最近打开；None 或无法识别时为 UTF-8）
#[tauri::command]
fn read_text_file(path: String, encoding: Option<String>) -> Result<String, String> {
    let bytes = fs::read(Path::new(&path)).map_err(|e| format!("读取文件失败: {e}"))?;
    let enc = encoding
        .and_then(|label| Encoding::for_label(label.as_bytes()))
        .unwrap_or(UTF_8);
    Ok(enc.decode(&bytes).0.into_owned())
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

// 查找系统 Microsoft Edge（Windows 11 自带，用于 headless 生成 PDF）
fn find_edge() -> Option<PathBuf> {
    let candidates = [
        r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe",
        r"C:\Program Files\Microsoft\Edge\Application\msedge.exe",
    ];
    candidates.iter().map(PathBuf::from).find(|p| p.exists())
}

// 清理历史遗留的 PDF 临时文件（异常退出时可能残留，超过一天才删，避免碰到正在进行的任务）
fn cleanup_old_pdf_temp() {
    let tmp = std::env::temp_dir();
    let Ok(entries) = fs::read_dir(&tmp) else {
        return;
    };
    let cutoff = std::time::SystemTime::now().checked_sub(Duration::from_secs(24 * 3600));
    for entry in entries.flatten() {
        let name = entry.file_name();
        let name = name.to_string_lossy();
        let is_ours = name.starts_with("edge-pdf-md-editor-")
            || (name.starts_with("md-editor-") && name.ends_with(".html"));
        if !is_ours {
            continue;
        }
        let is_old = match (cutoff, entry.metadata().and_then(|m| m.modified())) {
            (Some(cutoff), Ok(modified)) => modified < cutoff,
            _ => false,
        };
        if is_old {
            let path = entry.path();
            if path.is_dir() {
                let _ = fs::remove_dir_all(&path);
            } else {
                let _ = fs::remove_file(&path);
            }
        }
    }
}

// 等待 PDF 写完：文件大小连续约 1 秒不变，且以 %PDF 开头、尾部包含 %%EOF
fn wait_for_pdf_ready(path: &Path, timeout: Duration) -> Result<(), String> {
    let start = std::time::Instant::now();
    let mut last_size: Option<u64> = None;
    let mut stable = 0u32;

    loop {
        if let Ok(meta) = fs::metadata(path) {
            let len = meta.len();
            if last_size == Some(len) && len > 0 {
                stable += 1;
            } else {
                stable = 0;
                last_size = Some(len);
            }

            if stable >= 2 {
                if let Ok(bytes) = fs::read(path) {
                    let head_ok = bytes.starts_with(b"%PDF");
                    let tail_start = bytes.len().saturating_sub(1024);
                    let tail_ok = bytes[tail_start..].windows(5).any(|w| w == b"%%EOF");
                    if head_ok && tail_ok {
                        return Ok(());
                    }
                }
                // 大小稳定但结构不完整（写入中途），重新等待
                stable = 0;
            }
        }

        if start.elapsed() >= timeout {
            return Err(format!("PDF 生成超时（等待 {} 秒仍未完成）", timeout.as_secs()));
        }
        std::thread::sleep(Duration::from_millis(500));
    }
}

// 将自包含 HTML 渲染为真正的 PDF 文件（Edge headless --print-to-pdf）
#[tauri::command]
async fn html_to_pdf(html: String, out_path: String) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || -> Result<(), String> {
        cleanup_old_pdf_temp();

        let tmp = std::env::temp_dir();
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let unique = format!("md-editor-{}-{}", std::process::id(), nanos);
        let html_path = tmp.join(format!("{unique}.html"));
        let profile = tmp.join(format!("edge-pdf-{unique}"));
        fs::write(&html_path, &html).map_err(|e| format!("写临时文件失败：{e}"))?;

        let edge = find_edge()
            .ok_or_else(|| "未找到 Microsoft Edge，无法生成 PDF".to_string())?;
        let file_url = format!("file:///{}", html_path.display().to_string().replace('\\', "/"));

        let mut child = Command::new(edge)
            .args([
                "--headless=new",
                "--disable-gpu",
                "--no-pdf-header-footer",
                "--print-to-pdf-no-header",
                "--run-all-compositor-stages-before-draw",
                "--virtual-time-budget=15000",
                &format!("--print-to-pdf={out_path}"),
                &format!("--user-data-dir={}", profile.display()),
                &file_url,
            ])
            .spawn()
            .map_err(|e| format!("启动 Edge 失败：{e}"))?;

        // msedge.exe 启动器会在渲染子进程完成前提前退出（退出码 0），
        // 不能以进程退出作为完成标志，必须轮询 PDF 文件
        let pdf_result = wait_for_pdf_ready(Path::new(&out_path), Duration::from_secs(90));

        // PDF 已写完，渲染进程必然已读完临时 HTML，此时删除才安全
        let _ = child.try_wait();
        std::thread::sleep(Duration::from_millis(300));
        let _ = fs::remove_file(&html_path);
        let _ = fs::remove_dir_all(&profile);

        pdf_result
    })
    .await
    .map_err(|e| format!("任务执行失败：{e}"))?
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .invoke_handler(tauri::generate_handler![
            save_text_file,
            save_binary_file,
            save_image_file,
            read_text_file,
            fetch_web_page,
            html_to_pdf
        ])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
