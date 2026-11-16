"use client";

import { useEffect, useState } from "react";
import { FOCUS, PHASE, SIDE, type Focus, type Phase } from "../constants";
import { openingPhase, positionAfter, type Position } from "../game/position";
import type { GameState, WinLine } from "../game/useGame";

/**
 * Shows the game one beat at a time, so each change is visible on its own.
 * The game has every move the moment it's made; pacing keeps its own count
 * of how many moves it has applied, the phase on display, and where attention
 * is, and moves them forward on timers.
 */

/** Milliseconds before each beat. */
const TIMING = {
  /** Between the handover and the phase change, and the phase change and focus. */
  lead: 600,
  /** A hand in focus before attention moves to the board. */
  handFocus: 700,
  /** Board or tray in focus before the opponent's move shows. */
  beforeAction: 900,
  /** The result stays on screen before the highlight goes off. */
  afterAction: 600,
};

type Pacing = {
  movesApplied: number;
  phase: Phase | null;
  focus: Focus | null;
  ended: boolean;
};

const INITIAL: Pacing = {
  movesApplied: 0,
  phase: null,
  focus: null,
  ended: false,
};

/** One step of what's on screen: after `after` milliseconds, pacing becomes `next`. */
type Beat = { after: number; next: Pacing };

const focusOn = (p: Pacing, focus: Focus | null): Pacing => ({ ...p, focus });

const toPhase = (p: Pacing, phase: Phase): Pacing => ({
  ...p,
  phase,
  focus: null,
});

const applyNext = (p: Pacing): Pacing => ({
  ...p,
  movesApplied: p.movesApplied + 1,
});

/** After a placement: the end, if it was a finished game's last move; otherwise the next phase. */
const endOr = (p: Pacing, game: GameState, phase: Phase): Pacing =>
  game.over && p.movesApplied === game.history.length
    ? { ...p, focus: null, ended: true }
    : toPhase(p, phase);

/** The next beat from here, if any. */
function nextBeat(pacing: Pacing, game: GameState): Beat | null {
  if (pacing.ended) {
    return game.over ? null : { after: 0, next: INITIAL };
  }
  const { lead, handFocus, beforeAction, afterAction } = TIMING;
  // The next move to apply, once the game has it.
  const nextMove = game.history[pacing.movesApplied];
  const onScreen = positionAfter(game.history.slice(0, pacing.movesApplied));

  switch (pacing.phase) {
    case null:
      return game.first === null
        ? null // waits for the player to pick who goes first
        : { after: 0, next: toPhase(pacing, openingPhase(game.first)) };

    case PHASE.YOU_SELECT:
      switch (pacing.focus) {
        case null:
          return { after: lead, next: focusOn(pacing, FOCUS.TRAY) };
        case FOCUS.TRAY:
          // Waits for the player's click.
          return nextMove
            ? { after: 0, next: focusOn(applyNext(pacing), FOCUS.HANDOVER) }
            : null;
        case FOCUS.HANDOVER:
          return { after: lead, next: toPhase(pacing, PHASE.OPPONENT_PLACES) };
      }
      return null;

    case PHASE.OPPONENT_PLACES:
      switch (pacing.focus) {
        case null:
          return { after: lead, next: focusOn(pacing, FOCUS.HAND) };
        case FOCUS.HAND:
          return { after: handFocus, next: focusOn(pacing, FOCUS.BOARD) };
        case FOCUS.BOARD:
          if (onScreen.hands[SIDE.OPPONENT] !== null) {
            // Waits here until the engine has replied.
            return nextMove
              ? { after: beforeAction, next: applyNext(pacing) }
              : null;
          }
          return { after: afterAction, next: focusOn(pacing, FOCUS.SETTLE) };
        case FOCUS.SETTLE:
          return {
            after: lead,
            next: endOr(pacing, game, PHASE.OPPONENT_SELECTS),
          };
      }
      return null;

    case PHASE.OPPONENT_SELECTS:
      switch (pacing.focus) {
        case null:
          return { after: lead, next: focusOn(pacing, FOCUS.TRAY) };
        case FOCUS.TRAY:
          if (onScreen.hands[SIDE.YOU] === null) {
            // Waits here until the engine has replied (only at the opening;
            // otherwise the selection came with the placement).
            return nextMove
              ? { after: beforeAction, next: applyNext(pacing) }
              : null;
          }
          return { after: afterAction, next: focusOn(pacing, FOCUS.SETTLE) };
        case FOCUS.SETTLE:
          return { after: lead, next: toPhase(pacing, PHASE.YOU_PLACE) };
      }
      return null;

    case PHASE.YOU_PLACE:
      switch (pacing.focus) {
        case null:
          return { after: lead, next: focusOn(pacing, FOCUS.HAND) };
        case FOCUS.HAND:
          return { after: handFocus, next: focusOn(pacing, FOCUS.BOARD) };
        case FOCUS.BOARD:
          // Waits for the player's click.
          return nextMove
            ? {
                after: 0,
                next: endOr(applyNext(pacing), game, PHASE.YOU_SELECT),
              }
            : null;
      }
      return null;
  }
}

/** What's on screen */
export type View = Position & {
  phase: Phase | null;
  focus: Focus | null;
  ended: boolean;
  winLines: WinLine[];
};

export function usePacing(game: GameState): View {
  const [pacing, setPacing] = useState<Pacing>(INITIAL);

  useEffect(() => {
    const beat = nextBeat(pacing, game);
    if (!beat) return;
    const timer = setTimeout(() => setPacing(beat.next), beat.after);
    return () => clearTimeout(timer);
  }, [pacing, game]);

  const caughtUp = pacing.movesApplied === game.history.length;
  return {
    ...positionAfter(game.history.slice(0, pacing.movesApplied)),
    phase: pacing.phase,
    focus: pacing.focus,
    ended: pacing.ended,
    winLines: game.over && caughtUp ? game.winLines : [],
  };
}
