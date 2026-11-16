"use client";

import { useState } from "react";
import Board from "./components/board/Board";
import type { BoardPieces } from "./components/board/types";
import HandZone from "./components/hand/HandZone";
import PhaseIndicator from "./components/phase/PhaseIndicator";
import Tray from "./components/tray/Tray";
import {
  BOARD_MARK,
  PHASE,
  PHASE_SIDE,
  SIDE,
  TRAY_MARK,
  type Phase,
} from "./constants";

export default function Home() {
  const [pieces, setPieces] = useState<BoardPieces>(Array(16).fill(null));
  const [available, setAvailable] = useState<boolean[]>(Array(16).fill(true));
  const [yourHand, setYourHand] = useState<number | null>(null);
  const [opponentHand, setOpponentHand] = useState<number | null>(null);
  const [phase, setPhase] = useState<Phase | null>(PHASE.YOU_SELECT);
  const [lastMove, setLastMove] = useState<number | null>(null);
  const [picked, setPicked] = useState<number | null>(null);

  const selecting =
    phase === PHASE.YOU_SELECT || phase === PHASE.OPPONENT_SELECTS;
  const placing = phase === PHASE.YOU_PLACE || phase === PHASE.OPPONENT_PLACES;
  const actor = phase && PHASE_SIDE[phase];

  const selectPiece = (piece: number) => {
    const next = [...available];
    next[piece] = false;
    setAvailable(next);
    setPicked(piece);
    if (phase === PHASE.YOU_SELECT) {
      setOpponentHand(piece);
      setPhase(PHASE.OPPONENT_PLACES);
    } else {
      setYourHand(piece);
      setPhase(PHASE.YOU_PLACE);
    }
  };

  const placePiece = (square: number) => {
    const next = [...pieces];
    if (phase === PHASE.OPPONENT_PLACES) {
      next[square] = opponentHand;
      setOpponentHand(null);
    } else {
      next[square] = yourHand;
      setYourHand(null);
    }
    const nextPhase =
      phase === PHASE.OPPONENT_PLACES
        ? PHASE.OPPONENT_SELECTS
        : PHASE.YOU_SELECT;
    setPhase(next.every((p) => p !== null) ? null : nextPhase);
    setPieces(next);
    setLastMove(square);
  };

  return (
    <main className="flex min-h-screen flex-col items-center justify-center gap-4">
      <h1 className="text-4xl font-bold">ESCAPADE</h1>
      <div className="flex gap-6">
        <div className="w-[120px]">
          <Tray
            available={available}
            marks={
              placing && picked !== null ? { [picked]: [TRAY_MARK.PICKED] } : {}
            }
            live={selecting}
            actor={selecting ? actor : null}
            onPieceClick={selectPiece}
          />
        </div>
        <div className="w-[480px]">
          <Board
            pieces={pieces}
            marks={
              lastMove === null ? {} : { [lastMove]: [BOARD_MARK.LAST_MOVE] }
            }
            live={placing}
            actor={placing ? actor : null}
            onSquareClick={placePiece}
          />
        </div>
        <div className="flex w-80 flex-col justify-between">
          <HandZone
            side={SIDE.OPPONENT}
            label="Opponent"
            piece={opponentHand}
            active={phase === PHASE.OPPONENT_PLACES}
          />
          <div className="rounded bg-white p-3 shadow-sm">
            <PhaseIndicator phase={phase} />
          </div>
          <HandZone
            side={SIDE.YOU}
            label="Player"
            piece={yourHand}
            active={phase === PHASE.YOU_PLACE}
          />
        </div>
      </div>
    </main>
  );
}
