import { Cell } from "../types/game";

export default function GameBoard({
  cells,
  enableHover,
  loading,
  onCellClick,
}: {
  cells: Cell[];
  enableHover: boolean;
  loading: boolean;
  onCellClick: (square: number) => void;
}) {
  // Render rows in reverse order: row 3 (indices 12-15) at top, row 0 (indices 0-3) at bottom
  const rows = [
    [12, 13, 14, 15], // Row 3 (top)
    [8, 9, 10, 11], // Row 2
    [4, 5, 6, 7], // Row 1
    [0, 1, 2, 3], // Row 0 (bottom)
  ];

  return (
    <div className="relative w-full h-full">
      <div className="grid grid-cols-4 grid-rows-4 w-full h-full border-2 border-gray-300">
        {rows.flat().map((index) => {
          const cell = cells[index];
          return (
            <div
              key={index}
              onClick={() => enableHover && onCellClick(index)}
              className={`bg-gray-50 flex items-center justify-center relative overflow-hidden ${
                cell.highlighted
                  ? "border-4 border-gray-900"
                  : "border border-gray-300"
              } ${
                enableHover
                  ? "hover:bg-gray-100 transition-colors cursor-pointer"
                  : ""
              }`}
            >
              {/* Square coordinate label */}
              <div className="absolute top-1 left-1 text-xs text-gray-400 font-mono">
                {index}
              </div>

              {cell.piece != null ? (
                <img
                  src={`/pieces/piece${cell.piece}.svg`}
                  alt=""
                  className="w-[90%] h-[90%] object-contain"
                />
              ) : null}
            </div>
          );
        })}
      </div>

      {/* Loading Overlay */}
      {loading && (
        <div className="absolute inset-0 bg-white/70 flex items-center justify-center">
          <div className="w-16 h-16 border-4 border-gray-300 border-t-gray-700 rounded-full animate-spin"></div>
        </div>
      )}
    </div>
  );
}
