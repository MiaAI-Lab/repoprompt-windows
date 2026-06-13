import { useState, useMemo, useCallback } from "react";
import type { FileEntry, TreeNode } from "../types";
import { open } from "@tauri-apps/plugin-dialog";

interface Props {
  workspacePath: string;
  files: FileEntry[];
  selectedFiles: Set<string>;
  onSelectedFilesChange: (selected: Set<string>) => void;
  onOpenWorkspace: (path: string) => void;
  onRefresh: () => void;
}

function buildTree(files: FileEntry[]): TreeNode[] {
  const root: TreeNode[] = [];
  const map = new Map<string, TreeNode>();
  const sorted = [...files].sort((a, b) => {
    if (a.is_dir !== b.is_dir) return a.is_dir ? -1 : 1;
    return a.relative_path.localeCompare(b.relative_path);
  });
  for (const file of sorted) {
    const parts = file.relative_path.split("/");
    const node: TreeNode = { ...file, children: [], checked: false, indeterminate: false, depth: parts.length - 1 };
    map.set(file.relative_path, node);
    if (parts.length > 1) {
      const parentPath = parts.slice(0, -1).join("/");
      const parent = map.get(parentPath);
      if (parent) {
        parent.children.push(node);
      } else {
        const parentNode: TreeNode = {
          path: file.path.replace(file.name, ""),
          relative_path: parentPath,
          name: parts[parts.length - 2],
          size: 0, is_dir: true, extension: "",
          children: [node], checked: false, indeterminate: false, depth: parts.length - 2,
        };
        map.set(parentPath, parentNode);
        root.push(parentNode);
      }
    } else {
      root.push(node);
    }
  }
  return root;
}

