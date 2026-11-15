"use client";

import { useState } from "react";
import Board from "./components/board/Board";
import type { BoardPieces } from "./components/board/types";

// Placeholder until the play screen lands: click empty squares to place
// pieces 0–15 in turn.
export default function Home() {
  const [pieces, setPieces] = useState<BoardPieces>(Array(16).fill(null));
  const [lastMove, setLastMove] = useState<number | null>(null);
  const placed = pieces.filter((p) => p !== null).length;

  const place = (square: number) => {
    const next = [...pieces];
    next[square] = placed;
    setPieces(next);
    setLastMove(square);
  };

  return (
    <main className="flex min-h-screen flex-col items-center justify-center gap-4">
      <h1 className="text-4xl font-bold">ESCAPADE</h1>
      <div className="w-[min(480px,90vw)]">
        <Board
          pieces={pieces}
          marks={lastMove === null ? {} : { [lastMove]: ["last-move"] }}
          live={placed < 16}
          actor={placed < 16 ? "you" : null}
          onSquareClick={place}
        />
      </div>
    </main>
  );
}
