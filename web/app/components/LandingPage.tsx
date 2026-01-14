import Link from "next/link";

export default function LandingPage({
  onSelectMode,
  onOpenEditor,
}: {
  onSelectMode: (perspective: "first" | "second") => void;
  onOpenEditor: () => void;
}) {
  const boardState = Array(16).fill(null);

  return (
    <div className="w-full h-full relative">
      {/* Board Grid */}
      <div className="grid grid-cols-4 grid-rows-4 w-full h-full border-2 border-gray-300">
        {boardState.map((pieceId, index) => {
          return (
            <div key={index} className="bg-gray-50 border border-gray-300" />
          );
        })}
      </div>

      {/* Overlay Content */}
      <div className="absolute inset-0 flex flex-col items-center justify-center p-8 gap-8 overflow-y-auto">
        {/* Text Content */}
        <div className="text-center max-w-lg border-2 border-gray-300 bg-white/95 p-6 rounded-lg shadow-lg">
          <p className="text-xl text-gray-900 leading-relaxed mb-4">
            A Quarto engine with perfect play and real-time analysis. Built with
            Rust and Next.js.
          </p>
          <p className="text-base font-medium text-gray-900">Jonathan Roth</p>
          <p className="text-sm text-gray-600 mb-4">
            Full Stack Software Engineer
          </p>
          <div className="flex justify-center gap-4 text-sm mb-3">
            <a
              href="https://github.com/JonathanRoth13"
              target="_blank"
              rel="noopener noreferrer"
              className="text-gray-700 hover:text-gray-900 underline"
            >
              GitHub
            </a>
            <a
              href="https://www.linkedin.com/in/jonathanroth13/"
              target="_blank"
              rel="noopener noreferrer"
              className="text-gray-700 hover:text-gray-900 underline"
            >
              LinkedIn
            </a>
            <a
              href="mailto:JonathanRoth@protonmail.com"
              className="text-gray-700 hover:text-gray-900 underline"
            >
              Email
            </a>
          </div>
          {/* <Link
            href="/about"
            className="text-sm text-gray-700 hover:text-gray-900 underline"
          >
            How it works
          </Link> */}
        </div>

        {/* Options */}
        <div className="flex flex-col gap-6 w-full max-w-lg">
          {/* Play Mode Group */}
          <div className="border-2 border-gray-300 rounded-lg p-4 bg-white/95 shadow-lg">
            <h3 className="text-lg font-semibold text-gray-900 mb-3 text-center">
              Play Against Engine
            </h3>
            <div className="flex flex-col gap-2">
              <button
                onClick={() => onSelectMode("first")}
                className="w-full px-6 py-3 bg-gray-700 text-white rounded-lg font-medium hover:bg-gray-800 transition-colors"
              >
                Player Moves First
              </button>
              <button
                onClick={() => onSelectMode("second")}
                className="w-full px-6 py-3 bg-gray-700 text-white rounded-lg font-medium hover:bg-gray-800 transition-colors"
              >
                Player Moves Second
              </button>
            </div>
          </div>

          {/* Analysis Mode */}
          {/* <div className="border-2 border-gray-300 rounded-lg p-4 bg-white/95 shadow-lg">
            <h3 className="text-lg font-semibold text-gray-900 mb-3 text-center">
              Analyze a Position
            </h3>
            <button
              onClick={onOpenEditor}
              className="w-full px-6 py-3 bg-gray-700 text-white rounded-lg font-medium hover:bg-gray-800 transition-colors"
            >
              Open Board Editor
            </button>
          </div> */}
        </div>
      </div>
    </div>
  );
}
