# DoWhereMarkdown

<p align="center">
  <img src="src/logo.png" alt="DoWhereMarkdown" width="128">
</p>

**DoWhereMarkdown** 是一款简洁高效的桌面 Markdown 编辑器，左侧写作、右侧实时预览。基于 Tauri 2 构建，前端为零构建工具的原生 HTML/CSS/JavaScript，所有第三方库均已本地化，**完全离线可用**。

- 版本：0.1.0
- 平台：Windows 10 / 11（NSIS 安装包）
- 作者技术博客：<https://www.dowhere.com>

---

## 功能特性

### ✍️ 编辑与实时预览

- 双栏布局（编辑 + 预览），支持「编辑 + 预览 / 仅编辑 / 仅预览」三种模式
- 输入即时渲染，预览区与编辑区滚动同步
- 面板可折叠，分隔线可拖拽调整宽度
- 撤销 / 重做历史（最多 100 步）
- 查找与替换（`Ctrl+F` / `Ctrl+H`）

### 📝 Markdown 语法支持

- 标题 H1–H6、加粗、斜体、下划线、删除线、上标 / 下标
- 引用块、有序 / 无序列表、GFM 任务列表
- 行内代码与围栏代码块
- 链接、图片、表格、分隔线
- 代码块 **30+ 种语言语法高亮**（highlight.js）
- **数学公式**（KaTeX）：`$...$` 行内公式、`$$...$$` 块级公式
- **Mermaid 图表 12 种**：流程图、时序图、类图、状态图、ER 图、用户旅程图、甘特图、饼图、思维导图、时间线、Git 图、象限图；插入对话框支持实时预览

### 🗂️ 大纲导航

- 直接从 **Markdown 源码**解析标题（正确排除围栏代码块中的 `#`，支持 Setext 标题）
- 按层级缩进显示，点击标题在编辑器与预览区同步跳转
- 编辑 / 预览滚动时自动高亮当前章节
- 标题前的三角箭头可树形折叠 / 展开子标题
- 面板宽度可拖拽，折叠状态自动持久化

### 📂 文件、导入与导出

- **文件操作**：新建、打开、保存、另存为、最近打开（最多 8 条，失效记录自动清理）、打开文件位置、退出
- **多编码支持**：UTF-8、GBK、GB18030、Big5（读写均按所选编码处理）
- **导入**：Markdown、TXT、HTML、DOCX
  - DOCX 通过 mammoth 转换（保留标题、粗斜体、列表、表格、图片）
  - HTML 自动提取正文并转换为 Markdown
- **导出为**：
  - Markdown（.md）
  - Word（.doc）
  - PDF（.pdf）——调用系统自带的 Microsoft Edge 无头渲染生成，非打印对话框
  - HTML（.html，自包含页面）
  - PNG 图片——支持 9:16、4:5、3:4、1:1、16:9 五种比例
- 支持将文件或图片**直接拖入窗口**：文档文件直接打开，图片插入正文

### 🌐 网页转 Markdown

- 「工具 → 网页转 MD」输入网址即可抓取正文并转换为 Markdown
- 桌面端由 Rust（reqwest）直接发起请求，不受浏览器 CORS 限制
- 自动跟随重定向、按页面 charset 解码（兼容 GBK 等编码）
- 转换保留图片（含懒加载图片）、链接、表格、嵌套列表、任务列表与代码块，相对路径自动转绝对路径

### 🖼️ 图片处理

- 粘贴 / 选择图片时，可选择 **Base64 嵌入文档** 或 **保存到本地目录**
- 本地模式按时间戳自动命名（`image-{时间戳}.png`），以绝对路径插入文档

### 🌍 界面与体验

- **10 种界面语言**：简体中文、繁體中文、English、日本語、한국어、Español、Français、Deutsch、Русский、Português
- 亮色 / 暗色主题一键切换
- 按区域定制的**自定义右键菜单**（编辑器、预览区链接 / 图片、大纲、输入框、空白区域）
- 预览区外部链接自动交由系统浏览器打开，不会导致应用界面跳转
- **关闭窗口时提示保存**：有未保存更改时弹出「保存 / 不保存 / 取消」确认
- 当前文件名实时显示在窗口标题（格式：`文件名 - DoWhereMarkdown`）
- 全屏切换（`F11`）

---

## 技术栈

| 部分 | 技术 |
| --- | --- |
| 桌面框架 | Tauri 2.x（`withGlobalTauri`，前端通过 `window.__TAURI__` 调用原生能力） |
| 后端语言 | Rust（std::fs 文件读写、reqwest + rustls 网页抓取、encoding_rs 编解码） |
| Tauri 插件 | dialog（原生打开 / 保存对话框）、opener（打开链接 / 文件位置）、clipboard-manager（剪贴板） |
| 前端 | 原生 HTML / CSS / JavaScript，单文件、无构建步骤 |
| Markdown 解析 | marked + marked-highlight |
| 代码高亮 | highlight.js（30+ 语言包） |
| 数学公式 | KaTeX 0.16.9 |
| 图表 | Mermaid 10.9.8 |
| DOCX 解析 | mammoth 1.12.3 |
| 图片导出 | dom-to-image-more |

> 所有前端依赖均放在 `src/vendor/` 目录本地加载，运行与打包均无需联网，也不依赖任何 CDN。

