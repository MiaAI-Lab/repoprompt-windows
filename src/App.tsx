import { useState, useCallback, useEffect } from "react";
import type { FileEntry, AppSettings } from "./types";
import { getSettings, scanDirectory } from "./hooks/useTauri";
import FileTreePanel from "./components/FileTreePanel";
import ContextPanel from "./components/ContextPanel";
import OutputPanel from "./components/OutputPanel";
import SettingsPanel from "./components/SettingsPanel";
import AboutPanel from "./components/AboutPanel";

type Panel = "files" | "context" | "output" | "settings" | "about";

export default function App() {
  const [currentPanel, setCurrentPanel] = useState<Panel>("files");
  const [workspacePath, setWorkspacePath] = useState<string>("");
  const [files, setFiles] = useState<FileEntry[]>([]);
  const [selectedFiles, setSelectedFiles] = useState<Set<string>>(new Set());
  const [settings, setSettings] = useState<AppSettings | null>(null);
  const [contextOutput, setContextOutput] = useState<string>("");

  useEffect(() => {
    getSettings().then(setSettings).catch(console.error);
  }, []);

  const handleOpenWorkspace = useCallback(async (path: string) => {
    setWorkspacePath(path);
    try {
      const entries = await scanDirectory(path, settings?.show_hidden_files ?? false);
      setFiles(entries);
      setCurrentPanel("files");
    } catch (err) {
      console.error("Failed to scan directory:", err);
    }
  }, [settings]);

  const handleRefreshFiles = useCallback(async () => {
    if (!workspacePath) return;
    try {
      const entries = await scanDirectory(workspacePath, settings?.show_hidden_files ?? false);
      setFiles(entries);
    } catch (err) {
      console.error("Failed to refresh files:", err);
    }
  }, [workspacePath, settings]);

  const handleSelectedFilesChange = useCallback((selected: Set<string>) => {
    setSelectedFiles(selected);
  }, []);

  return (
    <div className="h-screen w-screen flex flex-col bg-[#0f0f0f] text-[#e0e0e0] overflow-hidden">
      <header className="flex items-center justify-between px-4 py-2 bg-[#1a1a1a] border-b border-[#333] select-none">
        <div className="flex items-center gap-3">
          <h1 className="text-sm font-semibold text-[#4f8cff]">RepoPrompt</h1>
          <span className="text-xs text-[#6b6b6b]">for Windows</span>
        </div>
        <div className="flex items-center gap-1">
          <span className="text-xs text-[#6b6b6b]">{workspacePath || "No workspace open"}</span>
        </div>
      </header>

      <nav className="flex bg-[#1a1a1a] border-b border-[#333] px-2">
        {([["files", "📁 Files"], ["context", "🔧 Context"], ["output", "📋 Output"], ["settings", "⚙️ Settings"], ["about", "ℹ️ About"]] as [Panel, string][]).map(([key, label]) => (
          <button key={key} onClick={() => setCurrentPanel(key)}
            className={`px-4 py-2 text-sm font-medium transition-colors ${
              currentPanel === key
                ? "text-[#4f8cff] border-b-2 border-[#4f8cff] bg-[#252525]"
                : "text-[#a0a0a0] hover:text-[#e0e0e0] hover:bg-[#252525]"
            }`}>{label}</button>
        ))}
      </nav>

      <main className="flex-1 overflow-hidden">
        {currentPanel === "files" && (
          <FileTreePanel
            workspacePath={workspacePath} files={files} selectedFiles={selectedFiles}
            onSelectedFilesChange={handleSelectedFilesChange}
            onOpenWorkspace={handleOpenWorkspace} onRefresh={handleRefreshFiles}
          />
        )}
        {currentPanel === "context" && (
          <ContextPanel
            workspacePath={workspacePath} selectedFiles={selectedFiles} files={files}
            settings={settings}
            onOutputGenerated={setContextOutput} onNavigateToOutput={() => setCurrentPanel("output")}
          />
        )}
        {currentPanel === "output" && (
          <OutputPanel output={contextOutput} onBack={() => setCurrentPanel("context")} />
        )}
        {currentPanel === "settings" && (
          <SettingsPanel settings={settings} onSettingsChange={setSettings} />
        )}
        {currentPanel === "about" && (
          <AboutPanel />
        )}
      </main>

      <footer className="flex items-center justify-between px-4 py-1 bg-[#1a1a1a] border-t border-[#333] text-xs text-[#6b6b6b]">
        <span>{files.length > 0
          ? `${files.filter((f) => !f.is_dir).length} files, ${files.filter((f) => f.is_dir).length} directories`
          : "No workspace loaded"}</span>
        <span>{selectedFiles.size > 0 && `${selectedFiles.size} files selected`}</span>
      </footer>
    </div>
  );
}
