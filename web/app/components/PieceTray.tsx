import { TrayPiece } from "../types/game";

export default function PieceTray({
  enabled,
  pieces,
  onPieceClick,
}: {
  enabled: boolean;
  pieces: TrayPiece[];
  onPieceClick: (pieceId: number) => void;
}) {
  return (
    <div className="grid grid-cols-2 grid-rows-8 gap-2 w-full h-full p-4">
      {pieces.map((piece, pieceId) => {
        const canInteract = enabled && piece.available;
        return (
          <div
            key={pieceId}
            onClick={() => canInteract && onPieceClick(pieceId)}
            className={`bg-gray-50 rounded flex items-center justify-center relative transition-all ${
              piece.border
                ? "border-4 border-gray-900"
                : "border-2 border-gray-300"
            } ${!piece.available ? "opacity-30" : "opacity-100"} ${
              canInteract
                ? "cursor-pointer hover:border-gray-500 hover:shadow-md"
                : "cursor-not-allowed"
            }`}
          >
            {/* Piece coordinate label */}
            <div className="absolute top-1 left-1 text-xs text-gray-400 font-mono">
              {pieceId}
            </div>

            <img
              src={`/pieces/piece${pieceId}.svg`}
              alt={`Piece ${pieceId}`}
              className="w-[85%] h-[85%] object-contain"
            />
          </div>
        );
      })}
    </div>
  );
}