---

## 项目结构

```
.
├── package.json               # 仅依赖 @tauri-apps/cli
├── make_icon.py               # 从 logo.png 生成 Tauri 应用图标
├── logo.png                   # 产品 Logo
├── src/                       # 前端（tauri.conf.json 中 frontendDist 指向此处）
│   ├── index.html             # 应用主体：界面结构 + 样式 + 全部业务脚本
│   ├── logo.png
│   └── vendor/                # 本地化的第三方库与语言包
│       ├── i18n.js            # 10 种界面语言文案
│       ├── marked.min.js
│       ├── marked-highlight.umd.js
│       ├── highlight.min.js
│       ├── lang-*.min.js      # highlight.js 各语言定义
│       ├── katex.min.js / katex.min.css
│       ├── fonts/             # KaTeX woff2 字体
│       ├── mermaid.min.js
│       ├── mammoth.browser.min.js
│       └── dom-to-image-more.min.js
└── src-tauri/                 # Rust 后端与打包配置
    ├── Cargo.toml
    ├── build.rs
    ├── tauri.conf.json        # Tauri 配置（窗口、打包目标、安全策略）
    ├── capabilities/
    │   └── default.json       # 窗口 / 对话框 / 剪贴板权限声明
    ├── icons/                 # 应用图标（各尺寸 PNG / ICO / ICNS）
    └── src/
        └── lib.rs             # 原生命令：文件读写、网页抓取、Edge 生成 PDF
```

Rust 端提供的 `#[tauri::command]`：

| 命令 | 作用 |
| --- | --- |
| `read_text_file` | 按指定编码读取文本文件 |
| `save_text_file` | 按指定编码写入文本文件（.md / .doc / .html） |
| `save_binary_file` | 写入二进制文件（.png） |
| `save_image_file` | 图片按时间戳命名保存到指定目录，返回绝对路径 |
| `fetch_web_page` | 抓取网页源码（自动重定向、charset 解码） |
| `html_to_pdf` | 调用 Microsoft Edge 无头模式将 HTML 渲染为 PDF |

---

## 环境要求

- Windows 10 / 11
- [Node.js](https://nodejs.org/)（开发期使用，建议 18+）
- [Rust](https://www.rust-lang.org/tools/install) 工具链（stable）
- Microsoft Edge（Windows 10/11 系统自带，**仅导出 PDF 时需要**）
- 首次执行 `tauri build` 时会自动下载 NSIS 打包工具链（需联网一次）

---

## 快速开始

### 1. 安装依赖

```bash
npm install
```

### 2. 开发运行

```bash
npm run tauri dev
```

该命令会启动 WebView2 开发窗口，修改 `src/index.html` 后刷新窗口即可看到变化。

### 3. 构建安装包

```bash
npm run tauri build
```

构建完成后，NSIS 安装程序位于：

```
src-tauri/target/release/bundle/nsis/DoWhereMarkdown_0.1.0_x64-setup.exe
```

### 4. 浏览器中直接打开（免安装）

由于前端是纯静态页面，也可以直接用浏览器打开 [src/index.html](src/index.html)，编辑、预览、导入、网页转 MD（需配合代理）等功能均可使用；仅「最近打开、打开文件位置、原生保存对话框、PDF 直出」等桌面能力不可用，导出会自动回退为浏览器下载 / 打印。

---

## 快捷键

| 快捷键 | 功能 |
| --- | --- |
| `Ctrl + N` | 新建文档 |
| `Ctrl + O` | 打开文件 |
| `Ctrl + S` | 保存（已有路径时直接写回原文件） |
| `Ctrl + Shift + S` | 另存为 |
| `Ctrl + Z` | 撤销 |
| `Ctrl + Y` | 重做 |
| `Ctrl + B` | 加粗 |
| `Ctrl + I` | 斜体 |
| `Ctrl + U` | 下划线 |
| `Ctrl + K` | 插入链接 |
| `Ctrl + Shift + K` | 插入图片 |
| `Ctrl + F` | 查找 |
| `Ctrl + H` | 替换 |
| `F11` | 切换全屏 |
| `Tab` | 缩进 |

---

## 使用说明

### 菜单栏

- **文件**：新建、打开、导入、最近打开、保存、另存为、偏好设置、导出为、编码、打开文件位置、退出
- **编辑**：撤销、重做、查找、清空
- **插入**：代码块、链接、图片、表格、Mermaid、任务列表
- **格式**：加粗、斜体、下划线、删除线、上下标、H1–H6、引用、列表、行内代码
- **视图**：布局模式、全屏、大纲面板开关
- **工具**：网页转 MD
- **帮助**：使用帮助、关于

菜单栏最右侧为 **主题切换** 与 **界面语言** 图标。

### 偏好设置

「文件 → 偏好设置」中可配置：

- 图片处理方式（Base64 嵌入 / 保存到本地目录）及图片存放目录
- 界面语言
- 默认布局与默认编码
- 关闭窗口时是否提示保存未保存的更改

### PDF 导出说明

PDF 导出会将文档渲染为一份自包含 HTML（内联全部样式与 KaTeX 字体），再调用系统 Microsoft Edge 的无头模式 `--print-to-pdf` 直接生成 PDF 文件，全程不弹出打印对话框。若导出失败，请确认系统中已安装 Microsoft Edge。
