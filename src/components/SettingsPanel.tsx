import { useState } from "react";
import type { AppSettings } from "../types";
import { updateSettings } from "../hooks/useTauri";

interface Props {
  settings: AppSettings | null;
  onSettingsChange: (settings: AppSettings) => void;
}

export default function SettingsPanel({ settings, onSettingsChange }: Props) {
  const [localSettings, setLocalSettings] = useState<AppSettings | null>(settings);
  const [saving, setSaving] = useState(false);
  const [saved, setSaved] = useState(false);
  const [newPattern, setNewPattern] = useState("");

  if (!localSettings) return null;

  const handleSave = async () => {
    if (!localSettings) return;
    setSaving(true);
    try {
      await updateSettings(localSettings);
      onSettingsChange(localSettings);
      setSaved(true);
      setTimeout(() => setSaved(false), 2000);
    } catch (err) {
      console.error("Failed to save settings:", err);
    } finally {
      setSaving(false);
    }
  };

  const addPattern = () => {
    if (!newPattern.trim() || !localSettings) return;
    const updated = {
      ...localSettings,
      custom_ignore_patterns: [...localSettings.custom_ignore_patterns, newPattern.trim()],
    };
    setLocalSettings(updated);
    setNewPattern("");
  };

  const removePattern = (index: number) => {
    if (!localSettings) return;
    const updated = {
      ...localSettings,
      custom_ignore_patterns: localSettings.custom_ignore_patterns.filter((_, i) => i !== index),
    };
    setLocalSettings(updated);
  };

  return (
    <div className="h-full overflow-y-auto p-4 space-y-4">
      {/* Ignore Patterns */}
      <section className="bg-[#1a1a1a] rounded-lg p-4 border border-[#333] space-y-3">
        <h2 className="text-sm font-semibold">🚫 Ignore Patterns</h2>
        <p className="text-[10px] text-[#6b6b6b]">
          Patterns added here are applied in addition to .gitignore rules.
          Uses gitignore-style pattern syntax.
        </p>

        <div className="flex gap-2">
          <input type="text" value={newPattern} onChange={(e) => setNewPattern(e.target.value)}
            onKeyDown={(e) => e.key === "Enter" && addPattern()}
            placeholder="e.g., *.log, tmp/, build/"
            className="flex-1 px-3 py-1.5 text-xs bg-[#252525] border border-[#333] rounded text-[#e0e0e0] placeholder-[#6b6b6b] focus:outline-none focus:border-[#4f8cff]" />
          <button onClick={addPattern}
            className="px-3 py-1.5 text-xs bg-[#4f8cff] text-white rounded hover:bg-[#3a7bf5]">Add</button>
        </div>

        <div className="space-y-1 max-h-40 overflow-y-auto">
          {localSettings.custom_ignore_patterns.map((pattern, i) => (
            <div key={i} className="flex items-center gap-2 px-2 py-1 bg-[#252525] rounded">
              <span className="text-xs font-mono text-[#a0a0a0] flex-1">{pattern}</span>
              <button onClick={() => removePattern(i)} className="text-[#ef4444] text-xs hover:underline">✕</button>
            </div>
          ))}
        </div>
      </section>

      {/* General Settings */}
      <section className="bg-[#1a1a1a] rounded-lg p-4 border border-[#333] space-y-3">
        <h2 className="text-sm font-semibold">🔧 General</h2>

        <div className="space-y-2">
          <label className="flex items-center gap-2 cursor-pointer">
            <input type="checkbox" checked={localSettings.show_hidden_files}
              onChange={(e) => setLocalSettings({ ...localSettings, show_hidden_files: e.target.checked })}
              className="accent-[#4f8cff]" />
            <span className="text-xs">Show hidden files (starting with .)</span>
          </label>
          <label className="flex items-center gap-2 cursor-pointer">
            <input type="checkbox" checked={localSettings.include_file_tree}
              onChange={(e) => setLocalSettings({ ...localSettings, include_file_tree: e.target.checked })}
              className="accent-[#4f8cff]" />
            <span className="text-xs">Include file tree by default</span>
          </label>
          <label className="flex items-center gap-2 cursor-pointer">
            <input type="checkbox" checked={localSettings.include_git_diff}
              onChange={(e) => setLocalSettings({ ...localSettings, include_git_diff: e.target.checked })}
              className="accent-[#4f8cff]" />
            <span className="text-xs">Include git diff by default</span>
          </label>
        </div>

        <div>
          <label className="text-xs text-[#a0a0a0] block mb-1">Default Output Format</label>
          <select value={localSettings.output_format}
            onChange={(e) => setLocalSettings({ ...localSettings, output_format: e.target.value })}
            className="px-3 py-1.5 text-xs bg-[#252525] border border-[#333] rounded text-[#e0e0e0] focus:outline-none focus:border-[#4f8cff]">
            <option value="Xml">XML (best for LLMs)</option>
            <option value="Markdown">Markdown</option>
            <option value="Text">Plain Text</option>
          </select>
        </div>

        <div>
          <label className="text-xs text-[#a0a0a0] block mb-1">
            Default Token Budget
            <span className="ml-2 text-[#4f8cff] font-mono">{localSettings.default_token_budget.toLocaleString()}</span>
          </label>
          <input type="range" min={1000} max={1000000} step={1000}
            value={localSettings.default_token_budget}
            onChange={(e) => setLocalSettings({ ...localSettings, default_token_budget: Number(e.target.value) })}
            className="w-full accent-[#4f8cff]" />
        </div>

        <div>
          <label className="text-xs text-[#a0a0a0] block mb-1">Max File Size (MB)</label>
          <input type="number"
            value={Math.round(localSettings.max_file_size / (1024 * 1024))}
            onChange={(e) => setLocalSettings({ ...localSettings, max_file_size: Number(e.target.value) * 1024 * 1024 })}
            min={1} max={100}
            className="w-24 px-2 py-1 text-xs bg-[#252525] border border-[#333] rounded text-[#e0e0e0] focus:outline-none focus:border-[#4f8cff]" />
        </div>
      </section>

      {/* Save button */}
      <button onClick={handleSave} disabled={saving}
        className="w-full py-2 text-sm font-semibold bg-[#4f8cff] text-white rounded-lg hover:bg-[#3a7bf5] disabled:bg-[#333] disabled:text-[#6b6b6b]">
        {saving ? "Saving..." : saved ? "✅ Saved!" : "💾 Save Settings"}
      </button>
    </div>
  );
}
