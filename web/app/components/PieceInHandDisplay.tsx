export default function PieceInHandDisplay({ pieceId }: { pieceId: number }) {
  // Size of one board cell = board size / 4
  const cellSize = "calc(min(50vw, calc(100vh - 7rem - 3vw)) / 4)";

  return (
    <div className="flex-1 flex items-center justify-center">
      <div
        className="bg-gray-50 border border-gray-300 flex items-center justify-center relative"
        style={{
          width: cellSize,
          height: cellSize,
        }}
      >
        {/* Piece coordinate label */}
        <div className="absolute top-1 left-1 text-xs text-gray-400 font-mono">
          {pieceId}
        </div>

        <img
          src={`/pieces/piece${pieceId}.svg`}
          alt={`Piece ${pieceId}`}
          className="w-[90%] h-[90%] object-contain"
        />
      </div>
    </div>
  );
}