export default function FileTreePanel({
  workspacePath, files, selectedFiles, onSelectedFilesChange, onOpenWorkspace, onRefresh,
}: Props) {
  const [searchQuery, setSearchQuery] = useState("");
  const [expandedDirs, setExpandedDirs] = useState<Set<string>>(new Set());

  const tree = useMemo(() => buildTree(files), [files]);

  const filteredFiles = useMemo(() => {
    if (!searchQuery.trim()) return files;
    const q = searchQuery.toLowerCase();
    return files.filter((f) => f.name.toLowerCase().includes(q) || f.relative_path.toLowerCase().includes(q));
  }, [files, searchQuery]);

  const handleOpenFolder = useCallback(async () => {
    const selected = await open({ directory: true, multiple: false, title: "Open Workspace Folder" });
    if (selected && typeof selected === "string") onOpenWorkspace(selected);
  }, [onOpenWorkspace]);

  const toggleFile = useCallback((relativePath: string) => {
    const next = new Set(selectedFiles);
    if (next.has(relativePath)) next.delete(relativePath); else next.add(relativePath);
    onSelectedFilesChange(next);
  }, [selectedFiles, onSelectedFilesChange]);

  const toggleDir = useCallback((_dirPath: string, dirFiles: FileEntry[]) => {
    const next = new Set(selectedFiles);
    const allSelected = dirFiles.every((f) => next.has(f.relative_path));
    for (const f of dirFiles) {
      if (allSelected) next.delete(f.relative_path); else next.add(f.relative_path);
    }
    onSelectedFilesChange(next);
  }, [selectedFiles, onSelectedFilesChange]);

  const toggleExpand = useCallback((relativePath: string) => {
    setExpandedDirs((prev) => {
      const next = new Set(prev);
      if (next.has(relativePath)) next.delete(relativePath); else next.add(relativePath);
      return next;
    });
  }, []);

  const selectAll = useCallback(() => {
    onSelectedFilesChange(new Set(files.filter((f) => !f.is_dir).map((f) => f.relative_path)));
  }, [files, onSelectedFilesChange]);

  const deselectAll = useCallback(() => {
    onSelectedFilesChange(new Set());
  }, [onSelectedFilesChange]);

  const renderTree = (nodes: TreeNode[], depth: number = 0): React.ReactNode[] => {
    return nodes.flatMap((node) => {
      const hasChildren = node.children.length > 0;
      const isExpanded = expandedDirs.has(node.relative_path);
      const allLeafFiles = node.is_dir
        ? files.filter((f) => f.relative_path.startsWith(node.relative_path) && !f.is_dir)
        : [node];
      const allSelected = allLeafFiles.length > 0 && allLeafFiles.every((f) => selectedFiles.has(f.relative_path));
      const someSelected = allLeafFiles.some((f) => selectedFiles.has(f.relative_path));

      const items: React.ReactNode[] = [];
      items.push(
        <div key={node.relative_path}
          className="flex items-center gap-1.5 px-2 py-1 hover:bg-[#2a2a2a] cursor-pointer rounded group"
          style={{ paddingLeft: `${12 + depth * 16}px` }}
        >
          {hasChildren ? (
            <button onClick={() => toggleExpand(node.relative_path)}
              className="w-4 text-center text-xs text-[#6b6b6b] hover:text-[#e0e0e0]">{isExpanded ? "▼" : "▶"}</button>
          ) : <span className="w-4" />}
          <input type="checkbox" checked={allSelected}
            ref={(el) => { if (el) el.indeterminate = someSelected && !allSelected; }}
            onChange={() => { if (node.is_dir) toggleDir(node.relative_path, allLeafFiles); else toggleFile(node.relative_path); }}
            className="accent-[#4f8cff]" />
          <span className="text-sm">{node.is_dir ? "📁" : "📄"} {node.name}</span>
          {!node.is_dir && <span className="ml-auto text-xs text-[#6b6b6b]">{fmtSize(node.size)}</span>}
        </div>
      );
      if (hasChildren && isExpanded) items.push(...renderTree(node.children, depth + 1));
      return items;
    });
  };

  return (
    <div className="h-full flex flex-col">
      <div className="flex items-center gap-2 p-3 bg-[#1a1a1a] border-b border-[#333]">
        <button onClick={handleOpenFolder}
          className="px-3 py-1.5 text-xs bg-[#4f8cff] text-white rounded hover:bg-[#3a7bf5] transition-colors">📂 Open Folder</button>
        {workspacePath && (
          <>
            <button onClick={onRefresh}
              className="px-3 py-1.5 text-xs bg-[#252525] text-[#a0a0a0] rounded hover:bg-[#333] transition-colors">🔄 Refresh</button>
            <div className="flex-1" />
            <button onClick={selectAll} className="px-2 py-1 text-xs text-[#4f8cff] hover:underline">Select All</button>
            <button onClick={deselectAll} className="px-2 py-1 text-xs text-[#a0a0a0] hover:underline">Deselect</button>
          </>
        )}
      </div>
      <div className="p-2 border-b border-[#333]">
        <input type="text" placeholder="Search files..." value={searchQuery}
          onChange={(e) => setSearchQuery(e.target.value)}
          className="w-full px-3 py-1.5 text-sm bg-[#252525] border border-[#333] rounded text-[#e0e0e0] placeholder-[#6b6b6b] focus:outline-none focus:border-[#4f8cff]" />
      </div>
      <div className="flex-1 overflow-y-auto">
        {!workspacePath ? (
          <div className="flex flex-col items-center justify-center h-full text-[#6b6b6b] gap-4">
            <span className="text-4xl">📂</span>
            <p className="text-sm">Open a folder to browse files</p>
            <button onClick={handleOpenFolder}
              className="px-4 py-2 text-sm bg-[#4f8cff] text-white rounded hover:bg-[#3a7bf5]">Open Folder</button>
          </div>
        ) : searchQuery.trim() ? (
          filteredFiles.filter((f) => !f.is_dir).map((file) => (
            <div key={file.relative_path} className="flex items-center gap-2 px-3 py-1.5 hover:bg-[#2a2a2a] cursor-pointer">
              <input type="checkbox" checked={selectedFiles.has(file.relative_path)}
                onChange={() => toggleFile(file.relative_path)} className="accent-[#4f8cff]" />
              <span className="text-sm">📄 {file.relative_path}</span>
              <span className="ml-auto text-xs text-[#6b6b6b]">{fmtSize(file.size)}</span>
            </div>
          ))
        ) : (
          renderTree(tree)
        )}
      </div>
    </div>
  );
}

function fmtSize(bytes: number): string {
  if (bytes === 0) return "0 B";
  const u = ["B", "KB", "MB", "GB"];
  const i = Math.floor(Math.log(bytes) / Math.log(1024));
  return `${(bytes / Math.pow(1024, i)).toFixed(1)} ${u[i]}`;
}
