import { useState, useMemo } from "react";
import { invoke } from "@tauri-apps/api/core";

interface Props {
  output: string;
  onBack: () => void;
}

export default function OutputPanel({ output, onBack }: Props) {
  const [copied, setCopied] = useState(false);

  const tokenEstimate = useMemo(() => {
    if (!output) return 0;
    return Math.ceil(output.length / 3.5);
  }, [output]);

  const lineCount = useMemo(() => {
    if (!output) return 0;
    return output.split("\n").length;
  }, [output]);

  const handleCopy = async () => {
    try {
      await navigator.clipboard.writeText(output);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    } catch {
      const { writeText } = await import("@tauri-apps/plugin-clipboard-manager");
      await writeText(output);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    }
  };

  const handleSave = async () => {
    try {
      const { save } = await import("@tauri-apps/plugin-dialog");
      const filePath = await save({
        defaultPath: "context.txt",
        filters: [
          { name: "Text", extensions: ["txt", "md", "xml"] },
          { name: "All Files", extensions: ["*"] },
        ],
      });
      if (filePath) {
        await invoke("save_file", { path: filePath, content: output });
      }
    } catch (err) {
      console.error("Failed to save:", err);
    }
  };

  if (!output) {
    return (
      <div className="h-full flex flex-col items-center justify-center text-[#6b6b6b] gap-4">
        <span className="text-4xl">📋</span>
        <p className="text-sm">No context generated yet</p>
        <p className="text-xs">Build context from the Context tab</p>
        <button onClick={onBack} className="px-4 py-2 text-sm bg-[#4f8cff] text-white rounded hover:bg-[#3a7bf5]">Go to Context Builder</button>
      </div>
    );
  }

  return (
    <div className="h-full flex flex-col">
      <div className="flex items-center gap-2 p-3 bg-[#1a1a1a] border-b border-[#333]">
        <button onClick={onBack} className="px-3 py-1.5 text-xs bg-[#252525] text-[#a0a0a0] rounded hover:bg-[#333]">← Back</button>
        <div className="flex items-center gap-1 text-xs text-[#6b6b6b]">
          <span>{lineCount} lines</span><span>·</span>
          <span>~{tokenEstimate.toLocaleString()} tokens</span><span>·</span>
          <span>{output.length.toLocaleString()} chars</span>
        </div>
        <div className="flex-1" />
        <button onClick={handleCopy} className="px-3 py-1.5 text-xs bg-[#4f8cff] text-white rounded hover:bg-[#3a7bf5]">{copied ? "✅ Copied!" : "📋 Copy"}</button>
        <button onClick={handleSave} className="px-3 py-1.5 text-xs bg-[#252525] text-[#a0a0a0] rounded hover:bg-[#333]">💾 Save</button>
      </div>
      <div className="flex-1 overflow-y-auto p-4">
        <pre className="text-xs font-mono leading-relaxed whitespace-pre-wrap"><code>{output}</code></pre>
      </div>
    </div>
  );
}


