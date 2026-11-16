"use client";

import { useCallback, useState, type ReactNode } from "react";
import {
  FOCUS,
  PHASE,
  PHASE_LABEL,
  type Focus,
  type Phase,
} from "../constants";

type Step = {
  id: string;
  at: (phase: Phase | null, focus: Focus | null) => boolean;
  title: string;
  body: ReactNode;
  slider?: boolean;
  last?: boolean;
};

const moment =
  (p: Phase, f: Focus | null) => (phase: Phase | null, focus: Focus | null) =>
    phase === p && focus === f;

const STEPS: Step[] = [
  {
    id: "strength",
    at: (phase, focus) =>
      focus === null &&
      (phase === PHASE.YOU_SELECT || phase === PHASE.OPPONENT_SELECTS),
    title: "Difficulty",
    body: (
      <p>
        This slider sets how well your opponent plays. You can change it at any
        time, even in the middle of a game.
      </p>
    ),
    slider: true,
  },
  {
    id: "you-select",
    at: moment(PHASE.YOU_SELECT, null),
    title: PHASE_LABEL[PHASE.YOU_SELECT],
    body: (
      <p>
        Choose a piece from the tray for your opponent to place. The tray is
        outlined in blue when it&apos;s your turn.
      </p>
    ),
  },
  {
    id: "opponent-places",
    at: moment(PHASE.OPPONENT_PLACES, null),
    title: PHASE_LABEL[PHASE.OPPONENT_PLACES],
    body: (
      <>
        <p>
          The piece you picked is now in your opponent&apos;s hand, at the top
          right, and they have to place it on the board.
        </p>
        <p>Purple means it&apos;s your opponent acting.</p>
      </>
    ),
  },
  {
    id: "opponent-placed",
    at: moment(PHASE.OPPONENT_PLACES, FOCUS.SETTLE),
    title: PHASE_LABEL[PHASE.OPPONENT_PLACES],
    body: (
      <p>
        The square it went to keeps a light tint, so you can always see your
        opponent&apos;s last move.
      </p>
    ),
  },
  {
    id: "opponent-selects",
    at: moment(PHASE.OPPONENT_SELECTS, null),
    title: PHASE_LABEL[PHASE.OPPONENT_SELECTS],
    body: (
      <p>
        Your opponent chooses a piece from the tray for you to place. It will
        appear in your hand at the bottom right.
      </p>
    ),
  },
  {
    id: "you-place",
    at: moment(PHASE.YOU_PLACE, FOCUS.HAND),
    title: PHASE_LABEL[PHASE.YOU_PLACE],
    body: (
      <>
        <p>
          This is the piece your opponent picked for you. Click any empty square
          on the board to place it.
        </p>
      </>
    ),
  },
  {
    id: "full-round",
    at: (phase, focus) => phase !== null && focus === null,
    last: true,
    title: "That's a full round",
    body: <p>From here the same four steps repeat. Good luck!</p>,
  },
];

export function useTutorial() {
  const [on, setOn] = useState(true);
  const [done, setDone] = useState<string[]>([]);

  const stepAt = useCallback(
    (phase: Phase | null, focus: Focus | null): Step | null => {
      if (!on) return null;
      const ready = (s: Step) =>
        !done.includes(s.id) &&
        (!s.last || STEPS.every((o) => o === s || done.includes(o.id)));
      return STEPS.find((s) => ready(s) && s.at(phase, focus)) ?? null;
    },
    [on, done],
  );

  const holds = useCallback(
    (phase: Phase | null, focus: Focus | null) => stepAt(phase, focus) !== null,
    [stepAt],
  );

  return {
    stepAt,
    holds,
    atStart: done.length === 0,
    finish: (id: string) => setDone((d) => [...d, id]),
    stop: () => setOn(false),
  };
}
