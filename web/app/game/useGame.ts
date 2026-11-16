"use client";

import { useState } from "react";
import { PHASE, SIDE, type Phase, type Side } from "../constants";
import type { EngineResponse, PlayerMove, Quarto } from "../types/api";
import type { AnalysisResult } from "../types/engine";
import { nextPhase, openingPhase, positionAfter, type Move } from "./position";

export type GameState = {
  first: Side | null;
  history: Move[];
  legalMoves: PlayerMove[];
  /** The last move sent to the engine, kept so a failed request can be retried. */
  sentMove: string | null;
  error: string | null;
  over: boolean;
  winner: Side | null;
  winLines: WinLine[];
};

export type WinLine = { squares: number[]; attributes: number[] };

function toWinLines(quartos: Quarto[]): WinLine[] {
  const lines = new Map<string, WinLine>();
  for (const { intersection, attribute } of quartos) {
    const key = intersection.join();
    const line = lines.get(key) ?? {
      squares: [...intersection],
      attributes: [],
    };
    line.attributes.push(attribute);
    lines.set(key, line);
  }
  return [...lines.values()];
}

/** The player's opening moves: select any piece (no placement yet). */
const FIRST_MOVES: PlayerMove[] = Array.from({ length: 16 }, (_, piece) => ({
  piece,
  hex: `00000000000000000000${piece.toString(16).toUpperCase()}0`,
  description: `You selected piece ${piece}.`,
}));

/** Asks the engine for the opening selection (an empty board). */
const OPENING_REQUEST = " ".repeat(17);

const INITIAL: GameState = {
  first: null,
  history: [],
  legalMoves: FIRST_MOVES,
  sentMove: null,
  error: null,
  over: false,
  winner: null,
  winLines: [],
};

/** What happens next, or null before the game starts and once it's over. */
const currentPhase = (s: GameState): Phase | null => {
  if (s.first === null || s.over) return null;
  const last = s.history.at(-1);
  return last ? nextPhase(last) : openingPhase(s.first);
};

/** The engine's placement, then its selection; either can be missing. */
function movesFromReply(s: GameState, reply: EngineResponse): Move[] {
  const position = positionAfter(s.history);
  const moves: Move[] = [];
  const piece = position.hands[SIDE.OPPONENT];
  if (piece !== null) {
    const square = reply.board.findIndex(
      (p, sq) => p != null && position.board[sq] === null,
    );
    moves.push({ kind: "place", by: SIDE.OPPONENT, piece, square });
  }
  if (reply.event === "normal") {
    moves.push({
      kind: "select",
      by: SIDE.OPPONENT,
      piece: reply.pieceToPlace,
    });
  }
  return moves;
}

/** The game with the engine's reply added. */
function gameAfterReply(s: GameState, reply: EngineResponse): GameState {
  const history = [...s.history, ...movesFromReply(s, reply)];
  switch (reply.event) {
    case "engine_win":
      return {
        ...s,
        history,
        over: true,
        winner: SIDE.OPPONENT,
        winLines: toWinLines(reply.quartos),
      };
    case "draw":
      return { ...s, history, over: true };
    case "normal":
      return { ...s, history, legalMoves: reply.moves };
  }
}

const LOG_ENGINE = process.env.NODE_ENV === "development";

/** Each engine reply and its evaluation. */
function logReply(move: string, strength: number, reply: EngineResponse) {
  console.groupCollapsed(`[engine] ${reply.event}: ${reply.description}`);
  console.log("sent move", move, "strength", strength);
  console.log("evaluation", reply.analysis);
  console.log("reply", reply);
  console.groupEnd();
}

/** The player's options when their turn starts, as a table. */
function logOptions(moves: PlayerMove[], evaluation: AnalysisResult | null) {
  const outcomeOf = new Map<string, number>();
  for (const orbit of evaluation?.orbits ?? []) {
    for (const m of orbit.moves) outcomeOf.set(m.hex, orbit.outcome);
  }
  const best = evaluation?.orbits?.[0]?.outcome;
  console.groupCollapsed(`[engine] your options (${moves.length})`);
  console.table(
    moves.map((m) => ({
      square: m.square,
      piece: m.piece,
      outcome: outcomeOf.get(m.hex),
      best: best !== undefined && outcomeOf.get(m.hex) === best,
      quarto: m.quarto?.map((q) => q.attribute).join(", ") ?? "",
      description: m.description,
    })),
  );
  console.groupEnd();
}

async function requestReply(
  move: string,
  strength: number,
): Promise<EngineResponse> {
  const res = await fetch(
    `/api/play?ply=${encodeURIComponent(move)}&strength=${strength}`,
  );
  if (!res.ok) throw new Error(`Engine request failed (${res.status})`);
  return res.json();
}

export function useGame({ strength }: { strength: number }) {
  const [state, setState] = useState<GameState>(INITIAL);

  const playEngineTurn = (move: string) => {
    requestReply(move, strength).then(
      (reply) => {
        if (LOG_ENGINE) {
          logReply(move, strength, reply);
          if (reply.event === "normal") logOptions(reply.moves, reply.analysis);
        }
        setState((s) => gameAfterReply(s, reply));
      },
      (err: Error) => setState((s) => ({ ...s, error: err.message })),
    );
  };

  const start = (first: Side) => {
    if (first === SIDE.YOU) {
      if (LOG_ENGINE) logOptions(FIRST_MOVES, null);
      setState({ ...INITIAL, first });
      return;
    }
    setState({
      ...INITIAL,
      first,
      legalMoves: [],
      sentMove: OPENING_REQUEST,
    });
    playEngineTurn(OPENING_REQUEST);
  };

  const selectPiece = (piece: number) => {
    const s = state;
    if (currentPhase(s) !== PHASE.YOU_SELECT) return;
    if (!positionAfter(s.history).available[piece]) return;
    const last = s.history.at(-1);
    const square = last?.kind === "place" ? last.square : undefined;
    const move = s.legalMoves.find(
      (m) => m.piece === piece && m.square === square,
    );
    if (!move) return;

    setState({
      ...s,
      history: [...s.history, { kind: "select", by: SIDE.YOU, piece }],
      sentMove: move.hex,
      error: null,
    });
    playEngineTurn(move.hex);
  };

  const placePiece = (square: number) => {
    setState((s) => {
      if (currentPhase(s) !== PHASE.YOU_PLACE) return s;
      const position = positionAfter(s.history);
      const piece = position.hands[SIDE.YOU];
      if (piece === null || position.board[square] !== null) return s;
      const options = s.legalMoves.filter((m) => m.square === square);
      if (options.length === 0) return s;

      const history: Move[] = [
        ...s.history,
        { kind: "place", by: SIDE.YOU, piece, square },
      ];
      const winning = options.find((m) => m.quarto);
      if (winning) {
        return {
          ...s,
          history,
          over: true,
          winner: SIDE.YOU,
          winLines: toWinLines(winning.quarto!),
        };
      }
      if (options.every((m) => m.piece === undefined)) {
        return { ...s, history, over: true };
      }
      return { ...s, history };
    });
  };

  const retry = () => {
    if (!state.sentMove) return;
    setState((s) => ({ ...s, error: null }));
    playEngineTurn(state.sentMove);
  };

  return { state, start, selectPiece, placePiece, retry };
}
