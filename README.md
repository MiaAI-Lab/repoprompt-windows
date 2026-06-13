<div align="center">

  # 🔄 RepoPrompt for Windows

  **A context engineering tool for AI coding agents.**
  <br />
  Curate your code. Control your context. Build better prompts.

  <br />

  <a href="https://github.com/MiaAI-Lab/reprompt-windows/releases"><img src="https://img.shields.io/github/v/release/MiaAI-Lab/reprompt-windows?style=flat-square&label=Release" alt="Release" /></a>
  <a href="https://github.com/MiaAI-Lab/reprompt-windows/blob/main/LICENSE"><img src="https://img.shields.io/github/license/MiaAI-Lab/reprompt-windows?style=flat-square&label=License" alt="License" /></a>
  <img src="https://img.shields.io/badge/Windows-10%2B%20%7C%2011-0078D6?style=flat-square&logo=windows&logoColor=white" alt="Windows" />
  <img src="https://img.shields.io/badge/Installer-~8%20MB-2ea44f?style=flat-square" alt="Size" />
  <img src="https://img.shields.io/badge/Built%20With-Tauri%202%20%7C%20Rust%20%7C%20React-ff69b4?style=flat-square" alt="Built With" />

  <br /><br />

  ![RepoPrompt Screenshot](https://github.com/MiaAI-Lab/reprompt-windows/releases/download/v1.0.0/screenshot.png)
  ![RepoPrompt Screenshot 2](https://github.com/MiaAI-Lab/reprompt-windows/releases/download/v1.0.0/screenshot2.png)

  <br />

  <a href="#installation">📥 Download</a> ·
  <a href="#features">✨ Features</a> ·
  <a href="#workflow">💡 Workflow</a> ·
  <a href="https://github.com/MiaAI-Lab/reprompt-windows/issues">🐛 Report Bug</a>

</div>

---

## What is RepoPrompt?

RepoPrompt is a **context engineering tool** for AI coding agents. It solves a simple but critical problem:

> When you paste a codebase into ChatGPT, Claude, Gemini, or any LLM — you are paying by the token. Every irrelevant file is wasted context and wasted money. RepoPrompt lets you surgically select which files to include, slice them to specific line ranges, track your token budget, and generate clean, LLM-optimized XML output.

Think of it as a **curated context window builder** for your codebase.

---

## Why RepoPrompt?

| | RepoPrompt | Raw copy/paste |
|---|---|---|
| **File tree + structure** | ✅ Automatic file map included | ❌ Manual |
| **Token counting** | ✅ Per-file and total with budget warnings | ❌ Blind guessing |
| **Line-range slicing** | ✅ Include only relevant sections | ❌ Whole file or nothing |
| **.gitignore-aware** | ✅ Respects existing ignore rules | ❌ Risk of leaking secrets/configs |
| **Git diff support** | ✅ Include staged/unstaged changes | ❌ Manual git diff |
| **Output format** | ✅ XML/Markdown/Text — optimized for LLMs | ❌ Raw, unstructured |
| **MCP server** | ✅ External agents can query your workspace | ❌ Not possible |
| **Copy to clipboard** | ✅ One-click | ❌ Manual select + copy |

---

## ✨ Features

- **📁 Multi-root workspace** — Open one or more folders. Browse the full file tree.
- **🔍 File browser with search** — Filter files by name, extension, or path. Checkboxes for selection.
- **💰 Token budget management** — See token counts per file and total. Set a budget slider — get warned when you exceed it.
- **✂️ Line-range slicing** — Don't need the whole file? Select specific line ranges to include only what matters.
- **🚫 Smart ignore** — Respects `.gitignore`, `.repoprompt-ignore`, and custom ignore patterns. Automatically skips binaries, images, and common noise.
- **📋 LLM-optimized output** — Generates clean XML (Repomix-style `<context>` format), Markdown, or plain text. One-click copy to clipboard or save as `.txt`/`.md`/`.xml`.
- **🔄 Git integration** — Detects git repos, shows branch info, and can include staged/unstaged diffs in your context.
- **🔌 Basic MCP server** — Expose your workspace context to external AI agents via a JSON-RPC TCP server.
- **⚙️ Customizable** — Default ignore patterns, hidden file visibility, max file size, output format preferences.

---

## 💡 Workflow

```
1. Open a project folder
   ↓
2. Browse the file tree — search, filter, select
   ↓
3. (Optional) Set line ranges on specific files
   ↓
4. Set your token budget (slider)
   ↓
5. Hit "Build Context"
   ↓
6. Copy to clipboard or save as .xml/.md/.txt
   ↓
7. Paste into ChatGPT, Claude, Gemini, Cursor, or any AI tool
```

**That's it.** No accounts. No cloud. No telemetry. Just a curated context window for your AI pair programmer.

---

## Who is this for?

- **Developers using AI coding agents** — ChatGPT, Claude, Gemini, Cursor, Copilot, Codex CLI, etc.
- **Anyone who pastes code into LLMs** and wants to control token usage
- **Teams sharing code context** — generate standardized context files for reproducible AI interactions
- **Agent orchestration** — when you want an external agent (via MCP) to understand your full project structure

---

## Installation

Download the latest installer from the [Releases page](https://github.com/MiaAI-Lab/reprompt-windows/releases):

[![Download MSI](https://img.shields.io/badge/Download-MSI-0078D6?style=for-the-badge&logo=windows&logoColor=white)](https://github.com/MiaAI-Lab/reprompt-windows/releases/download/v1.0.0/RepoPrompt_1.0.0_x64_en-US.msi)
[![Download Setup EXE](https://img.shields.io/badge/Download-Setup_EXE-0078D6?style=for-the-badge&logo=windows&logoColor=white)](https://github.com/MiaAI-Lab/reprompt-windows/releases/download/v1.0.0/RepoPrompt_1.0.0_x64-setup.exe)

- **MSI** — Windows Installer package. Supports silent/unattended installation via msiexec.
- **Setup EXE** — NSIS-based installer with a graphical setup wizard.

### Requirements

- Windows 10 or 11
- [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/) (included with Windows 10/11; install the Evergreen Runtime if missing)

---

## Development Setup

### Prerequisites

- [Node.js](https://nodejs.org/) >= 18
- [npm](https://www.npmjs.com/) or [pnpm](https://pnpm.io/)
- [Rust](https://rustup.rs/) stable toolchain (install via rustup)
- [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/) (included with Windows 10/11)
- [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) with the "Desktop development with C++" workload

### Get started

```powershell
# Clone the repository
git clone https://github.com/MiaAI-Lab/reprompt-windows.git
cd reprompt-windows

# Install frontend dependencies
npm install

# Run in development mode (hot-reload)
npm run tauri dev
```

### Build production installers

```powershell
npm run tauri build
```

Output installers are written to:
- `src-tauri/target/release/bundle/msi/RepoPrompt_{version}_x64_en-US.msi`
- `src-tauri/target/release/bundle/nsis/RepoPrompt_{version}_x64-setup.exe`

### Run tests

```powershell
cd src-tauri && cargo test
```

---

## Project Layout

```
reprompt-windows/
├── src/                          React + TypeScript frontend
│   ├── App.tsx                    Main layout (5-tab navigation)
│   ├── main.tsx                   Entry point
│   ├── types.ts                   TypeScript type definitions
│   ├── index.css                  Global styles + Tailwind v4
│   ├── components/
│   │   ├── FileTreePanel.tsx      File tree browser with search/filter
│   │   ├── ContextPanel.tsx       Context builder (config + build)
│   │   ├── OutputPanel.tsx        View/copy/save generated context
│   │   ├── SettingsPanel.tsx      Ignore patterns, defaults, preferences
│   │   └── AboutPanel.tsx         Version, author, links
│   └── hooks/
│       └── useTauri.ts            Tauri invoke wrappers
├── src-tauri/                    Rust backend (Tauri 2)
│   ├── src/
│   │   ├── lib.rs                22 Tauri commands (bridge)
│   │   ├── main.rs               Entry point
│   │   └── engine/
│   │       ├── context/          ContextBuilder — core orchestrator
│   │       ├── filesystem/       Scanner, ignore rules, content reader
│   │       ├── tokenizer/        tiktoken-rs with 3 encoding types
│   │       ├── git/              git2 + CLI fallback
│   │       ├── settings/         AppSettings struct
│   │       └── mcp/              MCP tool definitions + TCP transport
│   ├── tauri.conf.json           App config (window, bundle, plugins)
│   ├── capabilities/             Permission scopes
│   ├── icons/                    App icons (all sizes)
│   ├── Cargo.toml                Rust dependencies
│   └── build.rs                  Tauri build script
├── package.json                  Node dependencies and scripts
├── vite.config.ts                Vite configuration
└── LICENSE                       Apache 2.0
```

---

## Security & Privacy

**RepoPrompt is a local-first context tool.** Period.

- ✅ **No cloud** — All files are read from and written to your local filesystem.
- ✅ **No telemetry** — Zero data collection. No analytics. No pings.
- ✅ **No accounts** — No sign-up, no login, no cloud sync.
- ✅ **No network requests** — The app never makes an external network call.
- ✅ **Local storage** — Settings live entirely in `%APPDATA%\com.miaailab.reprompt\`.
- ✅ **Safe file handling** — Binary files are detected and skipped. `.gitignore` is respected by default.

---

## 🙏 Acknowledgments

This project is a **Windows port** and **evolution** of [RepoPrompt CE](https://github.com/repoprompt/repoprompt-ce) by [@r3m00n](https://github.com/r3m00n). The original macOS app pioneered the context engineering workflow, and this port brings that same philosophy to Windows with a full stack rewrite in Tauri 2 + Rust + React.

Huge thanks to the original author for the inspiration and the excellent design patterns.

---

## 🤝 Contributing

Found a bug? Have an idea? Open an [issue](https://github.com/MiaAI-Lab/reprompt-windows/issues) or send a PR.

---

## 📄 License

Apache 2.0 — see [LICENSE](LICENSE).

<div align="center">
  <br />
  <sub>Built with ❤️ by <a href="https://github.com/MiaAI-Lab">Mia's AI Lab</a> using <a href="https://v2.tauri.app/">Tauri 2</a>, <a href="https://react.dev/">React</a>, <a href="https://tailwindcss.com/">Tailwind CSS</a>, and <a href="https://www.rust-lang.org/">Rust</a>.</sub>
  <br />
  <sub>Inspired by <a href="https://github.com/repoprompt/repoprompt-ce">RepoPrompt CE</a> by <a href="https://github.com/r3m00n">@r3m00n</a>.</sub>
  <br /><br />
  <a href="https://github.com/MiaAI-Lab"><img src="https://img.shields.io/badge/Mia's_AI_Lab-555?style=flat-square&logo=github&logoColor=white" alt="Mia's AI Lab" /></a>
  <a href="https://x.com/MiaAI_lab"><img src="https://img.shields.io/badge/@MiaAI_lab-000?style=flat-square&logo=x&logoColor=white" alt="X / Twitter" /></a>
</div>
