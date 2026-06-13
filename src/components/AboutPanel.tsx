export default function AboutPanel() {
  return (
    <div className="h-full flex flex-col items-center justify-center p-8">
      <div className="max-w-md w-full bg-[#1a1a1a] rounded-xl border border-[#333] p-8 text-center space-y-6">
        {/* Logo */}
        <div className="text-5xl mb-2">🔄</div>

        {/* App name */}
        <div>
          <h1 className="text-xl font-bold text-white">RepoPrompt</h1>
          <p className="text-xs text-[#6b6b6b] mt-1">for Windows</p>
        </div>

        {/* Version */}
        <div className="bg-[#252525] rounded-lg px-4 py-2 inline-block">
          <span className="text-sm text-[#a0a0a0]">Version </span>
          <span className="text-sm font-semibold text-[#4f8cff]">1.02</span>
        </div>

        {/* Divider */}
        <div className="border-t border-[#333]"></div>

        {/* Author */}
        <div>
          <p className="text-xs text-[#6b6b6b] mb-1">Created by</p>
          <p className="text-sm font-medium text-white">Mia's AI Lab</p>
        </div>

        {/* Divider */}
        <div className="border-t border-[#333]"></div>

        {/* Links */}
        <div className="space-y-3">
          <a href="https://github.com/MiaAI-Lab/"
             target="_blank"
             rel="noopener noreferrer"
             className="flex items-center justify-center gap-2 px-4 py-2.5 bg-[#252525] hover:bg-[#2a2a2a] rounded-lg text-sm text-[#a0a0a0] hover:text-white transition-colors">
            <span>🐙</span>
            <span>github.com/MiaAI-Lab</span>
          </a>
          <a href="https://x.com/MiaAI_lab"
             target="_blank"
             rel="noopener noreferrer"
             className="flex items-center justify-center gap-2 px-4 py-2.5 bg-[#252525] hover:bg-[#2a2a2a] rounded-lg text-sm text-[#a0a0a0] hover:text-white transition-colors">
            <span>𝕏</span>
            <span>@MiaAI_lab</span>
          </a>
        </div>

        {/* Footer */}
        <p className="text-[10px] text-[#444] pt-2">
          A context engineering tool for AI coding agents
        </p>
      </div>
    </div>
  );
}


