"use client";

import { useState, type ReactNode } from "react";
import About from "./components/about/About";
import Board from "./components/board/Board";
import type { BoardMarks } from "./components/board/types";
import GameOverCard from "./components/coach/GameOverCard";
import WelcomeCard from "./components/coach/WelcomeCard";
import Feedback from "./components/feedback/Feedback";
import HandZone from "./components/hand/HandZone";
import GameLayout from "./components/layout/GameLayout";
import PhaseIndicator from "./components/phase/PhaseIndicator";
import StrengthSlider from "./components/strength/StrengthSlider";
import Tray from "./components/tray/Tray";
import {
  BOARD_MARK,
  FOCUS,
  PHASE_SIDE,
  SIDE,
  TRAY_MARK,
  type Focus,
  type Side,
} from "./constants";
import { useGame } from "./game/useGame";
import { usePacing } from "./pacing/usePacing";

export default function PlayScreen() {
  const [strength, setStrength] = useState(85);
  const game = useGame({ strength });
  const view = usePacing(game.state);
  const { phase, focus } = view;

  const boardMarks: BoardMarks = {};
  if (view.lastMove !== null) {
    boardMarks[view.lastMove] = [BOARD_MARK.LAST_MOVE];
  }
  for (const sq of new Set(view.winLines.flatMap((line) => line.squares))) {
    boardMarks[sq] = [...(boardMarks[sq] ?? []), BOARD_MARK.WIN_LINE];
  }

  let coach: ReactNode = null;
  if (game.state.first === null) {
    coach = <WelcomeCard onStart={game.start} />;
  } else if (view.ended) {
    coach = (
      <GameOverCard
        winner={game.state.winner}
        lines={view.winLines}
        onPlayAgain={game.start}
      />
    );
  }

  const picked = view.hands[SIDE.OPPONENT];

  // Whoever's turn it is to act on the part of the screen that has focus.
  const actorOn = (part: Focus): Side | null => {
    if (phase === null || focus !== part) return null;
    return PHASE_SIDE[phase];
  };

  return (
    <GameLayout
      headerActions={
        <>
          <About />
          <Feedback />
        </>
      }
      tray={
        <Tray
          available={view.available}
          marks={picked === null ? {} : { [picked]: [TRAY_MARK.PICKED] }}
          live={actorOn(FOCUS.TRAY) === SIDE.YOU}
          actor={actorOn(FOCUS.TRAY)}
          onPieceClick={game.selectPiece}
        />
      }
      board={
        <Board
          pieces={view.board}
          marks={boardMarks}
          live={actorOn(FOCUS.BOARD) === SIDE.YOU}
          actor={actorOn(FOCUS.BOARD)}
          onSquareClick={game.placePiece}
        />
      }
      coach={coach}
      opponent={
        <div className="flex flex-col gap-3">
          <HandZone
            side={SIDE.OPPONENT}
            label="Opponent"
            piece={view.hands[SIDE.OPPONENT]}
            active={actorOn(FOCUS.HAND) === SIDE.OPPONENT}
          />
          <StrengthSlider value={strength} onChange={setStrength} />
        </div>
      }
      phase={
        <div className="flex flex-col gap-3">
          <PhaseIndicator phase={phase} />
          {game.state.error && (
            <div className="flex items-center justify-between gap-2 text-sm text-red-700">
              <span>{game.state.error}</span>
              <button
                type="button"
                className="rounded bg-red-700 px-2 py-1 font-semibold text-white"
                onClick={game.retry}
              >
                Retry
              </button>
            </div>
          )}
        </div>
      }
      you={
        <HandZone
          side={SIDE.YOU}
          label="Player"
          piece={view.hands[SIDE.YOU]}
          active={actorOn(FOCUS.HAND) === SIDE.YOU}
        />
      }
    />
  );
}
