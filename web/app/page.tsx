"use client";

import { useState } from "react";
import Board from "./components/board/Board";
import type { BoardPieces } from "./components/board/types";
import HandZone from "./components/hand/HandZone";
import GameLayout from "./components/layout/GameLayout";
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
    <GameLayout
      tray={
        <Tray
          available={available}
          marks={
            placing && picked !== null ? { [picked]: [TRAY_MARK.PICKED] } : {}
          }
          live={selecting}
          actor={selecting ? actor : null}
          onPieceClick={selectPiece}
        />
      }
      board={
        <Board
          pieces={pieces}
          marks={
            lastMove === null ? {} : { [lastMove]: [BOARD_MARK.LAST_MOVE] }
          }
          live={placing}
          actor={placing ? actor : null}
          onSquareClick={placePiece}
        />
      }
      opponent={
        <HandZone
          side={SIDE.OPPONENT}
          label="Opponent"
          piece={opponentHand}
          active={phase === PHASE.OPPONENT_PLACES}
        />
      }
      phase={<PhaseIndicator phase={phase} />}
      you={
        <HandZone
          side={SIDE.YOU}
          label="Player"
          piece={yourHand}
          active={phase === PHASE.YOU_PLACE}
        />
      }
    />
  );
}
