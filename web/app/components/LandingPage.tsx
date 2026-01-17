
export default function LandingPage({
  onSelectMode,
}: {
  onSelectMode: (perspective: "first" | "second") => void;
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
      <div className="absolute inset-0 flex flex-col items-center justify-center p-8 overflow-y-auto">
        <div className="max-w-lg border-2 border-gray-300 bg-white/95 p-6 rounded-lg shadow-lg">
          {/* Game Description */}
          <p className="text-gray-700 leading-relaxed">
            Quarto is played on a 4×4 board with 16 unique pieces. Each piece
            is tall or short, light or dark, square or circular, and hollow or
            solid. Players take turns choosing a piece which their opponent
            must place. Win by completing a row of four pieces that share any
            attribute.
          </p>

          {/* Divider */}
          <hr className="my-4 border-gray-200" />

          {/* Play Buttons */}
          <h3 className="text-lg font-semibold text-gray-900 mb-3 text-center">
            Play the Engine
          </h3>
          <div className="flex flex-col gap-2">
            <button
              onClick={() => onSelectMode("first")}
              className="w-full px-6 py-3 bg-gray-700 text-white rounded-lg font-medium hover:bg-gray-800 transition-colors"
            >
              Go First
            </button>
            <button
              onClick={() => onSelectMode("second")}
              className="w-full px-6 py-3 bg-gray-700 text-white rounded-lg font-medium hover:bg-gray-800 transition-colors"
            >
              Go Second
            </button>
          </div>
          <p className="text-xs text-gray-500 text-center mt-2">
            No signup required
          </p>

          {/* Divider */}
          <hr className="my-4 border-gray-200" />

          {/* Author Info */}
          <div className="text-center">
            <p className="text-sm text-gray-600 mb-2">
              Built by Jonathan Roth
            </p>
            <div className="flex justify-center gap-4 text-sm">
              <a
                href="https://github.com/JonathanRoth13/escapade"
                target="_blank"
                rel="noopener noreferrer"
                className="text-gray-700 hover:text-gray-900 underline"
              >
                Source Code
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
          </div>
        </div>
      </div>
    </div>
  );
}
