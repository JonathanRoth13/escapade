"use client";

import { useState } from "react";
import Board from "./components/board/Board";
import type { BoardPieces } from "./components/board/types";
import HandZone from "./components/hand/HandZone";
import Tray from "./components/tray/Tray";

// Placeholder until the play screen lands: pass-and-play, where you click
// for both sides through the four steps of a turn.
type Step = "you-select" | "opponent-places" | "opponent-selects" | "you-place";

export default function Home() {
  const [pieces, setPieces] = useState<BoardPieces>(Array(16).fill(null));
  const [available, setAvailable] = useState<boolean[]>(Array(16).fill(true));
  const [yourHand, setYourHand] = useState<number | null>(null);
  const [opponentHand, setOpponentHand] = useState<number | null>(null);
  const [step, setStep] = useState<Step>("you-select");
  const [lastMove, setLastMove] = useState<number | null>(null);
  const [picked, setPicked] = useState<number | null>(null);

  const selecting = step === "you-select" || step === "opponent-selects";
  const trayEmpty = available.every((a) => !a);

  const selectPiece = (piece: number) => {
    const next = [...available];
    next[piece] = false;
    setAvailable(next);
    setPicked(piece);
    if (step === "you-select") {
      setOpponentHand(piece);
      setStep("opponent-places");
    } else {
      setYourHand(piece);
      setStep("you-place");
    }
  };

  const placePiece = (square: number) => {
    const next = [...pieces];
    if (step === "opponent-places") {
      next[square] = opponentHand;
      setOpponentHand(null);
      setStep("opponent-selects");
    } else {
      next[square] = yourHand;
      setYourHand(null);
      setStep("you-select");
    }
    setPieces(next);
    setLastMove(square);
  };

  const actor =
    step === "you-select" || step === "you-place" ? "you" : "opponent";

  return (
    <main className="flex min-h-screen flex-col items-center justify-center gap-4">
      <h1 className="text-4xl font-bold">ESCAPADE</h1>
      <div className="flex gap-6">
        <div className="w-[120px]">
          <Tray
            available={available}
            marks={
              !selecting && picked !== null ? { [picked]: ["picked"] } : {}
            }
            live={selecting && !trayEmpty}
            actor={selecting && !trayEmpty ? actor : null}
            onPieceClick={selectPiece}
          />
        </div>
        <div className="w-[480px]">
          <Board
            pieces={pieces}
            marks={lastMove === null ? {} : { [lastMove]: ["last-move"] }}
            live={!selecting}
            actor={selecting ? null : actor}
            onSquareClick={placePiece}
          />
        </div>
        <div className="flex flex-col justify-between">
          <HandZone
            side="opponent"
            label="Opponent"
            piece={opponentHand}
            active={step === "opponent-places"}
          />
          <HandZone
            side="you"
            label="Player"
            piece={yourHand}
            active={step === "you-place"}
          />
        </div>
      </div>
    </main>
  );
}
