import { useState, useMemo, useEffect, useCallback } from "react";
import type { FileEntry, AppSettings, ContextResult, OutputFormat, FileRangeRequest, LineRange } from "../types";
import { buildContext, getGitInfo, countTokensForFile } from "../hooks/useTauri";

interface Props {
  workspacePath: string;
  selectedFiles: Set<string>;
  files: FileEntry[];
  settings: AppSettings | null;
  onOutputGenerated: (output: string) => void;
  onNavigateToOutput: () => void;
}

export default function ContextPanel({
  workspacePath, selectedFiles: selectedRelativePaths, files,
  settings, onOutputGenerated, onNavigateToOutput,
}: Props) {
  const [tokenBudget, setTokenBudget] = useState(128000);
  const [userInstructions, setUserInstructions] = useState("");
  const [outputFormat, setOutputFormat] = useState<OutputFormat>("Xml");
  const [includeFileTree, setIncludeFileTree] = useState(true);
  const [includeGitDiff, setIncludeGitDiff] = useState(false);
  const [includeUserInstructions, setIncludeUserInstructions] = useState(true);
  const [isBuilding, setIsBuilding] = useState(false);
  const [result, setResult] = useState<ContextResult | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [isGitRepo, setIsGitRepo] = useState(false);
  const [fileTokenCounts, setFileTokenCounts] = useState<Map<string, number>>(new Map());
  const [lineRanges, setLineRanges] = useState<Map<string, string>>(new Map());
  const [tokenLoading, setTokenLoading] = useState(false);

  const selectedArray = useMemo(() => {
    if (selectedRelativePaths instanceof Set) {
      return files.filter((f) => selectedRelativePaths.has(f.relative_path) && !f.is_dir);
    }
    return [];
  }, [selectedRelativePaths, files]);

  useEffect(() => {
    if (!workspacePath) return;
    getGitInfo(workspacePath).then((info) => setIsGitRepo(info !== null)).catch(() => setIsGitRepo(false));
  }, [workspacePath]);

  // Count tokens for each selected file using the backend tiktoken
  useEffect(() => {
    if (selectedArray.length === 0) { setFileTokenCounts(new Map()); return; }
    let cancelled = false;
    setTokenLoading(true);
    (async () => {
      const counts = new Map<string, number>();
      for (let i = 0; i < selectedArray.length; i += 5) {
        if (cancelled) return;
        const batch = selectedArray.slice(i, i + 5);
        await Promise.all(batch.map(async (f) => {
          try {
            const [tokens] = await countTokensForFile(f.path);
            counts.set(f.relative_path, tokens);
          } catch { /* skip binary/unreadable */ }
        }));
      }
      if (!cancelled) {
        setFileTokenCounts(counts);
        setTokenLoading(false);
      }
    })();
    return () => { cancelled = true; };
  }, [selectedArray]);

  const totalTokens = useMemo(() => {
    let total = 0;
    for (const [_, count] of fileTokenCounts) total += count;
    return total;
  }, [fileTokenCounts]);

  const handleLineRangeChange = useCallback((relPath: string, value: string) => {
    setLineRanges((prev) => {
      const next = new Map(prev);
      if (value.trim()) next.set(relPath, value);
      else next.delete(relPath);
      return next;
    });
  }, []);

  const buildLineRanges = useCallback((): FileRangeRequest[] => {
    const requests: FileRangeRequest[] = [];
    for (const [filePath, rangeStr] of lineRanges) {
      const ranges: LineRange[] = [];
      const parts = rangeStr.split(",").map((s) => s.trim());
      for (const part of parts) {
        const match = part.match(/^(\d+)(?:-(\d+))?$/);
        if (match) {
          const start = parseInt(match[1]);
          const end = match[2] ? parseInt(match[2]) : start;
          ranges.push({ start, end, description: null });
        }
      }
      if (ranges.length > 0) requests.push({ file_path: filePath, ranges });
    }
    return requests;
  }, [lineRanges]);

  const handleBuild = async () => {
    if (!workspacePath) return;
    setIsBuilding(true);
    setError(null);
    try {
      const request = {
        roots: [workspacePath],
        ignore_patterns: settings?.custom_ignore_patterns || [],
        token_budget: tokenBudget,
        output_format: outputFormat,
        include_file_tree: includeFileTree,
        include_file_contents: true,
        include_git_diff: includeGitDiff,
        include_user_instructions: includeUserInstructions,
        user_instructions: userInstructions,
        selected_files: selectedArray.map((f) => f.relative_path),
        line_ranges: buildLineRanges(),
        model_name: null as string | null,
      };
      const ctxResult = await buildContext(request);
      setResult(ctxResult);
      onOutputGenerated(ctxResult.output);
    } catch (err: any) {
      setError(typeof err === "string" ? err : err?.message || "Build failed");
    } finally {
      setIsBuilding(false);
    }
  };

  const handleCopyToClipboard = async () => {
    if (!result?.output) return;
    try { await navigator.clipboard.writeText(result.output); }
    catch { const { writeText } = await import("@tauri-apps/plugin-clipboard-manager"); await writeText(result.output); }
  };

  return (
    <div className="h-full flex flex-col overflow-hidden">
      <div className="flex-1 overflow-y-auto p-4 space-y-4">
        {/* Selected files with token counts */}
        <section className="bg-[#1a1a1a] rounded-lg p-4 border border-[#333]">
          <h2 className="text-sm font-semibold mb-2">
            📄 Selected Files
            <span className="text-[#6b6b6b] ml-2 text-xs">({selectedArray.length} files)</span>
          </h2>
          {selectedArray.length === 0 ? (
            <p className="text-xs text-[#6b6b6b]">Select files from the Files tab first</p>
          ) : (
            <div className="max-h-60 overflow-y-auto space-y-2">
              {selectedArray.slice(0, 100).map((f) => (
                <div key={f.relative_path} className="text-xs">
                  <div className="flex justify-between items-center text-[#a0a0a0]">
                    <span className="truncate flex-1">{f.relative_path}</span>
                    <span className="ml-2 text-[#6b6b6b] whitespace-nowrap">
                      {fileTokenCounts.has(f.relative_path)
                        ? `${fileTokenCounts.get(f.relative_path)!.toLocaleString()} tok`
                        : fmtSize(f.size)}
                    </span>
                  </div>
                  {/* Line range input for each file */}
                  <input
                    type="text"
                    placeholder="Line ranges: 1-10,20-30 (leave empty for full file)"
                    value={lineRanges.get(f.relative_path) || ""}
                    onChange={(e) => handleLineRangeChange(f.relative_path, e.target.value)}
                    className="w-full mt-1 px-2 py-1 text-[10px] bg-[#252525] border border-[#333] rounded text-[#a0a0a0] placeholder-[#555] focus:outline-none focus:border-[#4f8cff]"
                  />
                </div>
              ))}
              {selectedArray.length > 100 && (
                <p className="text-xs text-[#6b6b6b]">...and {selectedArray.length - 100} more files</p>
              )}
            </div>
          )}
          {totalTokens > 0 && (
            <div className="mt-2 text-xs flex items-center gap-2">
              <span className="text-[#a0a0a0]">Total tokens: </span>
              <span className="text-[#4f8cff] font-mono">{totalTokens.toLocaleString()}</span>
              {tokenLoading && <span className="text-[#6b6b6b]">(counting...)</span>}
              {tokenBudget > 0 && (
                <span className={totalTokens > tokenBudget ? "text-[#ef4444]" : "text-[#22c55e]"}>
                  ({totalTokens > tokenBudget ? `⚠️ ${(totalTokens - tokenBudget).toLocaleString()} over budget` : "✅ Within budget"})
                </span>
              )}
            </div>
          )}
        </section>

        {/* Configuration */}
        <section className="bg-[#1a1a1a] rounded-lg p-4 border border-[#333] space-y-3">
          <h2 className="text-sm font-semibold">⚙️ Context Configuration</h2>
          <div>
            <label className="text-xs text-[#a0a0a0] block mb-1">Output Format</label>
            <div className="flex gap-2">
              {(["Xml", "Markdown", "Text"] as OutputFormat[]).map((fmt) => (
                <button key={fmt} onClick={() => setOutputFormat(fmt)}
                  className={`px-3 py-1 text-xs rounded ${outputFormat === fmt ? "bg-[#4f8cff] text-white" : "bg-[#252525] text-[#a0a0a0] hover:bg-[#333]"}`}>{fmt}</button>
              ))}
            </div>
          </div>
          <div>
            <label className="text-xs text-[#a0a0a0] block mb-1">
              Token Budget <span className="ml-2 text-[#4f8cff] font-mono">{tokenBudget.toLocaleString()}</span>
            </label>
            <input type="range" min={1000} max={1000000} step={1000} value={tokenBudget}
              onChange={(e) => setTokenBudget(Number(e.target.value))} className="w-full accent-[#4f8cff]" />
          </div>
          <div className="space-y-2">
            <label className="flex items-center gap-2 cursor-pointer">
              <input type="checkbox" checked={includeFileTree} onChange={(e) => setIncludeFileTree(e.target.checked)} className="accent-[#4f8cff]" />
              <span className="text-xs">Include file tree</span>
            </label>
            <label className="flex items-center gap-2 cursor-pointer">
              <input type="checkbox" checked={includeGitDiff} onChange={(e) => setIncludeGitDiff(e.target.checked)} className="accent-[#4f8cff]" disabled={!isGitRepo} />
              <span className="text-xs">Include Git diff</span>
              {!isGitRepo && <span className="text-[10px] text-[#6b6b6b]">(not a git repo)</span>}
            </label>
            <label className="flex items-center gap-2 cursor-pointer">
              <input type="checkbox" checked={includeUserInstructions} onChange={(e) => setIncludeUserInstructions(e.target.checked)} className="accent-[#4f8cff]" />
              <span className="text-xs">Include user instructions</span>
            </label>
          </div>
          {includeUserInstructions && (
            <textarea value={userInstructions} onChange={(e) => setUserInstructions(e.target.value)}
              placeholder="Enter instructions for the AI agent..."
              className="w-full h-24 px-3 py-2 text-xs bg-[#252525] border border-[#333] rounded text-[#e0e0e0] placeholder-[#6b6b6b] focus:outline-none focus:border-[#4f8cff] resize-none" />
          )}
        </section>

        <button onClick={handleBuild} disabled={isBuilding || !workspacePath || selectedArray.length === 0}
          className="w-full py-3 text-sm font-semibold bg-[#4f8cff] text-white rounded-lg hover:bg-[#3a7bf5] disabled:bg-[#333] disabled:text-[#6b6b6b] transition-colors">
          {isBuilding ? "🔄 Building Context..." : "🔨 Build Context"}
        </button>

        {error && <div className="p-3 bg-[#2a1a1a] border border-[#ef4444] rounded text-xs text-[#ef4444]">{error}</div>}
      </div>

      {result && (
        <div className="bg-[#1a1a1a] border-t border-[#333] p-3 flex items-center gap-3">
          <span className="text-xs text-[#22c55e]">✅ Context built!</span>
          <span className="text-xs text-[#a0a0a0]">{result.token_count.toLocaleString()} tokens · {result.included_files.length} files</span>
          {result.budget_truncated && <span className="text-xs text-[#f59e0b]">⚠️ Truncated by budget</span>}
          <div className="flex-1" />
          <button onClick={handleCopyToClipboard} className="px-3 py-1 text-xs bg-[#252525] text-[#a0a0a0] rounded hover:bg-[#333]">📋 Copy</button>
          <button onClick={onNavigateToOutput} className="px-3 py-1 text-xs bg-[#4f8cff] text-white rounded hover:bg-[#3a7bf5]">👁️ View Output</button>
        </div>
      )}
    </div>
  );
}

function fmtSize(bytes: number): string {
  if (bytes === 0) return "0 B";
  const u = ["B", "KB", "MB", "GB"];
  const i = Math.floor(Math.log(bytes) / Math.log(1024));
  return `${(bytes / Math.pow(1024, i)).toFixed(1)} ${u[i]}`;
}
